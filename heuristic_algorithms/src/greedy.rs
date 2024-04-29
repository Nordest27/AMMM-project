use crate::domain;

pub fn price_heuristic(product: &domain::Product) -> i32 {
    return product.price;
}

pub fn weight_heuristic(product: &domain::Product) -> i32 {
    return product.price/product.weight;
}

pub fn dim_side_heuristic(product: &domain::Product) -> i32 {
    return product.price/product.dim_side;
}

pub fn combined_heuristic(product: &domain::Product) -> i32 {
    return product.price/(product.dim_side * product.dim_side + product.weight);
}

pub fn greedy(
    problem: &domain::Problem, heuristic: fn(&domain::Product) -> i32
) -> Vec<domain::Product> {
    let mut problem = problem.clone();
    let mut products = problem.products.iter().cloned().collect::<Vec<domain::Product>>();
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
    println!("Greedy Solution: {}€", current_price);
    problem.suitcase.show();
    return selected_products;
}

