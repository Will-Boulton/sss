#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum SourceLocation {
    Point(SourcePoint),
    Range(SourceRange),
}

pub trait ToLocation {
    fn to_location(self) -> SourceLocation;
}

#[derive(PartialEq, Clone, Copy, Debug, Eq)]
pub struct SourcePoint {
    pub line: usize,
    pub col: usize,
}

impl ToLocation for SourcePoint {
    fn to_location(self) -> SourceLocation {
        SourceLocation::Point(self)
    }
}

impl ToLocation for SourceRange {
    fn to_location(self) -> SourceLocation {
        SourceLocation::Range(self)
    }
}

impl SourcePoint {
    pub(crate) fn zero() -> SourcePoint {
        SourcePoint { line: 0, col: 0 }
    }

    pub fn new(line: usize, col: usize) -> SourcePoint {
        SourcePoint { line, col }
    }

    pub(crate) fn next_pos(&self) -> SourcePoint {
        SourcePoint {
            line: self.line,
            col: self.col + 1,
        }
    }

    pub(crate) fn next_line(&self) -> SourcePoint {
        SourcePoint {
            line: self.line + 1,
            col: 0,
        }
    }

    pub(crate) fn range_to(&self, to: SourcePoint) -> SourceRange {
        SourceRange {
            from: self.clone(),
            to: to.clone(),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct SourceRange {
    pub from: SourcePoint,
    pub to: SourcePoint,
}

impl SourceRange {
    fn new(from: SourcePoint, to: SourcePoint) -> SourceRange {
        SourceRange { from, to }
    }
}

#[macro_export]
macro_rules! location {
    ($l:literal, $c:literal) => {
        SourceLocation::Point(SourcePoint::new($l, $c))
    };
    ($l:expr => $l2:expr) => {
        SourceLocation::Range($l.range_to($l2))
    };
    ($l:literal, $c:literal -> $l2:literal, $c2:literal) => {
        SourceLocation::Range(SourceRange::new(
            SourcePoint::new($l, $c),
            SourcePoint::new($l2, $c2),
        ))
    };
}
