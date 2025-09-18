use regex_engine::*;

fn main() {
    let r = Regex::new("[0-9]{3,}");

    // Full match
    assert!(r.check("123"));

    // Partial match
    assert!(r.has_match("abc 123 def"));

    // Search
    assert_eq!(r.search("abc 123 def"), Some((4, 7)));
}
