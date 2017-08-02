#[macro_use]
extern crate nom;

use std::cell::Cell;

mod parser;
mod tokens;
mod state;

use parser::Parser;
use tokens::Token;
use state::State;

pub struct Regex {
    tokens: Vec<Token>,
    i: Cell<usize>,
}

impl Regex {
    pub fn new(pattern: &str) -> Self {
        let ts = Parser::new(pattern)
            .parse()
            .expect("unable to parse regex");
        Self {
            tokens: ts,
            i: Cell::new(0),
        }
    }

    pub fn is_match_str(&self, text: &str) -> bool {
        self.is_match(text.as_bytes())
    }

    pub fn is_match(&self, bytes: &[u8]) -> bool {
        let mut state = State::Init;
        for t in self.tokens.iter() {
            // println!("Token: {:?}", t);
            match *t {
                Token::Empty if bytes.len() == 0 => state = State::Match,
                Token::Empty => state = State::NoMatch,
                Token::AnyChar => {
                    self.i.set(self.i.get() + 1);
                    state = State::Match;
                }
                Token::Word => {
                    if bytes.len() <= self.i.get() {
                        state = State::NoMatch;
                    } else if (bytes[self.i.get()] as char).is_alphanumeric() {
                        self.i.set(self.i.get() + 1);
                        state = State::Match;
                    } else {
                        state = State::NoMatch;
                    }
                }
                Token::Digit => {
                    if bytes.len() <= self.i.get() {
                        state = State::NoMatch;
                    } else if (bytes[self.i.get()] as char).is_numeric() {
                        self.i.set(self.i.get() + 1);
                        state = State::Match;
                    } else {
                        state = State::NoMatch;
                    }
                }
                Token::NonDigit => {
                    if bytes.len() <= self.i.get() {
                        state = State::NoMatch;
                    } else if !(bytes[self.i.get()] as char).is_numeric() {
                        self.i.set(self.i.get() + 1);
                        state = State::Match;
                    } else {
                        state = State::NoMatch;
                    }
                }
                Token::Whitespace => {
                    if bytes.len() <= self.i.get() {
                        state = State::NoMatch;
                    } else if (bytes[self.i.get()] as char).is_whitespace() {
                        self.i.set(self.i.get() + 1);
                        state = State::Match;
                    } else {
                        state = State::NoMatch;
                    }
                }
                Token::Literal(ref lit_bytes) => {
                    for b in lit_bytes {
                        if bytes.len() <= self.i.get() {
                            state = State::NoMatch;
                            break;
                        } else if *b == bytes[self.i.get()] {
                            self.i.set(self.i.get() + 1);
                            state = State::Match;
                        } else {
                            state = State::NoMatch;
                            break;
                        }
                    }
                }
            }
        }
        // println!("state: {:?}", state);
        state == State::Match
    }
}

#[cfg(test)]
mod tests {
    use super::Regex;
    #[test]
    fn construct() {
        let _ = Regex::new("a");
    }

    #[test]
    fn empty_literal() {
        let r = Regex::new("");
        assert!(r.is_match_str(""));
    }

    #[test]
    fn empty_literal_fail() {
        let r = Regex::new("");
        assert!(!r.is_match_str("a"));
    }

    #[test]
    fn literal() {
        let r = Regex::new("a");
        assert!(r.is_match_str("a"));
    }

    #[test]
    fn literal_fail() {
        let r = Regex::new("a");
        assert!(!r.is_match_str("b"));
    }

    #[test]
    fn literal_string() {
        let r = Regex::new("abc");
        assert!(r.is_match_str("abc"));
    }

    #[test]
    fn literal_string_fail() {
        let r = Regex::new("abc");
        assert!(!r.is_match_str("bbc"));
    }

    #[test]
    fn literal_string_num() {
        let r = Regex::new("123abc");
        assert!(r.is_match_str("123abc"));
    }

    #[test]
    fn literal_string_num_fail() {
        let r = Regex::new("123abc");
        assert!(!r.is_match_str("123"));
    }

    #[test]
    fn any() {
        let r = Regex::new(".");
        assert!(r.is_match_str("1"));
    }

    #[test]
    fn any_multiple() {
        let r = Regex::new(".......");
        assert!(r.is_match_str("abdie38"));
    }

    #[test]
    fn any_followed_literal() {
        let r = Regex::new(".abc");
        assert!(r.is_match_str("1abc"));
    }

    #[test]
    fn any_followed_literal_fail() {
        let r = Regex::new(".a");
        assert!(r.is_match_str("1a"));
    }

    #[test]
    fn word() {
        let r = Regex::new(r"\w");
        assert!(r.is_match_str("a"));
    }

    #[test]
    fn word_fail() {
        let r = Regex::new(r"\w");
        assert!(!r.is_match_str("!"));
    }

    #[test]
    fn digit() {
        let r = Regex::new(r"\d");
        assert!(r.is_match_str("1"));
    }

    #[test]
    fn digit_fail() {
        let r = Regex::new(r"\d");
        assert!(!r.is_match_str("a"));
    }

    #[test]
    fn nondigit() {
        let r = Regex::new(r"\D");
        assert!(r.is_match_str("a"));
    }

    #[test]
    fn nondigit_fail() {
        let r = Regex::new(r"\D");
        assert!(!r.is_match_str("1"));
    }

    #[test]
    fn whitespace() {
        let r = Regex::new(r"\s");
        assert!(r.is_match_str(" "));
    }

    #[test]
    fn whitespace_fail() {
        let r = Regex::new(r"\s");
        assert!(!r.is_match_str("1"));
    }
}
