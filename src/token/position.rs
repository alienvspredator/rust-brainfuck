use std::fmt::{Display, Formatter, Write};
use std::ops::{Add, Not};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Position {
    pub offset: usize,
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn is_valid(&self) -> bool {
        self.line > 0
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_valid() {
            if self.column != 0 {
                write!(f, "{}:{}", self.line, self.column)
            } else {
                write!(f, "{}", self.line)
            }
        } else {
            write!(f, "-")
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Pos(pub usize);

pub const NO_POS: Pos = Pos(0);

impl Pos {
    pub fn is_valid(self) -> bool {
        self != NO_POS
    }
}

impl<T: Into<usize>> Add<T> for Pos {
    type Output = Pos;

    fn add(self, rhs: T) -> Self::Output {
        Pos(self.0 + rhs.into())
    }
}

impl Into<usize> for Pos {
    fn into(self) -> usize {
        self.0
    }
}

impl From<usize> for Pos {
    fn from(pos: usize) -> Self {
        Pos(pos)
    }
}
