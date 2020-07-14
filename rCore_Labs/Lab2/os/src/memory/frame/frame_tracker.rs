//! 「`Box`」 [`FrameTracker`] to provide physical frame
#![allow(unused)]
use crate::memory::{address::*, FRAME_ALLOCATOR, PAGE_SIZE};

pub struct FrameTracker(pub(super) PhysicalPageNumber);

impl FrameTracker {
    /// PhysicalAddress of frame
    pub fn address(&self) -> PhysicalAddress {
        self.0.into()
    }

    /// PageNumber of frame
    pub fn page_number(&self) -> PhysicalPageNumber {
        self.0
    }
}

impl Drop for FrameTracker {
    fn drop(&mut self) {
        FRAME_ALLOCATOR.lock().dealloc(self);
    }
}