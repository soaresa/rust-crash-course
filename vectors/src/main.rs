fn main() {
    let mut vector1: Vec<i32> = Vec::new();
    vector1.push(7);
    vector1.push(13);
    println!("{}", vector1[0]);
    println!("{}", vector1[1]);

    let mut vector2 = vec![1, 2, 3];
    println!("{}", vector2[0]); // print 1
    vector2.remove(0); // remove element 1
    println!("{}", vector2[0]); // print 2
    vector2.push(42);
    println!("{}", vector2[0]); // print 2
    println!("{}", vector2[2]); // print 42
}

