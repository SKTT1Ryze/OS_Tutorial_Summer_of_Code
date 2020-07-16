use crate::memory::address::*;
/// page table entry for Sv39
use bit_field::BitField;
use bitflags::*;

bitflags! {
    /// 8 flags in page table entry
    #[derive(Default)]
    pub struct Flags: u8 {
        /// valid
        const VALID = 1 << 0;
        /// readable
        const READABLE = 1 << 1;
        /// writable
        const WRITABLE = 1 << 2;
        /// executable
        const EXECUTABLE = 1 << 3;
        /// user
        const USER = 1 << 4;
        /// gloabl
        const GLOBAL = 1 << 5;
        /// accessed
        const ACCESSED = 1 << 6;
        /// dirty
        const DIRTY = 1 << 7;
    }
}

#[derive(Copy, Clone, Default)]
pub struct PageTableEntry(usize);

impl PageTableEntry {
    /// write page number and flags into a page table entry
    pub fn new(page_number: PhysicalPageNumber, flags: Flags) -> Self {
        Self(
            *0usize
                .set_bits(..8, flags.bits() as usize)
                .set_bits(10..54, page_number.into()),
        )
    }

    /// get physcial page number, linear mapping
    pub fn page_number(&self) -> PhysicalPageNumber {
        PhysicalPageNumber::from(self.0.get_bits(10..54))
    }

    /// get physcial page address, linear mapping
    pub fn address(&self) -> PhysicalAddress {
        PhysicalAddress::from(self.page_number())
    }

    /// get flags
    pub fn flags(&self) -> Flags {
        unsafe { Flags::from_bits_unchecked(self.0.get_bits(..8) as u8) }
    }

    /// is empty or not
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// clear
    pub fn clear(&mut self) {
        self.0 = 0;
    }

    /// check RWX is 000 or not
    pub fn has_next_level(&self) -> bool {
        let flags = self.flags();
        !(flags.contains(Flags::READABLE)
            || flags.contains(Flags::WRITABLE)
            || flags.contains(Flags::EXECUTABLE))
    }
}

impl core::fmt::Debug for PageTableEntry {
    fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter
            .debug_struct("PageTableEntry")
            .field("value", &self.0)
            .field("page_number", &self.page_number())
            .field("flags", &self.flags())
            .finish()
    }
}

macro_rules! implement_flags {
    ($field: ident, $name: ident, $quote: literal) => {
        impl Flags {
            #[doc = "return `Flags::"]
            #[doc = $quote]
            #[doc = "` or `Flags::empty()`"]
            pub fn $name(value: bool) -> Flags {
                if value {
                    Flags::$field
                } else {
                    Flags::empty()
                }
            }
        }
    };
}

implement_flags! {USER, user, "USER"}
implement_flags! {READABLE, readable, "READABLE"}
implement_flags! {WRITABLE, writable, "WRITABLE"}
implement_flags! {EXECUTABLE, executable, "EXECUTABLE"}
