use crate::domain::{Problem, Product, Suitcase};

pub fn perimeter_heuristic(
    problem: &Problem,
    product: &Product,
    x: i32, y: i32,
) -> i32 {
    let mut problem: Problem = problem.clone();
    if !problem.suitcase.add_product(product, Some((x, y))) {
        return 0;
    }
    return 10*product.price - problem.suitcase.get_perimeter();
}

pub fn one_step_deep_heuristic(
    problem: &Problem,
    product: &Product,
    x: i32, y: i32,
) -> i32 {
    let mut problem: Problem = problem.clone();
    if !problem.suitcase.add_product(product, Some((x, y))) {
        return 0;
    }
    //let remaining_products = problem.remaining_possible_products();
    let non_possible_products = problem.non_possible_products();

    //let mut remaining_products_heuristic = 0;
    let mut non_possible_products_heuristic = 0;
    for iter_product in &non_possible_products {
        non_possible_products_heuristic += iter_product.price;
    }
    return product.price - non_possible_products_heuristic - problem.suitcase.get_perimeter();
}

pub fn price_heuristic(
    problem: &Problem,
    product: &Product,
    x: i32, y: i32,
) -> i32 {
    let mut problem: Problem = problem.clone();
    if !problem.suitcase.add_product(product, Some((x, y))) {
        return 0;
    }
    return problem.suitcase.get_price();
}

pub fn combined_heuristic(
    problem: &Problem,
    product: &Product,
    x: i32, y: i32,
) -> i32 {
    let mut problem: Problem = problem.clone();
    if !problem.suitcase.add_product(product, Some((x, y))) {
        return 0;
    }
    return (100*problem.suitcase.get_price())/(product.weight + product.dim_side);
}

pub fn greedy_loop(
    problem: &Problem,
    product_placement_heuristic: fn(&Problem, &Product, i32, i32) -> i32,
    alpha: f32
) -> (Problem, i32) {
    let mut problem = problem.clone();
    let mut selected_products: Vec<Product> = Vec::new();
    loop {
        let mut best_heuristic = i32::MIN;
        let mut worst_heuristic = i32::MAX;
        let remaining_products = problem.remaining_possible_products();
        let mut product_placements: Vec<(Product, i32, i32)> = Vec::new();
        let mut product_placement_heuristics: Vec<i32> = Vec::new();
        for product in &remaining_products {
            let mut possible_fits = problem.suitcase.find_all_possible_fits(product);
            for (x, y) in possible_fits {
                let calculated_h = product_placement_heuristic(
                    &problem, product, x, y
                );
                if calculated_h > best_heuristic { best_heuristic = calculated_h; }
                if calculated_h < worst_heuristic { worst_heuristic = calculated_h; }
                product_placements.push((product.clone(), x, y));
                product_placement_heuristics.push(calculated_h);
            }
        }
        if product_placements.len() == 0 { break; }
        //println!("Best: {} Worst: {} ", best_heuristic, worst_heuristic);
        let threshold = best_heuristic - (alpha*((best_heuristic - worst_heuristic) as f32)) as i32;
        let mut rcl = Vec::new();
        for i in 0..product_placements.len() {
            if product_placement_heuristics[i] >= threshold {
                rcl.push(product_placements[i].clone());
            }
        }
        // println!("Best heuristic: {}", best_heuristic);

        let random_index = rand::random::<usize>() % rcl.len();
        let (product_to_insert, x, y) = rcl[random_index].clone();
        if !problem.suitcase.add_product(&product_to_insert, Some((x, y))) {
            println!("Error: Product could not be inserted");
            let price = problem.suitcase.get_price();
            return (problem, price);
        }
        selected_products.push(product_to_insert);
        // problem.suitcase.show();
    }
    let price: i32 = problem.suitcase.get_price();
    let weight: i32 = problem.suitcase.get_weight();
    println!("Greedy Loop Solution: {}€ {}g", price, weight);
    problem.suitcase.show();
    return (problem, price);
}
