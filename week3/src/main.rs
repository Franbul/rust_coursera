#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

struct Person2 {
    first_name: String,
    last_name: String,
    age: Option<u8>,
    size: Option<u8>,
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn new(username: String, email: String) -> Self {
        Self {
            username,
            email,
            sign_in_count: 1,
            active: true,
        }
    }

    fn deactivate(&mut self) {
        self.active = false;
    }
}

fn exercice3() {
    let mut user1 = User::new(String::from("John"), String::from(""));

    println!("Hello {}", user1.username);
    println!("Account {} status is {}", user1.username, user1.active);

    user1.deactivate(); // user1 is now inactive")
    println!("Account {} status is {}", user1.username, user1.active);
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

fn exerice2() {
    let mut myself = Person2 {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        age: Some(30),
        size: None,
    };

    //println!("{:?}", myself);

    println!("The personn age is : {:?}", myself.age);
    println!("The personn size is : {:?}", myself.size);
}

fn main() {
    // Concatenation of full name from a struct
    //exerice1();

    // Option<T> type
    //exerice2();

    // Structs and methods
    exercice3();
}
