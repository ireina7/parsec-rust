use std::{rc::Rc, str::Chars};


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Pos {
    pub col: usize,
    pub row: usize,
}


#[derive(Clone, Debug)]
pub struct ParseState<'a> {
    pub src: Chars<'a>,
    pub pos: Pos,
}



impl<'a> ParseState<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src: src.chars(),
            pos: Pos { col: 0, row: 0 },
        }
    }

    pub fn as_str(&self) -> &'a str {
        self.src.as_str()
    }

    pub fn pos(&self) -> Pos {
        self.pos
    }

    pub fn update_pos(&self, ch: char) -> () {
        
    }
}



/// Interface for parsers
/// Main Parser trait
pub trait Parser {
    type Target;
    fn parse<'a>(&self, stream: &mut ParseState<'a>) -> Option<Self::Target>;
}


