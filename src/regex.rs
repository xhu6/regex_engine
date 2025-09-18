use std::fmt::Display;

use crate::compiler::compile;
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
        let nfa = compile(&ast);

        Self { nfa }
    }

    pub fn check(&self, text: &str) -> bool {
        // Checks the text is a match.
        self.nfa.check(text)
    }

    pub fn has_match(&self, text: &str) -> bool {
        // Checks whether text contains a match.
        self.nfa.has_match(text)
    }

    pub fn search(&self, text: &str) -> Option<(usize, usize)> {
        // Finds the earliest longest match, treating quantifiers as greedy.
        self.nfa.search(text)
    }
}
