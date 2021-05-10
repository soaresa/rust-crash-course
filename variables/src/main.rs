fn main() {
    /*
    All variables in rust are immutable by default.
    To make possible change a variable you must use the keyword "mut" before the variable name
    */
    let mut x = 21;

    println!("The value of x is {}", x);

    x = 42;

    println!("And now is {}", x);
}
