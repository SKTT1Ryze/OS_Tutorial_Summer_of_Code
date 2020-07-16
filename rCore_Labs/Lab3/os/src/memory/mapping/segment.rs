//! [`MapType`] and [`Segment`]

use crate::memory::{address::*, mapping::Flags, range::Range};

/// Type of mapping
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MapType {
    /// linear mapping
    Linear,
    /// framed mapping
    Framed,
}

/// A mapping segment
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Segment {
    /// mapping type
    pub map_type: MapType,
    /// range of VirtualAddress
    pub range: Range<VirtualAddress>,
    /// flags
    pub flags: Flags,
}

impl Segment {
    /// traverse PhysicalPageNumber if possiable
    pub fn iter_mapped(&self) -> Option<impl Iterator<Item = PhysicalPageNumber>> {
        match self.map_type {
            // linear mapping
            MapType::Linear => Some(self.page_range().into().iter()),
            // framed mapping, need to alloc frames
            MapType::Framed => None,
        }
    }

    /// get range of VirtualPageNumber
    pub fn page_range(&self) -> Range<VirtualPageNumber> {
        Range::from(
            VirtualPageNumber::floor(self.range.start)..VirtualPageNumber::ceil(self.range.end),
        )
    }
}
