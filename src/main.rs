use regex_engine::*;

fn main() {
    // Test evil regex
    let r = Regex::new("(a|a)+");
    let res = r.check("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab");
    println!("{res}");
}
