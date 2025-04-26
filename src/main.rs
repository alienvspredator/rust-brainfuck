mod ast;
mod token;
mod parser;
mod scanner;

use std::error::Error;
use std::rc::Rc;
use crate::ast::{Body, Node, Program, Visitor, walk, IncPtr, Loop, DecByte};
use crate::parser::parse_program_from;

struct DebugVisitor;

impl Visitor for DebugVisitor {
    fn visit(&self, node: &Node) -> Option<&dyn Visitor> {
        println!("Visiting node: {:?}", node);
        Some(self)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let prog = parse_program_from(">++++++++[<+++++++++>-]<.
>++++[<+++++++>-]<+.
+++++++..
+++.
>>++++++[<+++++++>-]<++.
------------.
>++++++[<+++++++++>-]<+.
<.
+++.
------.
--------.
>>>++++[<++++++++>-]<+.")?;

    let visitor = DebugVisitor;
    walk(&visitor, &prog);
    Ok(())
}
