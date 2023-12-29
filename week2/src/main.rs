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
    exercice7();
}
