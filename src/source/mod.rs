pub use point::Point;
pub use range::Range;

mod point;
mod range;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum SourceLocation {
    Point(Point),
    Range(Range),
}

pub trait ToLocation {
    fn to_location(self) -> SourceLocation;
}
