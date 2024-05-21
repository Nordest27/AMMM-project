use crate::domain::{Problem, Suitcase};
use crate::greedy::{
    combined_heuristic, price_heuristic,
    perimeter_heuristic, one_step_deep_heuristic,
    greedy_loop
};
use crate::local_search::{hill_climbing};
use rand;
pub fn grasp(problem: &Problem, max_iterations: i32, alpha: f32) -> i32
{
    let mut problem= problem.clone();
    let mut best_objective = 0;
    let mut best_suitcase = problem.suitcase.clone();
    for _ in 0..max_iterations {
        //let (iter_problem, _) = greedy_loop(&problem, price_heuristic, alpha);
        //let (iter_problem, _) = greedy_loop(&problem, combined_heuristic, alpha);
        // let (iter_problem, _) = greedy_loop(&problem, perimeter_heuristic, alpha);
        let (iter_problem, _) = greedy_loop(&problem, one_step_deep_heuristic, alpha);
        let (suitcase, objective) = hill_climbing(&iter_problem);
        if objective > best_objective {
            best_objective = objective;
            suitcase.show();
            best_suitcase = suitcase.clone();
        }
    }
    println!("Best suitcase {}€ {}g",
             best_suitcase.get_price(), best_suitcase.get_weight());
    best_suitcase.show();
    problem.suitcase = best_suitcase;
    println!("Unused products:");
    for product in problem.non_possible_products().iter() {
        println!("{} {}mm {}€ {}g",
                 product.name, product.dim_side, product.price, product.weight);
    }
    return problem.suitcase.get_price();
}