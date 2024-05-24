use crate::domain::{Problem, Suitcase};
use crate::greedy::{
    combined_heuristic, price_heuristic,
    perimeter_heuristic, one_step_deep_heuristic,
    greedy_loop
};
use crate::local_search::{hill_climbing};
use rand;
pub fn grasp(problem: &Problem, alpha: f32) -> i32
{
    let ini_time = std::time::Instant::now();
    let mut problem= problem.clone();
    let mut best_objective = 0;
    let mut best_suitcase = problem.suitcase.clone();
    let mut last_improvement = 0;
    let mut i= 0;
    loop {
        if i - last_improvement > 25 {
            println!("No improvement in the last 25 iterations");
            break;
        }
        // let (iter_problem, _) = greedy_loop(&problem, price_heuristic, alpha);
        //let (iter_problem, _) = greedy_loop(&problem, combined_heuristic, alpha);
        // let (iter_problem, _) = greedy_loop(&problem, perimeter_heuristic, alpha);
        let (iter_problem, objective) = greedy_loop(&problem, one_step_deep_heuristic, alpha);
        let (suitcase, objective) = hill_climbing(&iter_problem);
        let suitcase = iter_problem.suitcase.clone();
        if objective > best_objective {
            println!("Improvement in iteration {} -> {}€ {}g",
                     i, suitcase.get_price(), suitcase.get_weight());
            best_objective = objective;
            suitcase.show();
            best_suitcase = suitcase.clone();
            last_improvement = i;
        }
        i += 1;
    }
    println!();
    println!("GRASP finished");
    println!("Elapsed time: {}s", ini_time.elapsed().as_secs());
    println!("Best suitcase {}€ {}g",
             best_suitcase.get_price(), best_suitcase.get_weight());
    best_suitcase.show();
    best_suitcase.show_collision_jump_matrix();
    problem.suitcase = best_suitcase;
    println!("Unused products:");
    for product in problem.non_possible_products().iter() {
        println!("{} {}mm {}€ {}g",
                 product.name, product.dim_side, product.price, product.weight);
    }
    println!();
    return problem.suitcase.get_price();
}