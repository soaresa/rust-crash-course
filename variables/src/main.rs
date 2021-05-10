fn main() {
    /*
    All variables in rust are immutable by default.
    To make possible change a variable you must use the keyword "mut" before the variable name
    */
    let mut x = 21;
    println!("The value of x is {}", x);
    x = 42;
    println!("And now is {}", x);

    // Data types
    let y = 66; // data type i32 (integer 32 bit)
    let w: i64 = 66; // data type i64 (integer 64 bit)
    let k: u64 = 66; // data type u64 (unsigned integer 64 bit)
    /*unsigned integer does not support negative numbers*/
    let f = 6.3; // f32
    let fb: f64 = 6.6;
    let is: bool = true;
    let not = false;

}
