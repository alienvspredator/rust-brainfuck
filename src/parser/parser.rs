use crate::ast;
use crate::scanner::{ErrorList, Scanner};
use crate::token::{self, Token};
use std::cell::RefCell;
use std::panic;
use std::rc::Rc;

pub struct Parser<'a> {
    source: Rc<token::Source>,
    pub(crate) errors: Rc<RefCell<ErrorList>>,
    scanner: Scanner<'a>,

    pos: token::Pos,
    tok: Token,
    lit: String,

    nested_lev: usize,
}

#[derive(Debug)]
pub(crate) struct Bailout {
    pub(crate) pos: token::Pos,
    pub(crate) msg: String,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a [u8]) -> Self {
        let source = Rc::new(token::Source::new(src.len()));
        let errors = Rc::new(RefCell::new(ErrorList::new()));
        let errors_for_scanner = errors.clone();
        let eh = move |pos: token::Position, msg: &str| {
            errors_for_scanner.borrow_mut().add(pos, msg);
        };

        let scanner = Scanner::new(source.clone(), src, Some(Box::new(eh)));

        let mut parser = Self {
            source,
            scanner,
            pos: Default::default(),
            tok: Token::ILLEGAL,
            lit: String::new(),
            errors,
            nested_lev: 0,
        };
        parser.next();
        parser
    }

    fn next(&mut self) {
        let (pos, tok, lit) = self.scanner.scan();
        self.pos = pos;
        self.tok = tok;
        self.lit = lit;
    }

    fn inc_nest_lev(&mut self) {
        const MAX_NEST_LEV: usize = 100_000;

        self.nested_lev += 1;
        if self.nested_lev > MAX_NEST_LEV {
            self.error(self.pos, "exceeded max nesting depth");
            panic::panic_any(Bailout {
                pos: self.pos,
                msg: "exceeded max nesting depth".to_string(),
            })
        }
    }

    fn dec_nest_lev(&mut self) {
        self.nested_lev -= 1;
    }

    pub(crate) fn error(&mut self, pos: token::Pos, msg: impl Into<String>) {
        self.errors
            .borrow_mut()
            .add(self.source.position(pos), msg.into());
    }

    fn error_expected(&mut self, pos: token::Pos, msg: &str) {
        let mut message = format!("expected {}", msg);
        if pos == self.pos {
            message += &format!(", found '{}'", self.tok);
        }

        self.error(pos, message);
    }

    fn expect(&mut self, tok: Token) -> token::Pos {
        let pos = self.pos;
        if self.tok != tok {
            self.error_expected(pos, &tok.to_string());
        }
        self.next(); // make progress
        pos
    }

    fn expect2(&mut self, tok: Token) -> token::Pos {
        let pos = if self.tok == tok {
            self.pos
        } else {
            self.error_expected(self.pos, format!("'{}'", tok.to_string()).as_str());
            token::NO_POS
        };
        self.next();
        pos
    }

    fn advance(&mut self) {
        while self.tok != Token::EOF {
            self.next();
        }
    }

    fn parse_inc_ptr(&mut self) -> ast::IncPtr {
        ast::IncPtr {
            pos: self.expect(Token::IncPtr),
        }
    }

    fn parse_dec_ptr(&mut self) -> ast::DecPtr {
        ast::DecPtr {
            pos: self.expect(Token::DecPtr),
        }
    }

    fn parse_inc_byte(&mut self) -> ast::IncByte {
        ast::IncByte {
            pos: self.expect(Token::IncByte),
        }
    }

    fn parse_dec_byte(&mut self) -> ast::DecByte {
        ast::DecByte {
            pos: self.expect(Token::DecByte),
        }
    }

    fn parse_output_byte(&mut self) -> ast::OutputByte {
        ast::OutputByte {
            pos: self.expect(Token::OutputByte),
        }
    }

    fn parse_input_byte(&mut self) -> ast::InputByte {
        ast::InputByte {
            pos: self.expect(Token::InputByte),
        }
    }

    fn parse_node_list(&mut self) -> Vec<ast::Node> {
        let mut list = Vec::new();
        while self.tok != Token::LoopClose && self.tok != Token::EOF {
            list.push(self.parse_node());
        }
        list
    }

    fn parse_body(&mut self) -> ast::Body {
        ast::Body {
            pos: self.pos,
            list: self.parse_node_list(),
        }
    }

    fn parse_loop(&mut self) -> ast::Loop {
        let pos = self.pos;
        self.expect(Token::LoopOpen);
        let body = self.parse_body();
        self.expect2(Token::LoopClose);

        ast::Loop {
            pos,
            body: Rc::new(ast::Node::Body(body)),
        }
    }

    fn parse_node(&mut self) -> ast::Node {
        self.inc_nest_lev();
        let result = match self.tok {
            Token::IncPtr => ast::Node::IncPtr(self.parse_inc_ptr()),
            Token::DecPtr => ast::Node::DecPtr(self.parse_dec_ptr()),
            Token::IncByte => ast::Node::IncByte(self.parse_inc_byte()),
            Token::DecByte => ast::Node::DecByte(self.parse_dec_byte()),
            Token::OutputByte => ast::Node::OutputByte(self.parse_output_byte()),
            Token::InputByte => ast::Node::InputByte(self.parse_input_byte()),
            Token::LoopOpen => ast::Node::Loop(self.parse_loop()),
            _ => {
                let pos = self.pos;
                self.error_expected(pos, "node");
                self.advance();
                ast::Node::BadNode(ast::BadNode {
                    from: pos,
                    to: self.pos,
                })
            }
        };
        self.dec_nest_lev();
        result
    }

    pub fn parse_program(&mut self) -> Option<ast::Program> {
        if !self.errors.borrow().is_empty() {
            return None;
        }

        let mut nodes = Vec::new();
        while self.tok != Token::EOF {
            nodes.push(self.parse_node());
        }

        Some(ast::Program {
            body: Rc::new(ast::Node::Body(ast::Body {
                list: nodes,
                pos: self.pos,
            })),
        })
    }

    pub fn errors(&self) -> std::cell::Ref<'_, ErrorList> {
        self.errors.borrow()
    }
}
