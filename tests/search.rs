use regex_engine::Regex;

#[test]
fn search_exact_text() {
    let r = Regex::new("the");
    let res = r.search("the");
    assert_eq!(res, Some((0, 3)))
}

#[test]
fn search_text() {
    let r = Regex::new("the");
    let res = r.search("another");
    assert_eq!(res, Some((3, 6)))
}

#[test]
fn search_fixed_size_word() {
    let r = Regex::new("[A-Za-z]{5}");
    let res = r.search("It was a sunny afternoon.");
    assert_eq!(res, Some((9, 14)))
}

#[test]
fn search_word() {
    let r = Regex::new("[A-Za-z]+");
    let res = r.search("> Good morning!");
    assert_eq!(res, Some((2, 6)))
}

#[test]
fn search_pattern() {
    let r = Regex::new("\\(a+\\)|a+");
    let res = r.search("f(a)aaaaaaaaaa");
    assert_eq!(res, Some((1, 4)))
}

#[test]
fn search_fail() {
    let r = Regex::new("[0-9]{3,}");
    let res = r.search("01, 23, 45, 67");
    assert_eq!(res, None)
}
