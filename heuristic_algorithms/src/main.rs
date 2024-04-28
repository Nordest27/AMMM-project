mod reader;
mod domain;
mod greedy;
mod greedy_local_search;
mod grasp;

fn main() {
    let problem = reader::read_input_file("../problems/project.0.dat");
    problem.show();
}