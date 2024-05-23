use crate::greedy::greedy_loop;
use crate::reader::write_problem_file;

mod reader;
mod domain;
mod greedy;
mod local_search;
mod grasp;
fn run_my_problems() {
    let problem = reader::read_input_file("../generated_problems/myproblem.1.dat");

    let objective = grasp::grasp(&problem, 0.1);

    println!("Objective: {}", objective);
}

fn run_all_problems() {
    let mut total_objective = 0;
    let mut total_best_objective = 0;
    for i in 0..10 {
        let problem = reader::read_input_file(
            format!("../problems/project.{}.dat", i).as_str());
        let best_objective = reader::read_output_file(
            format!("../problems/project.{}.sol", i).as_str());
        total_best_objective += best_objective;
        println!("Problem {}, Best Objective: {}", i, best_objective);
        // let (problem, objective) = greedy::greedy_loop(&problem, greedy::price_heuristic, 0.0);
        // let (problem, objective) = greedy::greedy(&problem, greedy::weight_heuristic);
        // let (problem, objective) = greedy::greedy(&problem, greedy::dim_side_heuristic);
        // let (problem, objective) = greedy::greedy(&problem, greedy::combined_heuristic);
        // let (problem, objective) = greedy::greedy_loop(&problem, greedy::one_step_deep_heuristic, 0.0);
        // let (problem, objective) = greedy::greedy_loop(&problem, greedy::perimeter_heuristic, 0.0);
        // let (problem, objective) = greedy::greedy_loop(&problem, greedy::corners_heuristic, 0.0);
        // let (suitcase, objective) = local_search::hill_climbing(&problem);

        let objective = grasp::grasp(&problem, 0.5);
        // problem.suitcase.show()
        // suitcase.show();
        total_objective += objective;
    }
    println!("Objective: {}, Best Objective Difference: {}",
             total_objective, total_best_objective - total_objective);
}

fn main() {
    let mut  problem = reader::read_input_file("../problems/project.9.dat");

    // problem.show();
    // let heuristics = [
    //     greedy::price_heuristic,
    //     greedy::weight_heuristic,
    //     greedy::dim_side_heuristic,
    //     greedy::combined_heuristic,
    // ];
    // for heuristic in heuristics.iter() {
    //     let _ = greedy::greedy(&problem, *heuristic);
    // }
    //
    // let _ = greedy_local_search::greedy_local_search(&problem);
    // run_all_problems();
    let problem = domain::generate_problem(30, 30);
    write_problem_file(&problem);
    let objective = grasp::grasp(&problem, 0.3);
    // let (problem, objective) = greedy_loop(&problem, greedy::one_step_deep_heuristic, 0.0);
    // run_my_problems();
}