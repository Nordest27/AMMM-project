use crate::domain;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::fs::write;
use std::io::Error;
use crate::domain::Problem;


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

/* Example:
OBJECTIVE: 8

B       B       B       B
B       B       B       B       D       D
B       B       B       B       D       D
B       B       B       B               E       E
                                        E       E
 */
pub fn read_output_file(file_path: &str) -> i32 {
    let file = File::open(file_path).expect("File not found");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut objective = 0;
    if let Some(Ok(line)) = lines.next() {
        let mut x = line.split_whitespace().last().unwrap().chars();
        objective = x.collect::<String>().parse().unwrap();
    }
    return objective;
}

pub fn write_problem_file(problem: &Problem) {
    let mut content = format!("x =  {};\n", problem.suitcase.dim_x);
    content.push_str(&format!("y =  {};\n", problem.suitcase.dim_y));
    content.push_str(&format!("c = {};\n", problem.suitcase.max_weight));
    content.push_str("\n");
    content.push_str(&format!("n =  {};\n\n", problem.products.len()));
    content.push_str("// Let's index products with letters: A, B, C, ...\n\n");
    content.push_str("//     ");
    for product in problem.products.iter() {
        content.push_str(&format!("{}   ", product.name));
    }
    content.push_str("\n");
    content.push_str("p = [");
    for product in problem.products.iter() {
        content.push_str(&format!("  {} ", product.price));
    }
    content.push_str(" ];\n");
    content.push_str("w = [");
    for product in problem.products.iter() {
        content.push_str(&format!("  {} ", product.weight));
    }
    content.push_str(" ];\n");
    content.push_str("s = [");
    for product in problem.products.iter() {
        content.push_str(&format!("  {} ", product.dim_side));
    }
    content.push_str(" ];\n");
    write("problem.dat", content).expect("Unable to write file");
}