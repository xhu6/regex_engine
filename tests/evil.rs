use regex_engine::Regex;

#[test]
fn evil_union() {
    let r = Regex::new("(a|a|a|b|a|a|a|a|a)+");
    let res = r.check("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab");
    assert!(res);
}

#[test]
fn evil_union_fail() {
    let r = Regex::new("(a|a)+");
    let res = r.check("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab");
    assert!(!res);
}

#[test]
fn evil_multiple_quantifier() {
    let r = Regex::new("((a+)*b)*");
    let res = r.check("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab");
    assert!(res);
}

#[test]
fn evil_multiple_quantifier_fail() {
    let r = Regex::new("((a+)*)*");
    let res = r.check("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab");
    assert!(!res);
}

#[test]
fn evil_repeated_pattern() {
    let r = Regex::new("a*a*b?a*a*");
    let res = r.check("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab");
    assert!(res);
}

#[test]
fn evil_repeated_pattern_fail() {
    let r = Regex::new("a*a*");
    let res = r.check("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab");
    assert!(!res);
}
