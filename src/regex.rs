use std::fmt::Display;

use crate::lexer::lexer;
use crate::nfa::Nfa;
use crate::parser::parse;

pub struct Regex {
    nfa: Nfa,
}

impl Display for Regex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.nfa)
    }
}

impl Regex {
    pub fn new(pattern: &str) -> Self {
        let tokens = lexer(pattern).unwrap();
        let ast = parse(&tokens).unwrap();
        let nfa = Nfa::new(&ast);

        Self { nfa }
    }

    pub fn check(&self, text: &str) -> bool {
        self.nfa.check(text)
    }
}
