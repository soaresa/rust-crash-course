struct Color {
    red: u8, // u8: 0-255
    green: u8,
    blue: u8
}

fn main() {
    let red = Color { red: 255, green: 0, blue: 0 };
    print_color(&red); // pass as a reference to not move variable into the function
    print_color(&red); 
}

fn print_color(c: &Color) {
    println!("Color - R:{} G:{} B:{}", c.red, c.green, c.blue);
}
