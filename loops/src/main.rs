fn main() {
    // exploring loops
    
    /* infinite loop */
    let mut n = 1;
    loop {
        n += 1;

        if n == 7 {
            continue; // skip everything else below this code line
        }

        if n > 30 {
            break;
        }
        println!("The n value is {}", n);
    }        
}
