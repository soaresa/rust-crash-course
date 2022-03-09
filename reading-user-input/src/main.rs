use std::io;

fn main() {
    let mut input = String::new();

    println!("Hello there. Say something:");

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("Success! You wrote: {}", input.to_uppercase());
        },
        Err(e) => println!("Oops! Something went wrong: {}", e)
    }
}
