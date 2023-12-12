use std::io::{self, Write};


fn two_b_two_determinant(data:&Vec<f64>)
-> f64 {
    assert!(data.len() == 4);
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
fn determinant(data: &Vec<f64>)
-> f64 {
    let size = (data.len() as f64 ).sqrt() as usize;
    let mut cofactors:Vec<f64> = Vec::with_capacity(size);

    if size == 2 {
        two_b_two_determinant(data)
    } else {

        for i in 0..size {
            //find determinant at each element
            let mut cofactor = data[i];
            let new_data = remove_r_c(i+1, 1, &data);
            let new_size = new_data.len();
            if new_size == 4 {

                cofactor = cofactor * f64::powi(-1.0,(i+1+1) as i32) *two_b_two_determinant(&new_data);
            } else {
                cofactor = cofactor * f64::powi(-1.0, (i + 1 + 1) as i32) * determinant(&new_data);

            }
            
            cofactors.push(cofactor);
        }
        cofactors.iter().sum()
    }   

}

fn inverse_matrix(data: &Vec<f64>) 
-> Vec<f64> {
    let det = determinant(&data);
    let size = (data.len() as f64).sqrt() as usize;
    let mut cofactors:Vec<f64> = Vec::with_capacity(size);
    let mut cofactors_transpose:Vec<f64> = Vec::with_capacity(size);

    if size == 2 {
        cofactors_transpose = vec![data[3], data[1], data[2], data [0]];
    } else {
        for i in 0..size {
            for j in 0..size {
                let new_data = remove_r_c(j+1, i+1, &data);
                let new_det = determinant(&new_data);
                cofactors.push(new_det);
            }
        }


        for i in 0..size{
            for j in 0..size{
                let index = size * j + i;
                cofactors_transpose.push(cofactors[index]);
            }
        }
    }

    let inverse:Vec<f64> = cofactors_transpose.into_iter().map( |x| x / det).collect();
    
    inverse
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

    let det = determinant(&matrix_flat);
    println!("det : {}", det);
    let inverse = inverse_matrix(&matrix_flat);

    println!("inverse:");    
    for row in 0..(size) {
        for col in 0..(size) {
            let index = col * (size) + row;
            print!(" {:.2} ", inverse[index]);
       }

        println!();
    }
}
