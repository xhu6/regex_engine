# Regex engine in Rust

Matches text against a pattern using a lazy NFA.

## Parsing

The AST generated assumes left-associative operations.

The precendence of operations is... (top is highest)
- Brackets
- Quantifiers
- Concatenation
- Union

The parser uses these production rules:
```
A -> B|A
A -> B

B -> CB
B -> C

C -> D+
C -> D?
C -> D*
C -> D

D -> (A)
D -> Literal
```
