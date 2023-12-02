use std::io;

fn main() {
    println!("Enter the size of the square matrix:");
    
    let mut size = String::new();
    io::stdin().read_line(&mut size).expect("Failed to read line");
    let size: usize = size.trim().parse().expect("Invalid size");

    let mut matrix: Vec<Vec<i32>> = vec![vec![0; size]; size];

    println!("Enter {} entries separated by whitespace per {} rows separated by newline:", size, size);
    
    for i in 0..size {
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

        matrix[i] = numbers;
    }

    println!("Matrix:");

    for row in matrix {
        for num in row {
            print!("{} ", num);
        }
        println!();
    }
}
