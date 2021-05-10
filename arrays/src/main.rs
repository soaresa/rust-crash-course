fn main() {
    let numbers = [1, 2, 3, 4, 5];
    for i in numbers.iter() {
        println!("{}", i);
    }   
    for i in 0..numbers.len() {
        println!("{} -> {}", i, numbers[i]);
    }

    // declaring an array with 5 i32
    let num_array: [i32; 5] = [6, 7, 8, 9, 10];
    for i in num_array.iter() {
        println!("{}", i);
    }

    // declaring an array of 400 elements with the value 2
    let big_array = [2; 400];
    for j in 0..big_array.len() {
        println!("{} -> {}", j, big_array[j]);
    }


}