mod ast;
mod token;

use crate::ast::{Body, Node, Program, Visitor, walk, IncPtr, Loop, DecByte};

struct DebugVisitor;

impl Visitor for DebugVisitor {
    fn visit(&self, node: &Node) -> Option<&dyn Visitor> {
        println!("Visiting node: {:?}", node);
        Some(self)
    }
}

fn main() {
    let ast = Node::Program(Program {
        body: Box::new(Node::Body(Body {
            body_pos: 0,
            list: vec![
                Node::IncPtr(IncPtr { inc: 1 }),
                Node::Loop(Loop {
                    loop_pos: 2,
                    body: Box::new(Node::Body(Body {
                        body_pos: 3,
                        list: vec![Node::DecByte(DecByte { dec: 4 })],
                    })),
                }),
            ],
        })),
    });

    let visitor = DebugVisitor;
    walk(&visitor, &ast);
}
