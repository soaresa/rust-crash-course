fn main() {
    print_numbers_to(10);
}

fn print_numbers_to(num: u32) {
    for x in 1..num {
        if (is_even(x)) {
            println!("{} is even!", x);
        } else {
            println!("{} is odd!", x);
        }
    }
}

fn is_even(num: u32) -> bool {
    return num % 2 == 0;
}
