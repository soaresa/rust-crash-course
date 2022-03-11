extern crate regex;

use regex::Regex;

fn main() {
    let re = Regex::new(r"\w{5}").unwrap();
    let text = "White";
    let text2 = "Walter";

    println!("Found match? {}", re.is_match(text));
    println!("Found match? {}", re.is_match(text2));

    let re2 = Regex::new(r"(\w{5})").unwrap();
    match re2.captures(text) {
        Some(caps) => println!("Found match: {}", &caps[0]), // OR caps.get(0).unwrap().as_str()
        None => println!("Match no found")
    }
}
