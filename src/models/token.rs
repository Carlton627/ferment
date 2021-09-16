#[derive(Debug)]
#[allow(unused)]
pub enum TokenType {
    OpenParenthesis,
    CloseParenthesis,
    BeginBracket,
    EndBracket,
    Operator,
    Character,
    Number,
    SpecialChar,
    Keyword,
    Newline,
}

#[derive(Debug)]
#[allow(unused)]
pub enum TokenDataType {
    Word(String),
    Character(char),
    Number(i32),
    Keyword(String),
}

pub struct Token {
    pub token_type: TokenType,
    pub value: TokenDataType,
}
