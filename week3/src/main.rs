#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

fn full_name(person: &Person) -> String {
    let mut full_name = person.first_name.clone();
    full_name.push_str(" ");
    full_name.push_str(&person.last_name);
    full_name
}

fn exerice1() {
    let myself = Person {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        age: 30,
    };
    println!("{:?}", myself);

    println!("My name is {}", full_name(&myself));
}

fn main() {
    // Concatenation of full name from a struct
    exerice1();
}
