use crate::domain::{Problem, Suitcase};
use crate::greedy::{greedy, price_heuristic, perimeter_heuristic, random_heuristic, one_step_deep_heuristic, greedy_loop};
use crate::local_search::{hill_climbing, simulated_annealing};
use rand;
pub fn grasp(problem: &Problem, max_iterations: i32) -> i32
{
    let mut problem= problem.clone();
    let mut best_objective = 0;
    let mut best_suitcase = problem.suitcase.clone();
    for i in 0..max_iterations {
        let mut iter_problem: Problem;
        match i%3 {
            0 => (iter_problem, _) = greedy(&problem, price_heuristic),
            1 => (iter_problem, _) = greedy(&problem, random_heuristic),
            2 => (iter_problem, _) = greedy_loop(&problem, perimeter_heuristic),
            3 => (iter_problem, _) = greedy_loop(&problem, one_step_deep_heuristic),
            _ => unreachable!()
        }
        for _ in 0..rand::random::<usize>() % iter_problem.suitcase.products.len() {
            iter_problem.suitcase.remove_product(
                &iter_problem.products[
                    rand::random::<usize>() % iter_problem.products.len()
                    ]
            );
        }
        let (suitcase, objective) = hill_climbing(&iter_problem);
        if objective > best_objective {
            best_objective = objective;
            best_suitcase = suitcase;
        }
    }
    println!("Best suitcase {}€ {}g {}bo", best_suitcase.get_price(), best_suitcase.get_weight(), best_objective);
    best_suitcase.show();
    return best_suitcase.get_price();
}