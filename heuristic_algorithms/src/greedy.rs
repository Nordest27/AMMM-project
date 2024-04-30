use crate::domain::{Problem, Product, Suitcase};

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

pub fn one_step_deep_heuristic(
    suitcase: &Suitcase,
    product: &Product,
    x: i32, y: i32,
    remaining_products: &Vec<Product>,
    current_weight: i32,
) -> i32 {
    let current_weight = current_weight + product.weight;
    let mut suitcase = suitcase.clone();
    if !suitcase.add_product(product, Some((x, y))) {
        return 0;
    }
    if remaining_products.len() == 0 {
        return product.price;
    }

    // let mut useful_space = 0;
    let mut remaining_products_heuristic = 0;
    for iter_product in remaining_products {
        if iter_product.weight + current_weight > suitcase.max_weight ||
            !suitcase.does_fit(iter_product) {
            continue;
        }
        remaining_products_heuristic += iter_product.price;
    }
    return product.price + remaining_products_heuristic;
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

pub fn one_step_deep_greedy(
    problem: &Problem,
) -> (Problem, i32) {
    let mut problem = problem.clone();
    let mut selected_products: Vec<Product> = Vec::new();
    let mut current_weight = 0;
    let mut current_price = 0;
    loop {
        let mut best_heuristic = i32::MIN;
        let mut best_product_placement: Option<(Product, i32, i32)> = None;
        let remaining_products = problem.products.iter().filter(|product| {
            !selected_products.contains(product) &&
                product.weight + current_weight <= problem.suitcase.max_weight
        }).cloned().collect::<Vec<Product>>();

        for product in &remaining_products {
            let mut possible_fits = problem.suitcase.find_all_possible_fits(product);
            for (x, y) in possible_fits {
                let calculated_h = one_step_deep_heuristic(
                    &problem.suitcase, product, x, y,
                    &remaining_products, current_weight,
                );
                if calculated_h > best_heuristic {
                    best_heuristic = calculated_h;
                    best_product_placement = Some((product.clone(), x, y));
                }
                else if calculated_h == best_heuristic && rand::random::<bool>() {
                    best_product_placement = Some((product.clone(), x, y));
                }
            }
        }
        // println!("Best heuristic: {}", best_heuristic);
        if best_product_placement.is_none() {
            break;
        }
        let (product_to_insert, x, y) = best_product_placement.unwrap();
        if !problem.suitcase.add_product(&product_to_insert, Some((x, y))) {
            println!("Error: Product could not be inserted");
            return (problem, current_price);
        }
        current_weight += product_to_insert.weight;
        current_price += product_to_insert.price;
        selected_products.push(product_to_insert);
        // problem.suitcase.show();
    }

    println!("One Step Deep Greedy Solution: {}€ {}g", current_price, current_weight);
    // problem.suitcase.show();
    return (problem, current_price);
}

