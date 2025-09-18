use regex_engine::Regex;

#[test]
fn match_literal() {
    let r = Regex::new("the");
    let res = r.check("the");
    assert!(res);
}

#[test]
fn match_hex() {
    let r = Regex::new("\\x61\\x62\\x63");
    let res = r.check("abc");
    assert!(res);
}

#[test]
fn match_quantifiers() {
    let r = Regex::new("a*b?c+");
    let res = r.check("aaaabc");
    assert!(res);
}

#[test]
fn match_range_quantifiers() {
    let r = Regex::new("(a{2,}b{2,4}c{1,})*");
    let res = r.check("aabbcaaaabbbbccc");
    assert!(res);
}

#[test]
fn match_union() {
    let r = Regex::new("one|two|three");
    let res = r.check("two");
    assert!(res);
}

#[test]
fn match_brackets() {
    let r = Regex::new("(b|c|d)at?");
    let res = r.check("cat");
    assert!(res);
}

#[test]
fn match_literal_fail() {
    let r = Regex::new("the");
    let res = r.check("there");
    assert!(!res);
}

#[test]
fn match_digits() {
    let r = Regex::new("[0-9]*");
    let res = r.check("1203912");
    assert!(res);
}

#[test]
fn match_non_digits() {
    let r = Regex::new("[^0-9]*");
    let res = r.check("one thousand");
    assert!(res);
}

#[test]
fn match_digits_fail() {
    let r = Regex::new("[0-9]*");
    let res = r.check("0913a");
    assert!(!res);
}
