use std::io::{self, Write};


fn two_b_two_determinant(data:&[f64;4])
-> f64 {
    let a = data[0];
    let c = data[1];
    let b = data[2];
    let d = data[3];

    a*d - b*c
}

fn remove_r_c(row: usize, col: usize, data: &Vec<f64>) 
-> Vec<f64>{
    let size = (data.len() as f64 ).sqrt() as usize; 
    assert!(row <= size);
    assert!(col <= size);
    let mut row  = row;
    if size == row { row = 0;}
    let col_end = size * col + 1;
    let col_start = col_end - size;

    let filtered_data: Vec<f64> = data
        .iter()
        .enumerate()
        .filter(|&(index,_)|{
            !((col_start..col_end).contains(&(index + 1)) ||  (index + 1) % size == row )
        })
        .map(|(_, &value)| value)
        .collect();
   
    filtered_data
}
fn determinant(data: &[f64])
-> f64 {
    0.0
}
fn main() {
    println!("Enter the size of the vectors:");
    
    let mut size = String::new();
    io::stdin().read_line(&mut size).expect("Failed to read line");
    let size: usize = size.trim().parse().expect("Invalid size");
    let size_flat = size * size;
    let mut matrix_flat = vec![0.0; size_flat];

    println!("Enter {} vectors :", size);
    
    for i in 0..size {
        print!("Vector {}:   ", i + 1);
        io::stdout().flush().expect("Failed to flush");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let numbers: Vec<f64> = input
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


    let matrix_flat2 = remove_r_c(1, 1, &matrix_flat);

    for row in 0..(size-1) {
        for col in 0..(size-1) {
            let index = col * (size-1) + row;
            print!("{} ", matrix_flat2[index]);
        }
        println!();
    }
}
