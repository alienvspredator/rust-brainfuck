use std::rc::Rc;
use crate::token;

trait Spanned {
    fn pos(&self) -> token::Pos;
    fn end(&self) -> token::Pos;
}

#[derive(Debug)]
pub enum Node {
    Program(Program),
    IncPtr(IncPtr),
    DecPtr(DecPtr),
    IncByte(IncByte),
    DecByte(DecByte),
    OutputByte(OutputByte),
    InputByte(InputByte),
    Loop(Loop),
    Body(Body),
    BadNode(BadNode),
}

impl Spanned for Node {
    fn pos(&self) -> token::Pos {
        match self {
            Node::Program(n) => n.pos(),
            Node::IncPtr(n) => n.pos(),
            Node::DecPtr(n) => n.pos(),
            Node::IncByte(n) => n.pos(),
            Node::DecByte(n) => n.pos(),
            Node::OutputByte(n) => n.pos(),
            Node::InputByte(n) => n.pos(),
            Node::Loop(n) => n.pos(),
            Node::Body(n) => n.pos(),
            Node::BadNode(n) => n.pos(),
        }
    }

    fn end(&self) -> token::Pos {
        match self {
            Node::Program(n) => n.end(),
            Node::IncPtr(n) => n.end(),
            Node::DecPtr(n) => n.end(),
            Node::IncByte(n) => n.end(),
            Node::DecByte(n) => n.end(),
            Node::OutputByte(n) => n.end(),
            Node::InputByte(n) => n.end(),
            Node::Loop(n) => n.end(),
            Node::Body(n) => n.end(),
            Node::BadNode(n) => n.end(),
        }
    }
}

macro_rules! simple_node {
    ($name:ident, $field:ident) => {
        #[derive(Debug)]
        pub struct $name {
            pub $field: token::Pos,
        }

        impl Spanned for $name {
            fn pos(&self) -> token::Pos {
                self.$field
            }

            fn end(&self) -> token::Pos {
                self.$field + 1usize
            }
        }
    };
}

macro_rules! simple_nodes {
    ([$($name:ident),*], $field:ident) => {
        $(simple_node!($name, $field);)*
    };
}

simple_nodes!([
    IncPtr,
    DecPtr,
    IncByte,
    DecByte,
    OutputByte,
    InputByte
], pos);

#[derive(Debug)]
pub struct Program {
    pub body: Rc<Node>,
}

impl Program {
    pub fn pos(&self) -> token::Pos {
        0.into()
    }

    pub fn end(&self) -> token::Pos {
        0.into()
    }
}

#[derive(Debug)]
pub struct Loop {
    pub pos: token::Pos,
    pub body: Rc<Node>,
}

impl Spanned for Loop {
    fn pos(&self) -> token::Pos {
        self.pos
    }

    fn end(&self) -> token::Pos {
        if let Node::Body(body) = self.body.as_ref() {
            body.end()
        } else {
            self.pos + 1usize
        }
    }
}

#[derive(Debug)]
pub struct Body {
    pub pos: token::Pos,
    pub list: Vec<Node>,
}

impl Spanned for Body {
    fn pos(&self) -> token::Pos {
        self.pos
    }

    fn end(&self) -> token::Pos {
        if let Some(last) = self.list.last() {
            last.end()
        } else {
            self.pos + 1usize
        }
    }
}

#[derive(Debug)]
pub struct BadNode {
    pub from: token::Pos,
    pub to: token::Pos,
}

impl Spanned for BadNode {
    fn pos(&self) -> token::Pos {
        self.from
    }

    fn end(&self) -> token::Pos {
        self.to
    }
}
