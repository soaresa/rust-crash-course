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
    
    // while loop
    let mut x = 1;
    let y = "foo";
    while x < 30 {
        // if x is a multiple of 5
        if x % 5 == 0 {
            println!("X is: {}, {}", x, y);
        }        
        x +=1;
    }

    // for loop (11 not inclusive)
    for i in 1..11 {
        println!("The numer is {}", i);
    }

    let numbers = 101..111;
    for j in numbers {
        println!("Number: {}", j);
    }

    let colors = vec!["green", "gray", "blue", "red"];
    for c in colors.iter() { // must use iter() to refer to colors again below
        println!("Color {}.", c);
    }

    for (index, a) in colors.iter().enumerate() {
        println!("Color at {} is {}.", index, a);
    }
}
