fn main() {
    let x = 40;

    { 
        let x = 20;
        println!("1.1) x is {}", x);
    }

    println!("1.2) x is {}", x);

    let y = "a string";
    println!("2.1) y is {}", y);
    let y = true;
    println!("2.2) y is {}", y);
}
