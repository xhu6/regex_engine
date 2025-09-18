use regex_engine::*;

fn main() {
    // Test evil regex
    let r = Regex::new("a{3,}");
    let res = r.check("a");
    println!("{res}");
}
