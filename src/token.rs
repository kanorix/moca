#[derive(Debug)]
pub enum TokenKind {
    // operator (演算子)
    Plus,     // +
    Minus,    // -
    Multi,    // *
    Div,      // /
    Not,      // !
    Assign,   // =
    Equal,    // ==
    NotEqual, // !=
    And,      // &&
    Or,       // ||
    Dot,      // .

    // separator (区切り子)
    Lparen,  // (
    Rparen,  // )
    Lcurly,  // {
    Rcurly,  // }
    Lsquare, // [
    Rsquare, // ]
    Colon,   // :
    Scolon,  // ;

    // keyword
    Void,    // void
    If,      // if
    Else,    // else
    For,     // for
    New,     // new
    Class,   // class
    Int,     // int
    Double,  // double
    Boolean, // boolean

    // others
    Literal, // リテラル (整数・小数・文字列)
    Ident,   // 変数名・関数名
    Other,   // その他
}

#[derive(Debug)]
pub enum Value {
    // Address(i64),
    Integer(i64),
    Double(f64),
    String(String),
    Boolean(bool),
    Symbol(String),
}

// #[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    value: Value,
}
impl Token {
    pub fn new(kind: TokenKind, value: Value) -> Self {
        Token { kind, value }
    }

    pub fn to_string(&self) -> String {
        format!("Token( {:?}, {:?} )", self.kind, self.value)
    }
}
