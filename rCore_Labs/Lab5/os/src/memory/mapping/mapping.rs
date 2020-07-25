//! Page Table for Rv39 [`Mapping`]
//! 
//! 许多方法返回 [`Result`]，如果出现错误会返回 `Err(message)`。设计目标是，此时如果终止线程，则不会产生后续问题。
//! 但是如果错误是由操作系统代码逻辑产生的，则会直接 panic。
#![allow(unused)]
use crate::memory::{
    config::PAGE_SIZE,
    address::*,
    MemoryResult,
    frame::{FrameTracker, FRAME_ALLOCATOR},
    mapping::{Flags,MapType,PageTable,PageTableEntry,PageTableTracker,Segment},
};
use core::cmp::min;
use core::ptr::slice_from_raw_parts_mut;
use alloc::{vec, vec::Vec};

#[derive(Default)]
/// mapping relation for a process
pub struct Mapping {
    /// save all page tables
    page_tables: Vec<PageTableTracker>,
    /// physcial page number of root page table
    root_ppn: PhysicalPageNumber,
}

impl Mapping {
    /// save current mapping in `satp` and record
    pub fn activate(&self) {
        // ppn: [..27], mode: high 4 bits, 8 -> Sv39
        let new_satp: usize = self.root_ppn.0 | (8 << 60);
        unsafe {
            // write new_satp in satp
            llvm_asm!("csrw satp, $0" :: "r"(new_satp) :: "volatile");
            // refresh TLB
            llvm_asm!("sfence.vma" :::: "volatile");

        }
    }

    /// create a mapping with root
    pub fn new() -> MemoryResult<Mapping> {
        let root_table = PageTableTracker::new(FRAME_ALLOCATOR.lock().alloc()?);
        let root_ppn = root_table.page_number();
        Ok(Mapping {
            page_tables: vec![root_table],
            root_ppn,
        })
    }

    /// find 3 level page table entry
    /// 
    /// if not found, create one
    pub fn find_entry(&mut self, vpn: VirtualPageNumber) -> MemoryResult<&mut PageTableEntry> {
        // search from root page table
        let root_table: &mut PageTable = PhysicalAddress::from(self.root_ppn).deref_kernel();
        // level 3 page table entry
        let mut entry = &mut root_table.entries[vpn.levels()[0]];
        for vpn_slice in &vpn.levels()[1..] {
            if entry.is_empty() {
                // if page table not exist, alloc one
                let new_table = PageTableTracker::new(FRAME_ALLOCATOR.lock().alloc()?);
                let new_ppn = new_table.page_number();
                // write page number of new table in current page table entry
                *entry = PageTableEntry::new(new_ppn, Flags::VALID);
                // save new page table
                self.page_tables.push(new_table);
            }
            // enter next level page table
            entry = &mut entry.get_next_table().entries[*vpn_slice];
        }
        Ok(entry)
    }

    /// create mapping relation between VirtualPageNumber and PhysicalPageNumber
    fn map_one(
        &mut self,
        vpn: VirtualPageNumber,
        ppn: PhysicalPageNumber,
        flags: Flags,
    ) -> MemoryResult<()> {
        // get page table entry
        let entry = self.find_entry(vpn)?;
        assert!(entry.is_empty(), "virtual mapped");
        // page table entry is empty, write ppn
        *entry = PageTableEntry::new(ppn,flags);
        Ok(())
    }

    pub fn unmap(&mut self, segment: &Segment) {
        for vpn in segment.page_range().iter() {
            let entry = self.find_entry(vpn).unwrap();
            assert!(!entry.is_empty());
            // clear the page table entry
            entry.clear();
        }
    }

    /// find PhyscialAddress mapped with VirtualAddress
    pub fn lookup(va: VirtualAddress) -> Option<PhysicalAddress> {
        let mut current_ppn;
        unsafe {
            llvm_asm!("csrr $0, satp" : "=r"(current_ppn) ::: "volatile");
            current_ppn ^= 8 << 60;
        }
        let root_table: &PageTable = PhysicalAddress::from(PhysicalPageNumber(current_ppn)).deref_kernel();
        let vpn = VirtualPageNumber::floor(va);
        let mut entry = &root_table.entries[vpn.levels()[0]];
        // 为了支持大页的查找，我们用 length 表示查找到的物理页需要加多少位的偏移
        let mut length = 12 + 2 * 9;
        for vpn_slice in &vpn.levels()[1..] {
            if entry.is_empty() {
                return None;
            }
            if entry.has_next_level() {
                length -= 9;
                entry = &mut entry.get_next_table().entries[*vpn_slice];
            }
            else {
                break;
            }  
        }
        let base = PhysicalAddress::from(entry.page_number()).0;
        let offset = va.0 & ((1<<length)-1);
        Some(PhysicalAddress(base+offset))
    }


    /// add a mapping, maybe need to alloc physcial page
    /// 
    /// 未被分配物理页面的虚拟页号暂时不会写入页表当中，它们会在发生 PageFault 后再建立页表项。
    pub fn map(
        &mut self,
        segment: &Segment,
        init_data: Option<&[u8]>,
    ) -> MemoryResult<Vec<(VirtualPageNumber, FrameTracker)>> {
        match segment.map_type {
            // linear mapping
            MapType::Linear => {
                for vpn in segment.page_range().iter() {
                    self.map_one(vpn, vpn.into(), segment.flags | Flags::VALID)?;
                }
                // clone data
                if let Some(data) = init_data {
                    unsafe {
                        (&mut *slice_from_raw_parts_mut(segment.range.start.deref(), data.len()))
                            .copy_from_slice(data);
                    }
                }
                Ok(Vec::new())
            }
            // framed mapping
            MapType::Framed => {
                // 记录所有成功分配的页面映射
                let mut allocated_pairs = Vec::new();
                for vpn in segment.page_range().iter() {
                    // alloc physical page
                    let mut frame = FRAME_ALLOCATOR.lock().alloc()?;
                    // map, write zero, record
                    self.map_one(vpn, frame.page_number(), segment.flags | Flags::VALID)?;
                    frame.fill(0);
                    allocated_pairs.push((vpn,frame));
                }

                // clone data
                if let Some(data) = init_data {
                    if !data.is_empty() {
                        for (vpn, frame) in allocated_pairs.iter_mut() {
                            // 拷贝时必须考虑区间与整页不对齐的情况
                            //    start（仅第一页时非零）
                            //      |        stop（仅最后一页时非零）
                            // 0    |---data---|          4096
                            // |------------page------------|
                            let page_address = VirtualAddress::from(*vpn);
                            let start = if segment.range.start > page_address {
                                segment.range.start - page_address
                            }
                            else {
                                0
                            };
                            let stop = min(PAGE_SIZE, segment.range.end - page_address);
                            // now copy
                            let dst_slice = &mut frame[start..stop];
                            let src_slice = &data[(page_address + start - segment.range.start)
                                ..(page_address + stop - segment.range.start)];
                            dst_slice.copy_from_slice(src_slice);
                        }
                    }
                }
                Ok(allocated_pairs)
            }
        }
    }








}