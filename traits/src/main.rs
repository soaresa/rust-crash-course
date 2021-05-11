struct Person {
    name: String,
    age: u8
}

// implementing traits for Person
impl ToString for Person {
    fn to_string(&self) -> String {
        return format!("My name is {} and I am {},", self.name, self.age);
    }
}

fn main() {
    let alice = Person { name: String::from("Alice"), age: 21 };
    println!("{}", alice.to_string());
}

/*
A trait is a collection of methods defined for an unknown type: Self. 
They can access other methods declared in the same trait.
*/