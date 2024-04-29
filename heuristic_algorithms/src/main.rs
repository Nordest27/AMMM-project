mod reader;
mod domain;
mod greedy;
mod greedy_local_search;
mod grasp;

fn main() {
    let mut  problem = reader::read_input_file("../problems/project.0.dat");
    problem.show();
    let greedy_solution = greedy::greedy(&problem, greedy::price_heuristic);


    let greedy_solution = greedy::greedy(&problem, greedy::weight_heuristic);


    let greedy_solution = greedy::greedy(&problem, greedy::dim_side_heuristic);


    let greedy_solution = greedy::greedy(&problem, greedy::combined_heuristic);


}