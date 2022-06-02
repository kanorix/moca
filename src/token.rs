#[derive(Debug)]
pub enum TokenKind {
    Plus,      // +
    Minus,     // -
    Multi,     // *
    Div,       // /
    Not,       // !
    Assign,    // =
    Equal,     // ==
    NotEqual,  // !=
    And,       // &&
    Or,        // ||
    Dot,       // .
    Lparen,    // (
    Rparen,    // )
    Lcurly,    // {
    Rcurly,    // }
    Lsquare,   // [
    Rsquare,   // ]
    Semicolon, // ;
    //
    If,        // if
    Else,      // else
    New,       // new
    Class,
    Ident,     // 変数名や関数名
    Literal,   // リテラル
    Other,
}

#[derive(Debug)]
pub enum Value {
    // Address(i64),
    Int(i64),
    Double(f64),
    String(String),
    Boolean(bool),
    Void,
    None,
}

// const TOKEN_PAIR: &[(&str, TokenKind)] = &[("(", TokenKind::Lparen), (")", TokenKind::Rparen)];

// #[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    text: String,
    value: Value,
}
impl Token {
    pub fn new(kind: TokenKind, text: String, value: Value) -> Self {
        Token { kind, text, value }
    }

    pub fn to_string(&self) -> String {
        format!("Token({:?}, {:?}, {:?})", self.kind, self.text, self.value)
    }
}
