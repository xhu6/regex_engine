# Regex engine in Rust

Matches text against a pattern using a NFA in linear time to haystack length.

Usage:
```rs
let r = Regex::new("[0-9]{3,}");

// Full match
assert!(r.check("123"));

// Partial match
assert!(r.has_match("abc 123 def"));

// Search
assert_eq!(r.search("abc 123 def"), Some((4, 7)));
```

Engine features:
- Check text fully matches `check`
- Check text contains a match `has_match`
- Search text for a match `search`

Regex features:
- Character classes
- Ranges

## Parsing

The AST generated assumes left-associative operations.

The precendence of operations is... (top is highest)
- Brackets
- Quantifiers
- Concatenation
- Union

The parser uses these production rules:
```ebnf
(* Union *)
regex = a_exp {"|" a_exp};

(* Concatenation *)
a_exp = b_exp {b_exp};

b_exp = unit quantifier;

unit = literal
     | "[" ["^"] span {span} "]" (* Character class *)
     | "(" regex ")";

span = literal ["-" literal];

quantifier = "?" | "*" | "+"
           | "{" numeral "}"
           | "{" numeral "," [numeral] "}";
(* Upper range defaults to inf *)
```
