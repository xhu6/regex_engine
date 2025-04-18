# Regex engine in Rust

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
