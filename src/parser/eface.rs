use crate::ast::{Body, Node, Program};
use crate::parser::{Bailout, Parser};
use std::error::Error;
use std::io::Read;
use std::rc::Rc;

pub fn parse_program_from<T: IntoSource>(src: T) -> Result<Node, Box<dyn Error>> {
    let text = src.into_bytes()?;

    let mut parser = Parser::new(&text);
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| parser.parse_program()));

    let prog = match result {
        Ok(Some(p)) => p,
        Ok(None) => Program {
            body: Rc::new(Node::Body(Body {
                pos: Default::default(),
                list: vec![],
            })),
        },
        Err(e) => {
            if let Some(bail) = e.downcast_ref::<Bailout>() {
                parser.error(bail.pos, &bail.msg);
            } else {
                return Err("unexpected parser panic".into());
            }

            Program {
                body: Rc::new(Node::Body(Body {
                    pos: Default::default(),
                    list: vec![],
                })),
            }
        }
    };

    let mut errors = parser.errors.borrow_mut();
    errors.sort();
    errors
        .err()
        .map_or(Ok(Node::Program(prog)), |e| Err(Box::new(e.clone())))
}

pub trait IntoSource {
    fn into_bytes(self) -> Result<Vec<u8>, Box<dyn Error>>;
}

impl IntoSource for &str {
    fn into_bytes(self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(self.as_bytes().to_vec())
    }
}

impl IntoSource for String {
    fn into_bytes(self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(self.into_bytes())
    }
}

impl IntoSource for &[u8] {
    fn into_bytes(self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(self.to_vec())
    }
}

impl<T: Read + 'static> IntoSource for Box<T> {
    fn into_bytes(mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut buf = Vec::new();
        self.read_to_end(&mut buf)?;
        Ok(buf)
    }
}
