use crate::domain;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::io::Error;


/*
Example:
x =  3;
y =  3;
c = 10;

n =  6;

// Let's index products with letters: A, B, C, ...

//     A  B  C  D  E  F
p = [  4  2  1  2  2  1  ];
w = [  3  2  1  2  1  2  ];
s = [  2  1  1  1  1  1  ];
 */
fn read_var_assign(line: Option<Result<String, Error>>) -> i32 {
    if let Some(Ok(line)) = line {
        let mut x = line.split_whitespace().last().unwrap().chars();
        x.next_back();
        return x.collect::<String>().parse().unwrap();
    }
    return 0;
}
fn red_vec_assign(line: Option<Result<String, Error>>) -> Vec<i32> {
    if let Some(Ok(line)) = line {
        let numbers_with_whitespace: String = line.chars().filter(
            |c| c.is_numeric() || c.is_whitespace()
        ).collect();
        let numbers = numbers_with_whitespace.split_whitespace();
        return numbers.map(|x| x.parse().unwrap()).collect();
    }
    return Vec::new();
}
pub fn read_input_file(file_path: &str) -> domain::Problem {
    let file = File::open(file_path).expect("File not found");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut problem = domain::Problem {
        products: Vec::new(),
        suitcase: domain::Suitcase::new()
    };

    problem.suitcase.dim_x = read_var_assign(lines.next());
    problem.suitcase.dim_y = read_var_assign(lines.next());
    problem.suitcase.max_weight = read_var_assign(lines.next());

    for _ in 0..6 { lines.next(); }

    let prices = red_vec_assign(lines.next());
    let weights = red_vec_assign(lines.next());
    let side_dims = red_vec_assign(lines.next());
    for i in 0..side_dims.len() {
        problem.products.push(domain::Product {
            name: (65 + i) as u8 as char,
            dim_side: side_dims[i],
            weight: weights[i],
            price: prices[i],
        });
    }
    return problem;
}