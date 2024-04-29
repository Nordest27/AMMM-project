mod reader;
mod domain;
mod greedy;
mod greedy_local_search;
mod grasp;

fn main() {
    let mut  problem = reader::read_input_file("../problems/project.0.dat");
    /*
    problem.show();
    let heuristics = [
        greedy::price_heuristic,
        greedy::weight_heuristic,
        greedy::dim_side_heuristic,
        greedy::combined_heuristic,
    ];
    for heuristic in heuristics.iter() {
        let _ = greedy::greedy(&problem, *heuristic);
    }
    */
    let _ = greedy_local_search::greedy_local_search(&problem);
}