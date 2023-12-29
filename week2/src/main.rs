fn main() {
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
        println!("teenager")
    } else {
        println!("child")
    }
}
