use crate::domain::{Problem, Suitcase};
use crate::greedy::{greedy, price_heuristic, one_step_deep_heuristic, one_step_deep_greedy};
use crate::local_search::{hill_climbing, simulated_annealing};
use rand;
pub fn grasp(problem: &Problem, max_iterations: i32) -> i32
{
    let mut best_objective = 0;
    let mut best_suitcase = problem.suitcase.clone();
    for _ in 0..max_iterations {
        // let (problem, _) = greedy(&problem, price_heuristic);
        let (problem, _) = one_step_deep_greedy(&problem);
        let mut problem = problem.clone();
        let copy_of_suitcase = problem.suitcase.clone();
        for _ in 0..rand::random::<usize>() % problem.suitcase.products.len() {
            problem.suitcase.remove_product(&problem.products[rand::random::<usize>() % problem.products.len()]);
        }
        let objective = simulated_annealing(&problem, 0.5, 100);
        // let objective = hill_climbing(&problem);
        if objective > best_objective {
            best_objective = objective;
            best_suitcase = copy_of_suitcase;
        }
    }
    println!("Best Objective: {}", best_objective);
    best_suitcase.show();
    return best_objective;
}