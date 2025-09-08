use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};

const PATTERN: &str = "(a|å|b){16}b+(b|å|a){16}";
const TEXT: &str = "abbabababababbabbbbbbbbbbbbbabbbbabababaaaa";

pub fn regex_engine_bench(c: &mut Criterion) {
    use regex_engine::Regex;

    c.bench_function("regex_engine new", |b| {
        b.iter(|| Regex::new(black_box(PATTERN)))
    });

    let r = Regex::new(PATTERN);

    c.bench_function("regex_engine check", |b| {
        b.iter(|| r.check(black_box(TEXT)))
    });
}

pub fn regex_crate_bench(c: &mut Criterion) {
    use regex::Regex;

    c.bench_function("regex new", |b| b.iter(|| Regex::new(black_box(PATTERN))));

    // Full Match
    let p = "^".to_string() + PATTERN + "$";
    let r = Regex::new(&p).unwrap();

    c.bench_function("regex is_match", |b| b.iter(|| r.is_match(black_box(TEXT))));
}

criterion_group!(benches, regex_engine_bench, regex_crate_bench);
criterion_main!(benches);
