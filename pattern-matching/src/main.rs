fn main() {
    // numbers
    let number = 14;
    
    match number {
        1 => println!("It is one"),
        2 => println!("It is two"),
        3..=10 => println!("It is between 3 and 10"),
        12 | 13 => println!("It is 12 or 13"),
        _ => println!("It does not match")
    }

    // strings
    let name = "Luna";

    match name {
        "Duda" => println!("Hi 5!"),
        "Luna" => println!("Go sleep"),
        _ => println!("It does not match any name")
    }
}
