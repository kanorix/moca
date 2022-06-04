#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    // operator (演算子)
    Plus,        // +
    Minus,       // -
    Asterisk,    // *
    Slash,       // /
    Bang,        // !
    Assign,      // =
    Equal,       // ==
    NotEqual,    // !=
    LessThan,    // <
    GreaterThan, // >
    And,         // &&
    Or,          // ||
    // Dot,      // .

    // separator (区切り子)
    Lparen,    // (
    Rparen,    // )
    Lcurly,    // {
    Rcurly,    // }
    Lsquare,   // [
    Rsquare,   // ]
    Colon,     // :
    SemiColon, // ;

    // keyword
    Void,    // void
    Return,  // return
    If,      // if
    Else,    // else
    While,   // while
    Let,     // let
    Int,     // int
    Double,  // double
    Boolean, // boolean
    // New,     // new
    // Class,   // class

    // リテラル
    IntLiteral,    // 整数リテラル
    DoubleLiteral, // 小数リテラル
    StringLiteral, // 文字列リテラル
    BoolLiteral,   // 真偽リテラル

    // others
    Ident, // 変数名・関数名
    Other, // その他
}

// #[derive(Debug, Clone)]
pub enum Priority {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

pub fn get_priority(token_kind: TokenKind) -> Priority {
    match token_kind {
        TokenKind::Equal | TokenKind::NotEqual => Priority::Equals,
        TokenKind::LessThan | TokenKind::GreaterThan => Priority::LessGreater,
        TokenKind::Plus | TokenKind::Minus => Priority::Sum,
        TokenKind::Asterisk | TokenKind::Slash => Priority::Product,
        _ => Priority::Lowest,
    }
}

// #[derive(Debug, Clone)]
// pub enum Value {
//     Int(i32),    // 整数リテラル
//     Double(f64), // 小数リテラル
//     Str(String), // 文字列リテラル
//     Bool(bool),  // 真偽リテラル
//     Symbol(String),
//     None,
// }

#[derive(Debug, Clone)]
pub struct Token {
    pub token_kind: TokenKind,
    pub value: String,
}

impl Token {
    pub fn new(token_kind: TokenKind, value: String) -> Self {
        Token { token_kind, value }
    }

    pub fn is_same_kind(&self, kind: TokenKind) -> bool {
        self.token_kind == kind
    }

    pub fn to_string(&self) -> String {
        format!("Token: {:?}, {:?}", self.token_kind, self.value)
    }
}
