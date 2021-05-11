// add methods to a struct 
struct Vehicle {
    doors: u8,
    seats: u8
}

impl Vehicle {
    fn print_description(&self) {
        println!("Vehicle with {} doors and {} seats.", self.doors, self.seats);
    }
    fn is_seats_even(&self) -> bool {
        return self.seats % 2 == 0;
    }
}

fn main() {
    let x = Vehicle { doors: 5, seats: 8 };
    x.print_description();
    println!("Is seats even: {} ", x.is_seats_even());
}
