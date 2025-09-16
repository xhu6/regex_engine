use regex_engine::Regex;

#[test]
fn has_match_exact_literal() {
    let r = Regex::new("the");
    let res = r.has_match("the");
    assert!(res);
}

#[test]
fn has_match_partial_literal() {
    let r = Regex::new("the");
    let res = r.has_match("another");
    assert!(res);
}

#[test]
fn has_match_words() {
    let r = Regex::new("(they're|their|there)");
    let res = r.has_match("They are over there");
    assert!(res);
}

#[test]
fn has_match_quantifier() {
    let r = Regex::new("ab*");
    let res = r.has_match("aaaaaaaa");
    assert!(res);
}

#[test]
fn has_match_empty() {
    let r = Regex::new("b*");
    let res = r.has_match("aaaaaaaaaaaaaaaaa");
    assert!(res);
}

#[test]
fn has_match_evil_fail() {
    let r = Regex::new("(a|a)*b");
    let res = r.has_match("aaaaaaaaaaaaaaaaa");
    assert!(!res);
}

#[test]
fn has_match_quantifier_fail() {
    let r = Regex::new("b+");
    let res = r.has_match("aaaaaaaaaaaaaaaaa");
    assert!(!res);
}
