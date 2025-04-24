use crate::token;

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

impl Node {
    pub fn pos(&self) -> token::Pos {
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

    pub fn end(&self) -> token::Pos {
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

#[derive(Debug)]
pub struct Program {
    pub body: Box<Node>,
}

impl Program {
    pub fn pos(&self) -> token::Pos {
        0
    }

    pub fn end(&self) -> token::Pos {
        0
    }
}

#[derive(Debug)]
pub struct IncPtr {
    pub inc: token::Pos,
}

impl IncPtr {
    pub fn pos(&self) -> token::Pos {
        self.inc
    }

    pub fn end(&self) -> token::Pos {
        self.inc + 1
    }
}

#[derive(Debug)]
pub struct DecPtr {
    pub dec: token::Pos,
}
impl DecPtr {
    pub fn pos(&self) -> token::Pos {
        self.dec
    }

    pub fn end(&self) -> token::Pos {
        self.dec + 1
    }
}

#[derive(Debug)]
pub struct IncByte {
    pub inc: token::Pos,
}
impl IncByte {
    pub fn pos(&self) -> token::Pos {
        self.inc
    }

    pub fn end(&self) -> token::Pos {
        self.inc + 1
    }
}

#[derive(Debug)]
pub struct DecByte {
    pub dec: token::Pos,
}
impl DecByte {
    pub fn pos(&self) -> token::Pos {
        self.dec
    }

    pub fn end(&self) -> token::Pos {
        self.dec + 1
    }
}

#[derive(Debug)]
pub struct OutputByte {
    pub token_pos: token::Pos,
}
impl OutputByte {
    pub fn pos(&self) -> token::Pos {
        self.token_pos
    }

    pub fn end(&self) -> token::Pos {
        self.token_pos + 1
    }
}

#[derive(Debug)]
pub struct InputByte {
    pub token_pos: token::Pos,
}
impl InputByte {
    pub fn pos(&self) -> token::Pos {
        self.token_pos
    }

    pub fn end(&self) -> token::Pos {
        self.token_pos + 1
    }
}

#[derive(Debug)]
pub struct Loop {
    pub loop_pos: token::Pos,
    pub body: Box<Node>,
}

impl Loop {
    pub fn pos(&self) -> token::Pos {
        self.loop_pos
    }

    pub fn end(&self) -> token::Pos {
        if let Node::Body(body) = &*self.body {
            body.end()
        } else {
            self.loop_pos + 1
        }
    }
}

#[derive(Debug)]
pub struct Body {
    pub body_pos: token::Pos,
    pub list: Vec<Node>,
}
impl Body {
    pub fn pos(&self) -> token::Pos {
        self.body_pos
    }

    pub fn end(&self) -> token::Pos {
        if let Some(last) = self.list.last() {
            last.end()
        } else {
            self.body_pos + 1
        }
    }
}

#[derive(Debug)]
pub struct BadNode {
    pub from: token::Pos,
    pub to: token::Pos,
}
impl BadNode {
    pub fn pos(&self) -> token::Pos {
        self.from
    }

    pub fn end(&self) -> token::Pos {
        self.to
    }
}
