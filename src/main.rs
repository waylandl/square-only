use std::io::{self, Write};

fn main() {
    println!("Enter the size of the vectors:");
    
    let mut size = String::new();
    io::stdin().read_line(&mut size).expect("Failed to read line");
    let size: usize = size.trim().parse().expect("Invalid size");
    let size_flat = size * size;
    let mut matrix_flat = vec![0; size_flat];

    println!("Enter {} vectors :", size);
    
    for i in 0..size {
        print!("Vector {}:   ", i + 1);
        io::stdout().flush().expect("Failed to flush");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let numbers: Vec<i32> = input
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid number"))
            .collect();

        if numbers.len() != size {
            eprintln!("Error: You must enter exactly {} numbers.", size);
            return;
        }

        let start_index: usize = i*size;
        let end_index: usize = start_index + size;
        let numbers_iter = numbers.into_iter();
        matrix_flat.splice(start_index..end_index,numbers_iter);

    }



    for row in 0..size {
        for col in 0..size {
            let index = col * size + row;
            print!("{} ", matrix_flat[index]);
        }
        println!();
    }
}
