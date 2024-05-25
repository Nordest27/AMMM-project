use crate::domain::{Problem, Suitcase, Product};
use rand;
use crate::greedy::{one_step_deep_heuristic};

fn objective(suitcase: &Suitcase) -> i32{
    //let empty_sections = suitcase.get_empty_sections_with_size();
    let dims = suitcase.dim_x*suitcase.dim_y;
    return 10*dims*suitcase.get_price()
        -dims*suitcase.get_perimeter()
        -dims*suitcase.get_n_corners()
        //-(empty_sections.iter().map(|(_, size)|dims/size).sum::<i32>())
    ;
}

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
    suitcase: &Suitcase,
    remaining_products: &Vec<Product>
) -> Vec<Suitcase> {
    let mut replacements = Vec::new();
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
    for (product, _, _) in suitcase.products.iter() {
        let mut new_suitcase = suitcase.clone();
        new_suitcase.remove_product(product);
        removals.push(new_suitcase);
    }
    return removals;
}

fn get_moves(
    suitcase: &Suitcase
) -> Vec<Suitcase> {
    let mut moves = Vec::new();
    for (product, _, _) in &suitcase.products{
        let mut new_suitcase = suitcase.clone();
        new_suitcase.remove_product(product);
        let fits = new_suitcase.find_all_possible_corner_fits(product);
        for fit in fits {
            let mut suitcase_to_add = new_suitcase.clone();
            suitcase_to_add.add_product(product, Some(fit));
            moves.push(suitcase_to_add);
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
        let fits = suitcase.find_all_possible_corner_fits(product);
        for (x, y) in fits {
            let mut new_suitcase = suitcase.clone();
            if new_suitcase.add_product(product, Some((x, y))) {
                additions.push(new_suitcase);
            }
        }
    }
    return additions;
}

fn get_all_suitcase_neighbours(
    problem: &Problem,
) -> Vec<Suitcase>
{
    let remaining_products: Vec<Product> = problem.remaining_possible_products();
    let mut neighbours: Vec<Suitcase> = Vec::new();
    neighbours.append(&mut get_replacements(&problem.suitcase, &remaining_products));
    neighbours.append(&mut get_removals(&problem.suitcase));
    neighbours.append(&mut get_moves(&problem.suitcase));
    neighbours.append(&mut get_additions(&problem.suitcase, &remaining_products));
    return neighbours;
}

fn get_best_neighbour(
    problem: &Problem,
) -> Option<(Suitcase, i32)> {
    let mut best_suitcase = problem.suitcase.clone();
    let mut best_objective = objective(&best_suitcase);
    let mut neighbours = get_all_suitcase_neighbours(problem);
    let mut found = false;
    for neighbour in neighbours {
        let objective = objective(&neighbour);
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

pub fn hill_climbing(problem: &Problem) -> (Suitcase, i32) {
    let mut problem = problem.clone();
    let mut objective = objective(&problem.suitcase);
    loop {
        let (new_suitcase, new_objective) =
            match get_best_neighbour(&problem) {
            Some(neighbour) => neighbour,
            None => break,
        };

        if new_objective > objective {
            problem.suitcase = new_suitcase;
            objective = new_objective;
        }
        else { break; }
        // suitcase.show();
    }
    println!("Hill Climbing Solution: {}€ {}g", problem.suitcase.get_price(), problem.suitcase.get_weight());
    // problem.suitcase.show();
    let price = problem.suitcase.get_price();
    return (problem.suitcase, price);
}