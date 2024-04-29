use crate::domain;

pub fn size_aware_heuristic(
    suitcase: &domain::Suitcase,
    product: &domain::Product,
    x: i32, y: i32,
    remaining_products: &Vec<domain::Product>
) -> i32 {
    let mut suitcase = suitcase.clone();
    if !suitcase.add_product(product, Some((x, y))) {
        return 0;
    }
    if remaining_products.len() == 0 {
        return product.price;
    }
    let mut useful_space = 0;
    let mut products_price_mean = 0;
    let mut products_weight_mean = 0;
    let mut products_dim_side_mean = 0;
    for iter_product in remaining_products {
        useful_space += suitcase.get_useful_space(iter_product.dim_side);
        products_price_mean += iter_product.price;
        products_weight_mean += iter_product.weight;
        products_dim_side_mean += iter_product.dim_side;
    }
    products_price_mean /= remaining_products.len() as i32;
    products_weight_mean /= remaining_products.len() as i32;
    products_dim_side_mean /= remaining_products.len() as i32;
    return (100*product.price)/(product.dim_side + product.weight) +
        0*useful_space * products_price_mean / (products_weight_mean + products_dim_side_mean);
}


pub fn greedy_local_search(
    problem: &domain::Problem
) -> Vec<domain::Product> {
    let mut problem = problem.clone();
    let mut selected_products: Vec<domain::Product> = Vec::new();
    let mut current_weight = 0;
    let mut current_price = 0;
    let mut minimum_found = false;
    while !minimum_found {
        minimum_found = true;
        let mut best_heuristic =-1;
        let mut best_product_placement: Option<(domain::Product, i32, i32)> = None;
        let remaining_products = problem.products.iter().filter(|product| {
            !selected_products.contains(product)
        }).cloned().collect::<Vec<domain::Product>>();

        for product in &remaining_products {
            let mut possible_fits = problem.suitcase.find_all_possible_fits(product);
            println!("len possible fits: {}", possible_fits.len());
            if product.weight + current_weight > problem.suitcase.max_weight {
                continue;
            }
            for (x, y) in possible_fits {
                let calculated_h = size_aware_heuristic(
                    &problem.suitcase, product, x, y, &remaining_products
                );
                if calculated_h > best_heuristic {
                    best_heuristic = calculated_h;
                    best_product_placement = Some((product.clone(), x, y));
                    minimum_found = false;
                }
            }
        }
        println!("Best heuristic: {}", best_heuristic);
        if minimum_found {
            continue;
        }
        let (product_to_insert, x, y) = best_product_placement.unwrap();
        if !problem.suitcase.add_product(&product_to_insert, Some((x, y))) {
            println!("Error: Product could not be inserted");
            return selected_products;
        }
        current_weight += product_to_insert.weight;
        current_price += product_to_insert.price;
        selected_products.push(product_to_insert);
        problem.suitcase.show();
    }

    println!("Local Search Greedy Solution: {}€", current_price);
    problem.suitcase.show();
    return selected_products;
}