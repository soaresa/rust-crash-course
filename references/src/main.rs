fn main() {
    let mut x = 7;
    let xr = &x;

    println!("1. x is {}", x);
    println!("2. xr is {}", xr);

    let mut y = 13;
    {
        let adal = &mut y;
        *adal += 1;
        println!("4. adal is {}", adal);
    }
        
    println!("5. y is {}", y);   
}
