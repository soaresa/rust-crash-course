fn main() {
    /* Replace */
    {   
        let str1 = String::from("Rust is cool!");
        println!("Sfter replace: {}", str1.replace("cool", "great"));
    }

    /* Lines */
    {
        let str2 = String::from("The weather is\nfantastic\noutside kid!");
        for line in str2.lines() {
            println!("[ {} ]", line);
        }
    }

    /* Split */
    {
        let str3 = String::from("Say+my+name!");
        let tokens: Vec<&str> = str3.split("+").collect();
        println!("At index 1: {}", tokens[1]);
    }

    /* Trim */
    {
        let str4 = String::from(" keijo \n\r ");
        println!("Before trim: {}", str4);
        println!("After trim: {}", str4.trim());
    }

    /* Chars */
    {
        let str5 = String::from("You are God damn right!");

        /* Get character at index */
        match str5.chars().nth(4) {
            Some(c) => println!("Character at index 4: {}", c),
            None => println!("No character at index 4")
        }
    }
}
