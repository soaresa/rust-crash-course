fn main() {
    let x = 10;
    {
        let y = 5; // only accessible inside this block
        println!("x: {}, y: {}", x , y)
    }
}
