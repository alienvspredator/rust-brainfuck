use crate::token;
use std::rc::Rc;

const BOM: char = '\u{FEFF}';
const EOF: char = '\u{FFFF}';

pub struct Scanner<'a> {
    source: Rc<token::Source>,
    src: &'a [u8],
    eh: Option<Box<dyn FnMut(token::Position, &str)>>,

    // scanning state
    ch: char,
    offset: usize,
    rd_offset: usize,
    line_offset: usize,

    pub error_count: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(
        source: Rc<token::Source>,
        src: &'a [u8],
        error_handler: Option<Box<dyn FnMut(token::Position, &str)>>,
    ) -> Self {
        if source.size() != src.len() {
            panic!(
                "source size ({}) does not match src len ({})",
                source.size(),
                src.len()
            );
        }

        let mut scanner = Self {
            source,
            src,
            eh: error_handler,
            ch: ' ',
            offset: 0,
            rd_offset: 0,
            line_offset: 0,
            error_count: 0,
        };

        scanner.next();
        if scanner.ch == BOM {
            scanner.next();
        }

        scanner
    }

    fn next(&mut self) {
        if self.rd_offset < self.src.len() {
            self.offset = self.rd_offset;
            if self.ch == '\n' {
                self.line_offset = self.offset;
            }

            let s = &self.src[self.rd_offset..];
            let (r, w) = match std::str::from_utf8(s) {
                Ok(valid) => {
                    let ch = valid.chars().next().unwrap_or(EOF);
                    let len = ch.len_utf8();
                    (ch, len)
                }
                Err(_) => (EOF, 1),
            };

            if r == '\0' {
                self.error(self.offset, "illegal character NUL");
            } else if r == '\u{FFFD}' && w == 1 {
                self.error(self.offset, "illegal UTF-8 encoding");
            } else if r == BOM && self.offset > 0 {
                self.error(self.offset, "illegal byte order mark");
            }

            self.rd_offset += w;
            self.ch = r;
        } else {
            self.offset = self.src.len();
            if self.ch == '\n' {
                self.line_offset = self.offset;
            }
            self.ch = EOF;
        }
    }

    fn peek(&self) -> u8 {
        if self.rd_offset < self.src.len() {
            self.src[self.rd_offset]
        } else {
            0
        }
    }

    fn error(&mut self, offset: usize, msg: &str) {
        if let Some(ref mut handler) = self.eh {
            let pos = self.source.position(token::Pos(offset));
            handler(pos, msg);
        }
        self.error_count += 1;
    }

    fn errorf(&mut self, offset: usize, msg: &str, args: impl std::fmt::Display) {
        self.error(offset, format!("{}{}", msg, args).as_str());
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.ch, ' ' | '\t' | '\n' | '\r') {
            self.next();
        }
    }

    pub fn scan(&mut self) -> (token::Pos, token::Token, String) {
        self.skip_whitespace();

        let pos = token::Pos(self.offset);
        let ch = self.ch;

        self.next();

        let token = match ch {
            EOF => token::Token::EOF,
            '+' => token::Token::IncByte,
            '-' => token::Token::DecByte,
            '>' => token::Token::IncPtr,
            '<' => token::Token::DecPtr,
            '[' => token::Token::LoopOpen,
            ']' => token::Token::LoopClose,
            '.' => token::Token::OutputByte,
            ',' => token::Token::InputByte,
            _ => {
                if ch != BOM {
                    self.errorf(self.offset, "illegal character ", format!("{:?}", ch));
                }
                return (pos, token::Token::ILLEGAL, ch.to_string());
            }
        };

        (pos, token, String::new())
    }
}
