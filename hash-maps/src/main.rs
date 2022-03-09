use std::collections::HashMap;

fn main() {
    let mut marks = HashMap::new();

    // add values
    marks.insert("Rust Programming", 96);
    marks.insert("UX Design", 78);
    marks.insert("Web Development", 89);
   
    // find length of hash map
    println!("You have studied {} subjects.", marks.len());

    // get a single value
    match marks.get("Web Development") {
        Some(mark) => println!("You got {} for Web Development", mark),
        None => println!("You didn't study Web Development")
    }

    // remove a value
    marks.remove("UX Design");

    // loop through HashMap
    for (subject, mark) in &marks {
        println!("For {} you got {}", subject, mark);
    }

    // check for value
    println!("Did you study C++? {}", marks.contains_key("C++ Programming"));
}
