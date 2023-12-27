
// function to find duplicate numbers from a set of numbers
fn find_duplicate_numbers(numbers: Vec<i32>) -> Vec<i32> {
    // create a vector to store duplicate numbers
    let mut duplicate_numbers: Vec<i32> = Vec::new();

    // iterate over numbers
    for i in 0..numbers.len() {
        // iterate over numbers
        for j in i+1..numbers.len() {
            // check if numbers are equal
            if numbers[i] == numbers[j] {
                // push duplicate number to vector
                duplicate_numbers.push(numbers[i]);
            }
        }
    }

    // return duplicate numbers
    duplicate_numbers
}

// create main function
fn main() {
    // call find_duplicate_numbers function with sample data and print duplicate numbers
    println!("Duplicate numbers: {:?}", find_duplicate_numbers(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1]));
}