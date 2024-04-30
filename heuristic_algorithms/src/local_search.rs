use crate::domain::{Problem, Suitcase, Product};
use rand;
use crate::greedy::{one_step_deep_heuristic};

fn apply_replacement(
    suitcase: &Suitcase,
    product: &Product,
    x: i32,
    y: i32
) -> Option<Suitcase> {
    let mut suitcase: Suitcase = suitcase.clone();
    if !suitcase.replace_product(product, x, y) {
        return None;
    }
    Some(suitcase)
}

fn get_replacements(
    problem: Problem,
    suitcase: &Suitcase
) -> Vec<Suitcase> {
    let mut replacements = Vec::new();
    let suitcase_products = suitcase.products.iter().map(|(p, _, _)| p).cloned().collect::<Vec<Product>>();
    let remaining_products: Vec<Product> = problem.products.iter().filter(|product| {
        !suitcase_products.contains(&product)
    }).cloned().collect::<Vec<Product>>();
    for product in remaining_products.iter() {
        for (_, x, y) in suitcase.products.iter() {
            if let Some(new_suitcase) = apply_replacement(suitcase, product, *x, *y) {
                replacements.push(new_suitcase);
            }
        }
    }
    return replacements;
}

fn get_removals(
    suitcase: &Suitcase
) -> Vec<Suitcase> {
    let mut removals = Vec::new();
    for (product, x, y) in suitcase.products.iter() {
        let mut new_suitcase = suitcase.clone();
        new_suitcase.remove_product(product);
        removals.push(new_suitcase);
    }
    return removals;
}

fn get_moves(
    problem: Problem,
    suitcase: &Suitcase
) -> Vec<Suitcase> {
    let mut moves = Vec::new();
    for (product, _, _) in &suitcase.products{
        for x in 0..problem.suitcase.dim_x {
            for y in 0..problem.suitcase.dim_y {
                let mut new_suitcase = suitcase.clone();
                if new_suitcase.move_product(product, x, y) {
                    moves.push(new_suitcase);
                }
            }
        }
    }
    return moves;
}

fn get_additions(
    suitcase: &Suitcase,
    remaining_products: &Vec<Product>
) -> Vec<Suitcase> {

    let mut additions = Vec::new();
    for product in remaining_products.iter() {
        let fits = suitcase.find_all_possible_fits(product);
        for (x, y) in fits {
            let mut new_suitcase = suitcase.clone();
            if new_suitcase.add_product(product, Some((x, y))) {
                additions.push(new_suitcase);
            }
        }
    }
    return additions;
}



fn get_random_neighbour(
    problem: Problem,
    suitcase: &Suitcase
) -> Option<(Suitcase, i32)> {
    let suitcase_products = suitcase.products.iter().map(|(p, _, _)| p).cloned().collect::<Vec<Product>>();
    let remaining_products: Vec<Product> = problem.products.iter().filter(|product| {
        !suitcase_products.contains(&product)
    }).cloned().collect::<Vec<Product>>();
    let mut neighbours = Vec::new();
    neighbours.append(&mut get_replacements(problem.clone(), suitcase));
    neighbours.append(&mut get_removals(suitcase));
    neighbours.append(&mut get_moves(problem.clone(), suitcase));
    neighbours.append(&mut get_additions(suitcase, &remaining_products));
    if neighbours.len() == 0 {
        return None;
    }
    let random_index = rand::random::<usize>() % neighbours.len();
    let chosen_suitcase: Suitcase = neighbours[random_index].clone();
    let objective = chosen_suitcase.get_price();
    return Some((chosen_suitcase, objective));
}

pub fn simulated_annealing(problem: &Problem, temperature: f64, iters: i32) -> i32 {
    let mut suitcase: Suitcase = problem.suitcase.clone();
    let mut objective = suitcase.get_price();
    let mut temperature = temperature;
    let mut best_objective = objective;
    let mut best_suitcase = suitcase.clone();

    for _ in 0..iters {
        let (new_suitcase, new_objective) =
            match get_random_neighbour(problem.clone(), &suitcase) {
            Some(neighbour) => neighbour,
            None => break,
        };
        let delta = (new_objective - objective) as f64;
        // if delta < 0.0 {println!("Delta: {}, Temperature: {}, probability: {}", delta, temperature, (delta / temperature).exp())};
        if delta > 0.0 || rand::random::<f64>() < (delta / temperature).exp() {
            suitcase = new_suitcase;
            objective = new_objective;
        }

        if objective > best_objective {
            best_objective = objective;
            best_suitcase = suitcase.clone();
        }

        temperature *= 0.999; // Temperature reduction
        // suitcase.show();
    }

    println!("Simulated Annealing Solution: {}€ {}g", best_suitcase.get_price(), best_suitcase.get_weight());
    // best_suitcase.show();

    best_suitcase.get_price()
}

fn get_best_neighbour(
    problem: Problem,
    suitcase: &Suitcase
) -> Option<(Suitcase, i32)> {
    let suitcase_products = suitcase.products.iter().map(|(p, _, _)| p).cloned().collect::<Vec<Product>>();
    let remaining_products: Vec<Product> = problem.products.iter().filter(|product| {
        !suitcase_products.contains(&product)
    }).cloned().collect::<Vec<Product>>();
    let mut best_suitcase = suitcase.clone();
    let mut best_objective = suitcase.get_price();
    let mut neighbours = Vec::new();
    let mut found = false;
    neighbours.append(&mut get_replacements(problem.clone(), suitcase));
    neighbours.append(&mut get_removals(suitcase));
    neighbours.append(&mut get_moves(problem.clone(), suitcase));
    neighbours.append(&mut get_additions(suitcase, &remaining_products));
    for neighbour in neighbours {
        let objective = neighbour.get_price();
        if objective > best_objective {
            best_objective = objective;
            best_suitcase = neighbour;
            found = true;
        }
    }
    if !found {
        return None;
    }
    return Some((best_suitcase, best_objective));
}

pub fn hill_climbing(problem: &Problem) -> i32 {
    let mut suitcase: Suitcase = problem.suitcase.clone();
    let mut objective = suitcase.get_price();

    loop {
        let (new_suitcase, new_objective) =
            match get_best_neighbour(problem.clone(), &suitcase) {
            Some(neighbour) => neighbour,
            None => break,
        };

        if new_objective > objective {
            suitcase = new_suitcase;
            objective = new_objective;
        }
        else {
            break;
        }
        // suitcase.show();
    }

    // println!("Hill Climbing Solution: {}€ {}g", suitcase.get_price(), suitcase.get_weight());
    // suitcase.show();

    return suitcase.get_price();
}