# Regex engine in Rust

Matches text against a pattern using a NFA in linear time (w.r.t haystack length).

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

unit = literal | "(" regex ")";

quantifier = "?" | "*" | "+"
           | "{" numeral "}"
           | "{" numeral "," [numeral] "}";
(* Upper range defaults to inf *)
```
