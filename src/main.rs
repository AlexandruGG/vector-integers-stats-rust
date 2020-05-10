use std::collections::HashMap;
use std::io;

fn main() {
    let int_vec = get_vector_input();

    println!("Mean: {:.2}", mean(&int_vec));
    println!("Median: {}", median(&int_vec));
    println!("Mode: {}", mode(&int_vec));
}

fn get_vector_input() -> Vec<i32> {
    println!("Please input list of integers in the format: 1,3,5,7");

    let mut list_input = String::new();
    io::stdin().read_line(&mut list_input).unwrap();

    list_input.retain(|c| !c.is_whitespace());

    let mut vector: Vec<i32> = list_input
        .split(',')
        .map(|x: &str| x.parse().unwrap())
        .collect();

    vector.sort();

    vector
}

fn mean(vector: &Vec<i32>) -> f64 {
    f64::from(vector.iter().sum::<i32>()) / vector.len() as f64
}

fn median(vector: &Vec<i32>) -> f64 {
    let length = vector.len();
    let middle = length / 2;

    return if length % 2 != 0 {
        f64::from(vector[middle])
    } else {
        f64::from(vector[middle - 1] + vector[middle]) / 2.0
    };
}

fn mode(vector: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for num in vector {
        *map.entry(num).or_insert(0) += 1;
    }

    let mode = map.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();

    **mode.0
}
