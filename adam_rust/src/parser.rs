use nom::{self, alphanumeric};
use tokens::Token;

named!(literal<&[u8], Token>,
   do_parse!(
        l: alphanumeric >>
        (Token::Literal(l.to_vec()))
    )
);

named!(empty<&[u8], Token>,
   do_parse!(eof!() >> (Token::Empty))
);

named!(any_char<&[u8], Token>,
   do_parse!(char!('.') >> (Token::AnyChar))
);

named!(word<&[u8], Token>,
    do_parse!(tag!("\\w") >> (Token::Word))
);

named!(digit<&[u8], Token>,
    do_parse!(tag!("\\d") >> (Token::Digit))
);

named!(non_digit<&[u8], Token>,
    do_parse!(tag!("\\D") >> (Token::NonDigit))
);

named!(whitespace<&[u8], Token>,
    do_parse!(tag!("\\s") >> (Token::Whitespace))
);

named!(regex_parse<&[u8], Vec<Token>>,
    many1!(
        alt!(empty | any_char | word | digit | non_digit | whitespace | literal)
    )
);

pub(crate) struct Parser {
    chars: Vec<u8>,
}

impl Parser {
    pub fn new(s: &str) -> Self {
        Self {
            chars: s.bytes().collect(),
        }
    }

    pub fn parse(self) -> Result<Vec<Token>, nom::IError> {
        regex_parse(self.chars.as_slice()).to_full_result().clone()
    }

}


