fn main() {
    // exploring if statements    
    let n = 15;
    
    if n < 30 {
        println!("The number is less than 30.")
    } else {
        println!("The number is greater or equal to 30.")
    }

    if n == 15 {
        println!("The number equals 15.")
    }

    if n != 20 {
        println!("The number does not equals to 20.")
    } else if n > 20 {
        println!("The number is greater than 20.")
    }


}
