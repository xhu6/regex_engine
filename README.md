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
```rs
// Union
A -> B|A
A -> B

// Concatenation
B -> CB
B -> C

// Quantifiers
C -> D+
C -> D?
C -> D*
C -> D

// Expressions
D -> (A)
D -> Literal
```
