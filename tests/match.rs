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
