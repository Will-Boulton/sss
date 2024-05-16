#[derive(PartialEq, Clone, Copy, Debug, Eq)]
pub struct SourceLocation {
    pub line: usize,
    pub col: usize,
}

impl SourceLocation {
    pub(crate) fn zero() -> SourceLocation {
        SourceLocation { line: 0, col: 0 }
    }

    pub fn new(line: usize, col: usize) -> SourceLocation {
        SourceLocation { line, col }
    }

    pub(crate) fn next_pos(&self) -> SourceLocation {
        SourceLocation {
            line: self.line,
            col: self.col + 1,
        }
    }

    pub(crate) fn next_line(&self) -> SourceLocation {
        SourceLocation {
            line: self.line + 1,
            col: 0,
        }
    }

    pub(crate) fn range_to(&self, to: SourceLocation) -> SourceRange {
        SourceRange {
            from: self.clone(),
            to: to.clone(),
        }
    }
}

#[derive(PartialEq,Eq, Clone, Copy, Debug)]
pub struct SourceRange {
    pub from: SourceLocation,
    pub to: SourceLocation,
}

impl SourceRange {
    pub fn new(from: [usize; 2], to: [usize; 2]) -> Self {
        SourceRange {
            from: SourceLocation::new(from[0], from[1]),
            to: SourceLocation::new(to[0], to[1]),
        }
    }
}