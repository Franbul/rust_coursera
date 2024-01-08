use std::collections::HashMap;

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

fn count_vowels(sentence: &str) -> usize {
    let mut count = 0;
    for c in sentence.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => count += 1,
            _ => continue,
        }
    }
    count
}

fn get_longest_word(sentence: &str) -> &str {
    let mut longest_word = "";
    for word in sentence.split_whitespace() {
        if word.len() > longest_word.len() {
            longest_word = word;
        }
    }
    longest_word
}

fn exercice6() {
    let sentence = "the quick brown fox jumps over the lazy dog".to_string();
    // Use slicing to get the first three characters of the sentence
    println!("{}", &sentence[0..=4]);

    // concatenate using format!
    let description = format!("Title: Quick story\n{}", sentence);
    println!("{}", description);

    // iterate over the characters in the sentence
    for c in sentence.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => println!("Got a vowel!"),
            _ => continue,
        }
    }

    // Split and collect into a vector
    //let words: Vec<&str> = sentence.split_whitespace().collect();
    let words = sentence.split(' ').collect::<Vec<&str>>();
    println!("{:?}", words);

    let reversed = sentence.chars().rev().collect::<String>();
    println!("{}", reversed);

    let sentence_count = count_vowels(&sentence);
    println!("The sentence contains {} vowels", sentence_count);

    let longest_word = get_longest_word(&sentence);
    println!("The longest word is '{}'", longest_word);
}

fn get_item(index: usize) {
    //let index = 3; // this looks like an unsigned integer, but it's actually a usize
    let vec = vec![1, 2, 3, 4, 5];

    // Retrieve a value at a specific index
    let value = vec.get(index).unwrap();

    // print the value
    println!("The value at index {} is {:?}", index, value);
}

// create a function to sum items with iterators
fn sum_items_with_iterator(vec: &Vec<i32>) -> i32 {
    vec.iter().sum()
}

fn vector_sum(vec: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for value in vec {
        sum += value;
    }
    sum
}

fn exercice8() {
    let vec = vec![1, 2, 3, 4, 5];
    get_item(3);

    // Retrieve a value at a specific index
    let third_value = vec[2];
    println!("The third value in the vector is: {}", third_value);

    // Retrieve the last value
    let last_value = vec.last().unwrap();
    println!("The last value in the vector is: {}", last_value);

    // Retrieve the first value using pattern matching
    match vec.first() {
        Some(first_value) => println!("The first value in the vector is: {}", first_value),
        None => println!("The vector is empty!"),
    }

    let vector_sum = vector_sum(&vec);
    println!("The sum of the vector is: {}", vector_sum);

    let vector_sum_iter = sum_items_with_iterator(&vec);
    println!("The sum of the vector with iter is: {}", vector_sum_iter);
}

fn add_item_begin_end_vectors(v: &mut Vec<i32>, item: i32) {
    v.insert(0, item);
    v.push(item);
}

fn extend_vector(v: &mut Vec<i32>, other: &Vec<i32>) {
    v.extend(other);
}

fn exercice9() {
    let mut v = vec![1, 2, 3];
    v.push(4);
    println!("{:?}", v); // Output: [1, 2, 3, 4]

    // extend adds each element of the given slice to the vector
    let more_numbers = vec![5, 6];
    v.extend(more_numbers);
    println!("{:?}", v);

    // append adds the given vector to the vector, requires the vector to be mutable
    let mut other_numbers = vec![7, 8];
    v.append(&mut other_numbers);
    println!("{:?}", v);

    // insert items at a given index
    v.insert(0, 0);
    println!("{:?}", v); // Output: [0, 1, 2, 3, 4, 5, 6, 7, 8]

    // Add item at the beginning and end of the vector
    add_item_begin_end_vectors(&mut v, 12);
    println!("{:?}", v); // Output: [12, 0, 1, 2, 3, 4, 5, 6, 7, 8, 12]

    // Extend vector with an other vector
    let other = vec![10, 11];
    extend_vector(&mut v, &other);
    println!("{:?}", v); // Output: [10, 11, 12, 0, 1, 2, 3, 4, 5, 6, 7, 8, 12, 10, 11]
}

fn exercice10() {
    let mut reviews: HashMap<String, String> = HashMap::new();

    reviews.insert(
        String::from("Ancient Roman History"),
        String::from("Very accurate."),
    );
    reviews.insert(
        String::from("Cooking with Rhubarb"),
        String::from("Sweet recipes."),
    );
    reviews.insert(
        String::from("Programming in Rust"),
        String::from("Great examples."),
    );

    // Look for a specific review
    let book: &str = "Programming in Rust";
    println!("\nReview for \'{}\': {:?}", book, reviews.get(book));

    // Remove book review
    let obsolete: &str = "Ancient Roman History";
    println!("\n'{}\' removed.", obsolete);
    reviews.remove(obsolete);

    // Confirm book review removed
    println!("\nReview for \'{}\': {:?}", obsolete, reviews.get(obsolete));
}

