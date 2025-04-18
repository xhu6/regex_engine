# Regex engine in Rust

Matches text against a pattern using a lazy NFA.

## Parsing

The parser uses these production rules.

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
