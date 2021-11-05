
use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

fn main() {
    println!("Hello, world!");
    let data = fs::read_to_string("./scenes/scene.json").expect("issue reading data from file");

    let p: Person = serde_json::from_str(&data).expect("issue deserializing person");

    // Do things just like with any other Rust data structure.
    println!("Please call {} at the number {}", p.name, p.phones[0]);
}
