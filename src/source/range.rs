use crate::source::point::Point;
use crate::source::{SourceLocation, ToLocation};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Range {
    pub from: Point,
    pub to: Point,
}

impl Range {
    pub(crate) fn new(from: Point, to: Point) -> Range {
        Range { from, to }
    }
}

impl ToLocation for Range {
    fn to_location(self) -> SourceLocation {
        SourceLocation::Range(self)
    }
}
