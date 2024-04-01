// define a person struct
struct Person {
    name: String,
    age: u8,
}

fn main() {
    // create a person
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    println!("Name: {}", person.name);

    println!("Hello, world!");
}
