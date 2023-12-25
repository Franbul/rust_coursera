// Create a function that calculate average of several numbers and return it
fn average(numbers: &[i32]) -> f64 {
    let mut sum = 0;
    for num in numbers {
        sum += num;
    }
    (sum as f64) / (numbers.len() as f64)
}   

fn inverse_matrix(matrix: [[f64; 2]; 2]) -> Option<[[f64; 2]; 2]> {
    let det = matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0];
    if det == 0.0 {
        None
    } else {
        Some([
            [matrix[1][1] / det, -matrix[0][1] / det],
            [-matrix[1][0] / det, matrix[0][0] / det],
        ])
    }
}

// Call the average function with somme numbers
fn main() {
    //let numbers = [1, 2, 3, 4, 5];
    //let result = average(&numbers);
    //println!("The average is {}", result);


    
}

// Run: rustc test.rs
// Run: ./test
// Output: The average is 3


