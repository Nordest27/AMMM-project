use crate::domain::{Problem, Product, Suitcase};

pub fn random_heuristic(product: &Product) -> i32 {
    return rand::random::<i32>();
}

pub fn price_heuristic(product: &Product) -> i32 {
    return product.price;
}

pub fn weight_heuristic(product: &Product) -> i32 {
    return (10*product.price)/product.weight;
}

pub fn dim_side_heuristic(product: &Product) -> i32 {
    return (10*product.price)/product.dim_side;
}

pub fn combined_heuristic(product: &Product) -> i32 {
    return (100*product.price)/(product.dim_side + product.weight);
}

pub fn greedy(
    problem: &Problem,
    heuristic: fn(&Product) -> i32
) -> (Problem, i32) {
    let mut problem = problem.clone();
    let mut products = problem.products.iter().cloned().collect::<Vec<Product>>();
    products.sort_by(|a, b| heuristic(b).cmp(&heuristic(a)));
    let mut current_weight = 0;
    let mut current_price = 0;
    let mut selected_products = Vec::new();
    for product in products {
        if current_weight + product.weight <= problem.suitcase.max_weight &&
            problem.suitcase.add_product(&product, None) {
            current_weight += product.weight;
            current_price += product.price;
            selected_products.push(product);
        }
    }
    // println!("Greedy Solution: {}€", current_price);
    // problem.suitcase.show();
    return (problem, current_price);
}

pub fn perimeter_heuristic(
    problem: &Problem,
    product: &Product,
    x: i32, y: i32,
) -> i32 {
    let mut problem: Problem = problem.clone();
    if !problem.suitcase.add_product(product, Some((x, y))) {
        return 0;
    }
    return product.price - problem.suitcase.get_perimeter();
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
    let remaining_products = problem.remaining_possible_products();

    let mut remaining_products_heuristic = 0;
    for iter_product in &remaining_products {
        remaining_products_heuristic += iter_product.price;
    }
    return product.price + remaining_products_heuristic;
}

pub fn greedy_loop(
    problem: &Problem,
    product_placement_heuristic: fn(&Problem, &Product, i32, i32) -> i32
) -> (Problem, i32) {
    let mut problem = problem.clone();
    let mut selected_products: Vec<Product> = Vec::new();
    loop {
        let mut best_heuristic = i32::MIN;
        let remaining_products = problem.remaining_possible_products();
        let mut best_product_placements: Vec<(Product, i32, i32)> = Vec::new();
        for product in &remaining_products {
            let mut possible_fits = problem.suitcase.find_all_possible_fits(product);
            for (x, y) in possible_fits {
                let calculated_h = product_placement_heuristic(
                    &problem, product, x, y
                );
                if calculated_h > best_heuristic {
                    best_heuristic = calculated_h;
                    best_product_placements = vec!((product.clone(), x, y));
                }
                else if calculated_h == best_heuristic {
                    best_product_placements.push((product.clone(), x, y));
                }
            }
        }

        // println!("Best heuristic: {}", best_heuristic);
        if best_product_placements.len() == 0 {
            break;
        }
        let random_index = rand::random::<usize>() % best_product_placements.len();
        let (product_to_insert, x, y) = best_product_placements[random_index].clone();
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
    // println!("Greedy Loop Solution: {}€ {}g", price, weight);
    // problem.suitcase.show();
    return (problem, price);
}
