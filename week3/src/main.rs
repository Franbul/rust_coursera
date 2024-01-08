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

#[derive(Debug)]
struct Person3 {
    first_name: String,
    last_name: String,
    age: u8,
    uri: String,
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

impl Person3 {
    fn new(first_name: &str, last_name: &str, age: u8) -> Self {
        Self {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            age,
            uri: String::new(),
        }
    }

    fn update_uri(&mut self, uri: &str) {
        if self.first_name.is_empty() {
            panic!("first_name is empty");
        }

        if uri.is_empty() {
            self.uri = format!("https://www.google.com/search?q={}", self.first_name);
        } else {
            self.uri = uri.to_string();
        }
    }

    fn from_email(email_adress: &str) -> Self {
        // get first part of email_adress
        let mut parts = email_adress.split('@');
        let full_name = parts.next().unwrap();

        // how to split full_name to get first_name and last_name ?
        let mut name_parts = full_name.split('.');
        let first_name = name_parts.next().unwrap();
        let last_name = name_parts.next().unwrap();

        Self::new(first_name, last_name, 0)
    }

    fn full_name(&self) -> String {
        let mut full_name = self.first_name.clone();
        full_name.push_str(" ");
        full_name.push_str(&self.last_name);
        full_name
    }
}

fn exercice4() {
    let mut john = Person3::new("John", "Doe", 25);
    println!("{:?}", john);

    let mut john_bis = Person3::from_email("john.doe@gmail.com");
    println!("{:?}", john_bis);

    john_bis.update_uri(""); // Pass the required argument to the update_uri function
    println!("{:?}", john_bis);

    // Pour convertir un String en &str
    let my_name = format!(
        "{} {}",
        john_bis.first_name.as_str(),
        john_bis.last_name.as_str(),
    );
    println!("{}", my_name);
}

fn exercic5() {
    // derive génère automatiquement les implémentations de certaines traits
    // comme Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord
    // https://doc.rust-lang.org/reference/attributes/derive.html
    #[derive(Debug)]
    struct Point(i32, i32);

    let p = Point(1, 0); // Des paranethèses sont utilisées pour créer une instance d'un tuple struct
    println!("p.x = {}", p.0);
    println!("p.y = {}", p.1);
}

fn main() {
    // Concatenation of full name from a struct
    //exerice1();

    // Option<T> type
    //exerice2();

    // Structs and methods
    //exercice3();

    // Labo : associated functions and constructors
    //exercice4();

    // Point without variable name
    exercic5();
}
