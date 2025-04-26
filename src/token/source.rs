use crate::token::{Pos, Position};
use std::sync::Mutex;

pub struct Source {
    size: usize,
    lines: Mutex<Vec<usize>>,
}

impl Source {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            lines: Mutex::new(vec![0]),
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn line_count(&self) -> usize {
        self.lines.lock().unwrap().len()
    }

    pub fn add_line(&self, offset: usize) {
        let mut lines = self.lines.lock().unwrap();
        let i = lines.len();
        if (i == 0 || lines[i - 1] < offset) && offset < self.size {
            lines.push(offset);
        }
    }

    pub fn merge_lines(&self, line: usize) {
        if line < 1 {
            panic!("invalid line number {} (should be >= 1)", line);
        }

        let mut lines = self.lines.lock().unwrap();
        if line >= lines.len() {
            panic!("invalid line number {} (should be < {})", line, lines.len());
        }

        lines.remove(line);
    }

    pub fn set_lines(&self, lines: Vec<usize>) -> bool {
        // Verify the validity of the line table
        for i in 1..lines.len() {
            if lines[i] <= lines[i - 1] || self.size <= lines[i] {
                return false;
            }
        }

        let mut guard = self.lines.lock().unwrap();
        *guard = lines;
        true
    }

    pub fn set_lines_for_content(&self, content: &[u8]) {
        let mut lines: Vec<usize> = vec![];
        let mut line: Option<usize> = None;
        for (offset, &b) in content.iter().enumerate() {
            if let Some(line) = line {
                lines.push(line);
            }

            line = None;
            if b == b'\n' {
                line = Some(offset + 1);
            }
        }

        let mut guard = self.lines.lock().unwrap();
        *guard = lines;
    }

    pub fn line_start(&self, line: usize) -> Pos {
        if line < 1 {
            panic!("invalid line number {} (should be >= 1)", line);
        }

        let lines = self.lines.lock().unwrap();
        if line > lines.len() {
            panic!("invalid line number {} (should be < {})", line, lines.len());
        }

        lines[line - 1].into()
    }

    pub fn line(&self, p: Pos) -> usize {
        self.position(p).line
    }

    pub fn unpack(&self, offset: usize) -> (usize, usize) {
        let lines = self.lines.lock().unwrap();
        lines
            .iter()
            .position(|&i| i == offset)
            .map_or((0, 0), |i| (i + 1, offset - lines[i]))
    }

    pub fn position(&self, p: Pos) -> Position {
        if !p.is_valid() {
            return Position::default();
        }

        let (line, column) = self.unpack(p.into());

        Position {
            offset: p.into(),
            line,
            column,
        }
    }
}
