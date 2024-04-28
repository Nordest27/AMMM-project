mod reader;
mod domain;

fn main() {
    let problem = reader::read_input_file("../problems/project.0.dat");
    problem.show();
}