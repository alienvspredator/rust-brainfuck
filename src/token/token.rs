use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    ILLEGAL,
    EOF,
    IncPtr,
    DecPtr,
    IncByte,
    DecByte,
    OutputByte,
    InputByte,
    LoopOpen,
    LoopClose,
    Unknown(i32),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            Token::ILLEGAL => "ILLEGAL",
            Token::EOF => "EOF",
            Token::IncPtr => ">",
            Token::DecPtr => "<",
            Token::IncByte => "+",
            Token::DecByte => "-",
            Token::OutputByte => ".",
            Token::InputByte => ",",
            Token::LoopOpen => "[",
            Token::LoopClose => "]",
            Token::Unknown(tok) => return write!(f, "token({})", tok),
        };

        f.write_str(s)
    }
}
