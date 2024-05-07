use std::io::{self, Write};

fn main() {
    let array: Vec<i32> = vec![3, 2, 5, 8, 1, 4, 2, 7];
    let mut value = String::new();

    print!("Enter a value: ");
    io::stdout()
            .flush()
            .unwrap();

    io::stdin()
            .read_line(&mut value)
            .expect("Error");

    let value: i32 = value.trim().parse().expect("Only usize integers");

    let i = find_index(&array, value);

    match i {
        Some(index) => println!("The element index is: {}", index),
        None => println!("The element not found"),
    }
}

fn find_index(arr: &[i32], v: i32) -> Option<usize> {
    for (index, &element) in arr.iter().enumerate() {
        if v == element {
            return Some(index);
        }
    }
    None
}
