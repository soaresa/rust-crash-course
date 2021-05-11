use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::create("Output.txt").expect("Could not create the file!");
    file.write_all(b"Welcome to world!").expect("Cannont write to the file!");
}
