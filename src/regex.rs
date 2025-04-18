use std::fmt::Display;

use crate::lexer::*;
use crate::nfa::*;
use crate::parser::*;

pub struct Regex {
    nfa: NFA,
}

impl Display for Regex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.nfa)
    }
}

impl Regex {
    pub fn new(pattern: &str) -> Self {
        let tokens = lexer(pattern).unwrap();
        let ast = parse(&tokens);
        let nfa = NFA::new(ast);
        Self { nfa }
    }

    pub fn check(&self, text: &str) -> bool {
        self.nfa.check(text)
    }
}
