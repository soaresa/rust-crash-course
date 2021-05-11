fn main() {
    let mut text = String::from("Hey everybody!"); // use mut to push_str
    
    // length
    println!("Length: {}", text.len());
    // is empty?
    println!("Is empty: {}", text.is_empty());

    for token in text.split_whitespace() {
        println!("{}", token);
    }
    println!("Text contais 'everybody'? {}", text.contains("everybody"));
    println!("Text contais 'body'? {}", text.contains("body"));
    println!("Text contais 'nothing'? {}", text.contains("nothing"));

    text.push_str(" Welcome to my class!");
    println!("{}", text);
}
