use std::str::Chars;
use parsec_rust::core::traits::{
    parser::{Parser, ParseState},
};


pub struct Char {
    ch: char, // 要判断的字符
}

impl Parser for Char {
    type Target = char; // 解析结果的类型是字符型
    fn parse<'a>(&self, state: &mut ParseState<'a>) -> Option<Self::Target> {
        state.src.next()
            .filter(|&ch| self.ch == ch)
            .map(|ch| {
                state.update_pos(ch);
                ch
            })
    }
}

pub fn char(ch: char) -> Char {
    Char { ch }
}


fn main() {
    println!("Testing parser combinators...");
    println!("{}\n", String::from_utf8(vec![b'='; 90]).unwrap());

    let a = char('a');
    let mut input = ParseState::new("aabb");
    let output = a.parse(&mut input);
    println!("{:?}", output);
    println!("{:?}", input);
}
