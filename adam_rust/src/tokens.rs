#[derive(Debug, Clone)]
pub(crate) enum Token {
    Empty,
    Literal(Vec<u8>),
    AnyChar,
    Word,
    Digit,
    NonDigit,
    Whitespace,
}