#[derive(Debug)]
enum WineRegions {
    Bordeaux,
    Burgundy,
    Champagne,
    Tuscany,
    Rioja,
    NapaValley,
    Bourgogne,
}

struct Wine {
    name: String,
    region: WineRegions, // wine regions used as a type
}

fn supported_regions(w: &WineRegions) {
    match w {
        WineRegions::Rioja => println!("Rioja is supported!"),
        WineRegions::Bourgogne => println!("Bourgogne is the best wine!"),
        _ => println!("{:?} is not supported!", w),
    }
}

fn modify_wineregion(wine: &mut Wine, region: WineRegions) {
    wine.region = region;
}

fn exercice11() {
    let wine1 = Wine {
        name: String::from("Chateau Margaux"),
        region: WineRegions::Bordeaux,
    };

    let wine2 = Wine {
        name: String::from("Barolo"),
        region: WineRegions::Tuscany,
    };

    let mut wine3: Wine = Wine {
        name: String::from("epineuil"),
        region: WineRegions::Bourgogne,
    };

    println!("Wine 1: {} from {:?}", wine1.name, wine1.region);
    println!("Wine 2: {} from {:?}", wine2.name, wine2.region);
    println!("Wine 3: {} from {:?}", wine3.name, wine3.region);

    supported_regions(&wine1.region);
    supported_regions(&wine3.region);
    supported_regions(&WineRegions::Rioja);

    modify_wineregion(&mut wine3, WineRegions::Bordeaux);
    println!("Wine 3: {} from {:?}", wine3.name, wine3.region);
}

fn exercice12() {
    enum FileSize {
        Bytes(u128),
        Kilobytes(u128),
        Megabytes(u128),
        Gigabytes(u128),
        Terabyte(u128),
    }

    fn format_size(size: u128) -> String {
        let filesize = match size {
            0..=999 => FileSize::Bytes(size),
            1000..=999_999 => FileSize::Kilobytes(size / 1000),
            1_000_000..=999_999_999 => FileSize::Megabytes(size / 1_000_000),
            1_000_000_000..=999_999_999_999 => FileSize::Gigabytes(size / 1_000_000_000),
            1_000_000_000_000..=999_999_999_999_999 => FileSize::Terabyte(size / 1_000_000_000_000),
            _ => FileSize::Terabyte(size / 1_000_000_000_000),
        };

        match filesize {
            FileSize::Bytes(bytes) => format!("{} bytes", bytes),
            FileSize::Kilobytes(kb) => format!("{:.2} KB", kb as f64 / 1000.0),
            FileSize::Megabytes(mb) => format!("{:.2} MB", mb as f64 / 1000.0),
            FileSize::Gigabytes(gb) => format!("{:.2} GB", gb as f64 / 1000.0),
            FileSize::Terabyte(tb) => format!("{:.2} TB", tb as f64 / 1000.0),
        }
    }

    fn get_file_size_in_largest_possible_unit(size: u128) {
        let result = format_size(size); // Convert u64 to u128
        println!("{}", result);
    }

    get_file_size_in_largest_possible_unit(6880389113739);
}

fn exercice13() {
    fn divide(x: i32, y: i32) -> Option<i32> {
        if y == 0 {
            None // This is valid because it is the other variant of Option
        } else {
            Some(x / y) // Creates the Option<i32> value. Some() creates a new instance of Option
        }
    }

    let a = 10;
    let b = 2;

    let result = divide(a, b);

    match result {
        Some(x) => println!("Result: {}", x),
        None => println!("Error: division by zero"),
    }
}

fn exercice14() {
    #[derive(Debug)]
    enum Shape {
        Circle(f64),
        Square(f64),
        Triangle(f64, f64),
    }

    let shapes = vec![
        Shape::Circle(5.0),
        Shape::Square(3.0),
        Shape::Triangle(10.0, 4.0),
    ];

    fn get_larger_shape(shapes: &[Shape]) -> Option<&Shape> {
        let mut larger_shape: Option<&Shape> = None;
        let mut larger_area = 0.0;
        for shape in shapes {
            let area = match shape {
                Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
                Shape::Square(length) => length * length,
                Shape::Triangle(base, height) => 0.5 * base * height,
            };

            if area > larger_area {
                larger_area = area;
                larger_shape = Some(shape);
            }
        }
        larger_shape
    }

    let total_area: f64 = shapes
        .iter()
        .map(|shape| match shape {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(length) => length * length,
            Shape::Triangle(base, height) => 0.5 * base * height,
        })
        .sum();

    println!("Total area: {} sq. units", total_area);
    let largest_shape = get_larger_shape(&shapes);
    println!("The largest shape is {:?}", largest_shape);
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
    //exercic5();

    // String manipulation
    //exercice6();

    // Il n'y a pas de 7

    // Vector items sum + iterators
    //exercice8()

    // Vector methods
    //exercice9();

    // HashMap
    //exercice10();

    // Enum
    //exercice11();

    // Enum with match
    //exercice12();

    // Enum with match
    // exercice13();

    // iterators with Enum and match
    exercice14()
}
