use crate::source::range::Range;
use crate::source::{SourceLocation, ToLocation};

#[derive(PartialEq, Clone, Copy, Debug, Eq)]
pub struct Point {
    line: usize,
    col: usize,
}

impl Point {
    pub fn get_column(&self) -> usize {
        self.col
    }

    pub fn get_line(&self) -> usize {
        self.line
    }

    pub(crate) fn zero() -> Point {
        Point { line: 0, col: 0 }
    }

    pub fn new(line: usize, col: usize) -> Point {
        Point { line, col }
    }

    pub(crate) fn next_pos(&self) -> Point {
        Point {
            line: self.line,
            col: self.col + 1,
        }
    }

    pub(crate) fn next_line(&self) -> Point {
        Point {
            line: self.line + 1,
            col: 0,
        }
    }

    pub(crate) fn range_to(&self, to: Point) -> Range {
        Range {
            from: self.clone(),
            to: to.clone(),
        }
    }
}

impl ToLocation for Point {
    fn to_location(self) -> SourceLocation {
        SourceLocation::Point(self)
    }
}
