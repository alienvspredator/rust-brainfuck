use crate::token::Position;
use std::cmp::Ordering;
use std::error::Error as StdError;
use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Error {
    pub pos: Position,
    pub msg: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.pos.is_valid() {
            write!(f, "{}: {}", self.pos, self.msg)
        } else {
            write!(f, "{}", self.msg)
        }
    }
}

impl StdError for Error {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ErrorList(Vec<Error>);

impl ErrorList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add(&mut self, pos: Position, msg: impl Into<String>) {
        self.0.push(Error {
            pos,
            msg: msg.into(),
        });
    }

    pub fn reset(&mut self) {
        self.0.clear();
    }

    pub fn sort(&mut self) {
        self.0.sort_by(|a, b| match a.pos.line.cmp(&b.pos.line) {
            Ordering::Equal => match a.pos.column.cmp(&b.pos.column) {
                Ordering::Equal => a.msg.cmp(&b.msg),
                other => other,
            },
            other => other,
        });
    }

    pub fn remove_multiples(&mut self) {
        self.sort();
        let mut unique = Vec::new();
        let mut last_line = None;
        for err in &self.0 {
            if Some(err.pos.line) != last_line {
                unique.push(err.clone());
                last_line = Some(err.pos.line);
            }
        }
        self.0 = unique;
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn err(&self) -> Option<&Self> {
        if self.is_empty() { None } else { Some(self) }
    }
}

impl fmt::Display for ErrorList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.len() {
            0 => write!(f, "no errors"),
            1 => write!(f, "{}", self.0[0]),
            n => write!(f, "{} (and {} more errors)", self.0[0], n - 1),
        }
    }
}

impl StdError for ErrorList {}
