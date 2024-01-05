use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};
use std::str::FromStr;
use std::{io, vec};

fn exercice1() {
    // Manage proceed
    let proceed: bool = true;
    if proceed {
        println!("Proceeding");
    } else {
        println!("Not proceeding");
    }

    // Manage height
    let height: i32 = 183;
    if height > 185 {
        println!("Tall");
    } else if height > 170 {
        println!("Average");
    } else {
        println!("Short");
    }

    // Manage Age
    let age: i32 = 17;
    if age > 18 {
        println!("adult")
    } else if age > 12 {
        println!("teenager");
    } else {
        println!("child");
    }
}

fn exercice2() {
    // Manage height
    let mut height: i32 = 183;

    height = height - 10;

    let result = if height > 185 {
        "Tall"
    } else if height > 170 {
        "Average"
    } else {
        "Short"
    };
    println!("Result: {}", result);

    let mut health = if height < 100 { "good" } else { "bad" };
    println!("Health: {}", health);
}

fn exercice3() {
    let mut x = 0;
    loop {
        x += 1;
        if x > 10 {
            break;
        }
        println!("x = {}", x);
    }
}

fn exercice4() {
    let maybe_number = Some(42);

    if let Some(number) = maybe_number {
        println!("number = {}", number);
    } else {
        println!("No number");
    }
}

fn exercice5() {
    let maybe_number: Option<Option<()>> = Some(None);

    // verify if maybe_number contains a value
    if let Some(number) = maybe_number {
        println!("number = {:?}", number);
    } else {
        println!("No number");
    }
}

fn exercice6() {
    let mut input = String::new();
    while input.trim() != "quit" {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        println!("You typed: {}", input.trim());
    }
}

fn exercice7() {
    for i in 0..10 {
        println!("i = {}", i);
    }

    for i in 0..=10 {
        println!("i = {}", i);
    }

    for i in (0..10).rev() {
        println!("i = {}", i);
    }

    let numbers = vec![1, 2, 3, 4, 5];
    for i in numbers {
        println!("i = {}", i);
    }
}

fn mean(numbers: &[i32]) -> f32 {
    let mut result = 0;
    for number in numbers {
        result += number;
    }
    (result / numbers.len() as i32) as f32
}

fn sum(numbers: &[i32]) -> i32 {
    let mut result = 0;
    for number in numbers {
        result += number;
    }
    result
}

fn exercice8_1() {
    // Prompt for the number of elements
    println!("Enter the number of elements:");
    let mut num_elements = String::new();
    io::stdin()
        .read_line(&mut num_elements)
        .expect("Failed to read line");

    let num_elements: usize = match usize::from_str(num_elements.trim()) {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            return;
        }
    };

    println!("Number of values:{}", num_elements);

    // Prompt for the elements
    let mut numbers = Vec::new(); // Creating a new vector

    for i in 0..num_elements {
        println!("Enter the value for element {}", i);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: i32 = match i32::from_str(input.trim()) {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                return;
            }
        };
        numbers.push(input);
        println!("Numbers: {:?}", numbers);

        let result = mean(&numbers);
        println!("Mean: {}", result);

        let result = sum(&numbers);
        println!("Sum: {}", result);
    }
}

fn exercice8_2() {
    let numbers = vec![1, 2, 3, 4, 5];
    let result = mean(&numbers);
    println!("Mean: {}", result);

    let result = sum(&numbers);
    println!("Sum: {}", result);
}

fn own_vec(mut vector: Vec<i32>) {
    vector.push(10);
    println!("{:?}", vector);
}

fn borrow_vec(vector: &mut Vec<i32>) {
    vector.push(10);
}

fn own_integer(x: i32) {
    x + 1;
}

fn own_string(s: String) {
    println!("{}", s);
}

fn borrow_string(s: &mut String) {
    s.push_str(" en plus");
}

// Borrowing is the mechanism by which Rust allows you to lend ownership of a variable to a function
// or another part of your program without actually transferring ownership of the variable.
// When you borrow a variable, you're essentially saying
// "I want to use this variable for a little while, but I promise I won't modify it."
fn exercice9() {
    let mut my_vec: Vec<i32> = vec![1, 2, 3, 4, 5];
    let my_int = 10;
    let mut my_string = String::from("Hello, world!");

    // this compiles no problem!
    own_integer(my_int);
    println!("{}", my_int);

    //own_string(my_string); // take ownership of my_string
    //this is using my_string which has also moved and is invalid
    //println!("{:?}", my_string); // this will not compile!

    // this compiles no problem!
    borrow_string(&mut my_string);
    println!("{:?}", my_string);

    //own_vec(my_vec);
    // but this is using my_vec which was borrowed (moved) and yet is now invalid
    //println!("{:?}", my_vec); // this will not compile!

    // this compiles no problem!
    borrow_vec(&mut my_vec);
    println!("{:?}", my_vec); // this will compile!
                              // but this is using my_vec which was borrowed (moved) and yet is now invalid
                              //println!("{:?}", my_vec); // this will not compile!

    // this compiles no problem!
    //borrow_vec(&mut my_vec);
    //println!("{:?}", my_vec); // this will compile!
}

fn exercice10_1() {
    let file = File::open("non_existent_file.txt");
    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                panic!("File not found: {}", error)
            }
            std::io::ErrorKind::PermissionDenied => {
                panic!("Permission error: {}", error)
            }
            _ => {
                panic!("Error opening file: {}", error)
            }
        },
    };

    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                panic!("Error reading line: {}", error)
            }
        }
    }
}

fn write_to_file(path: &str, content: &str) -> Result<(), Error> {
    let mut file = match File::create(path) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    match file.write_all(content.as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

fn exercice10_2() {
    // Write a file with a valid path
    match write_to_file("test.txt", "Hello, Rust!") {
        Ok(_) => println!("Write operation successful."),
        Err(e) => println!("Error occurred: {}", e),
    }

    // Test with an invalid path
    match write_to_file("/invalid/path/test.txt", "Hello again!") {
        Ok(_) => println!("Write operation successful."),
        Err(e) => println!("Error occurred: {}", e),
    }
}

fn exercice11(file_path: String) {
    let file = File::open(file_path);
    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                panic!("File not found: {}", error)
            }
            _ => {
                panic!("Error opening file: {}", error)
            }
        },
    };

    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                panic!("Error reading line: {}", error)
            }
        }
    }
}

fn main() {
    // if conditions statements

    //exercice1();

    //exercice2();

    // loops statements

    //exercice3();

    //exercice4();

    // Loop
    //exercice5();

    // While
    //exercice6();

    // For
    //exercice7();

    // Lab : Using arguments in functions
    //exercice8_1();
    //exercice8_2();

    // Using the borrowing concept
    // exercice9();

    // Basic error handling with match
    //exercice10_1();
    //exercice10_2();

    // Create a File Reader Application
    let args: Vec<String> = env::args().collect();

    // The first argument is the path that was used to call the program.
    println!("My path is {}", args[0]);
    println!("My file name is {}", args[1]);
    let mut file_path = String::new(); // Declare the file_path variable
    file_path.push_str(&args[0]); // Append the first argument
    file_path.push_str("/"); // Append the separator
    file_path.push_str(&args[1]); // Append the second argument
    println!("My file path is {}", file_path);
    exercice11(file_path);
}
