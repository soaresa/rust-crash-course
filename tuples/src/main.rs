fn main() {
    let tup1 = (1, "Rust", false, 8, (11, 23, 15));
    println!("{}", tup1.3);
    println!("{}", (tup1.4).0);

    let tup2 = (32.1, true, "apple");
    let (a, b, c) = tup2;  // extract tuple values to variables  
    println!("a is {}", a);
    println!("b is {}", b);
    println!("c is {}", c);
}