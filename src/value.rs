use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Class {
    spans: Vec<(char, char)>,
    inverse: bool,
}

impl Class {
    pub fn new(spans: &[(char, char)], inverse: bool) -> Self {
        let mut spans: Vec<(char, char)> = spans.iter().filter(|x| x.0 <= x.1).cloned().collect();
        spans.sort_unstable();

        let mut out = Vec::new();

        if !spans.is_empty() {
            let mut new = spans[0];

            for s in &spans[1..] {
                // Use range to not deal with u32 casting
                if (new.1..s.0).nth(1).is_none() {
                    new.1 = new.1.max(s.1);
                } else {
                    out.push(new);
                    new = *s;
                }
            }

            out.push(new);
        }

        Class {
            spans: out,
            inverse,
        }
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ ")?;

        if self.inverse {
            write!(f, "^ ")?;
        }

        for span in &self.spans {
            write!(f, "{}-{} ", span.0, span.1)?;
        }

        write!(f, "]")
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Value {
    Char(char),
    Class(Class),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Char(x) => write!(f, "{x}"),
            Value::Class(x) => write!(f, "{x}"),
        }
    }
}

impl Value {
    pub fn matches(&self, value: char) -> bool {
        match self {
            Value::Char(x) => *x == value,

            Value::Class(x) => {
                let r = x.spans.binary_search_by_key(&value, |x| x.0);

                let accepted = r.is_ok()
                    || r.err()
                        .and_then(|idx| idx.checked_sub(1))
                        .and_then(|idx| x.spans.get(idx))
                        .map(|b| b.0 <= value && value <= b.1)
                        .unwrap_or(false);

                x.inverse ^ accepted
            }
        }
    }

    pub fn class(spans: &[(char, char)], inverse: bool) -> Self {
        Self::Class(Class::new(spans, inverse))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn char_matches() {
        assert!(Value::Char('a').matches('a'));
        assert!(!Value::Char('a').matches('b'));
    }

    #[test]
    fn empty() {
        let t = vec![];
        let c = Class::new(&t, false);
        assert!(c.spans.is_empty());
    }

    #[test]
    fn disjoint() {
        let t = vec![('a', 'c'), ('e', 'k'), ('o', 'z')];
        let c = Class::new(&t, false);
        assert_eq!(c.spans, t);
    }

    #[test]
    fn unordered_disjoint() {
        let mut t = vec![('e', 'k'), ('a', 'c'), ('o', 'z')];
        let c = Class::new(&t, false);

        t.sort_unstable();
        assert_eq!(c.spans, t);
    }

    #[test]
    fn overlapping() {
        let t = vec![('f', 'x'), ('a', 'n')];
        let c = Class::new(&t, false);

        assert_eq!(c.spans, vec![('a', 'x')]);
    }

    #[test]
    fn contained() {
        let t = vec![('f', 'x'), ('a', 'n'), ('0', 'z')];
        let c = Class::new(&t, false);

        assert_eq!(c.spans, vec![('0', 'z')]);
    }

    #[test]
    fn adjacent() {
        let t = vec![('a', 'g'), ('h', 'n'), ('n', 'z')];
        let c = Class::new(&t, false);

        assert_eq!(c.spans, vec![('a', 'z')]);
    }

    #[test]
    fn mixed() {
        let t = vec![('f', 'x'), ('a', 'c'), ('f', 'z')];
        let c = Class::new(&t, false);

        assert_eq!(c.spans, vec![('a', 'c'), ('f', 'z')]);
    }

    #[test]
    fn invalid() {
        let t = vec![('z', 'y')];
        let c = Class::new(&t, false);

        assert!(c.spans.is_empty());
    }

    #[test]
    fn class_matches_empty_fail() {
        let t = vec![];
        let v = Value::class(&t, false);

        assert!(!v.matches('a'));
    }

    #[test]
    fn class_matches_extensive() {
        let t = vec![('b', 'd'), ('f', 'g'), ('j', 'j')];
        let v = Value::class(&t, false);

        assert!(!v.matches('A'));
        assert!(!v.matches('a'));

        assert!(v.matches('b'));
        assert!(v.matches('c'));
        assert!(v.matches('d'));

        assert!(!v.matches('e'));

        assert!(v.matches('f'));
        assert!(v.matches('g'));

        assert!(!v.matches('h'));
        assert!(!v.matches('i'));

        assert!(v.matches('j'));

        assert!(!v.matches('k'));
        assert!(!v.matches('l'));
    }
}
