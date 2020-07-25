//! read device tree
//! 
//! traverse and initialize device tree

use crate::memory::VirtualAddress;
use core::slice;
use device_tree::{DeviceTree, Node};
use super::bus::virtio_mmio::virtio_probe;
/// 验证某内存段为设备树格式的 Magic Number（固定）
const DEVICE_TREE_MAGIC: u32 = 0xd00d_feed;

/// recursive traverse device tree
fn walk(node: &Node) {
    // check and initialize
    if let Ok(compatible) = node.prop_str("compatible") {
        if compatible == "virtio,mmio" {
            virtio_probe(node);
        }
    }
    // 遍历子树
    for child in node.children.iter() {
        walk(child);
    }
}

/// Headers of Device Tree
struct DtbHeader {
    magic: u32,
    size: u32,
}

/// traverse device tree and initialize device
pub fn init(dtb_va: VirtualAddress) {
    let header = unsafe { &*(dtb_va.0 as *const DtbHeader) };
    // from_be 是大小端序的转换（from big endian）
    let magic = u32::from_be(header.magic);
    if magic == DEVICE_TREE_MAGIC {
        let size = u32::from_be(header.size);
        // 拷贝数据，加载并遍历
        let data = unsafe { slice::from_raw_parts(dtb_va.0 as *const u8, size as usize) };
        if let Ok(dt) = DeviceTree::load(data) {
            walk(&dt.root);
        }
    }
}
