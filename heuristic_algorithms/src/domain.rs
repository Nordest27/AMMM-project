use std::cmp::max;
use std::cmp::min;
use rand::random;

pub fn show_rect(dim_x: i32, dim_y: i32, name: char) {
    print!("┏");
    for _ in 0..dim_x {
        print!(" ━ ")
    }
    print!("┓");
    for _ in 0..dim_y{
        println!();
        print!("│");
        for _ in 0..dim_x {
            print!(" {} ", name);
        }
        print!("│");
    }
    println!();
    print!("┗");
    for _ in 0..dim_x {
        print!(" ━ ")
    }
    print!("┛");
    println!();
}

pub fn show_rect_with_matrix(dim_x: i32, dim_y: i32, matrix: &Vec<Vec<char>>) {
    print!("┏");
    for _ in 0..dim_x {
        print!(" ━ ")
    }
    print!("┓");
    for i in 0..dim_y{
        println!();
        print!("│");
        for j in 0..dim_x {
            print!(" {} ", matrix[i as usize][j as usize]);
        }
        print!("│");
    }
    println!();
    print!("┗");
    for _ in 0..dim_x {
        print!(" ━ ")
    }
    print!("┛");
    println!();
}

#[derive(Clone)]
pub struct Product {
    pub name: char,
    // in mm (square box)
    pub dim_side: i32,
    // grams
    pub weight: i32,
    // €
    pub price: i32,
}

impl Product {
    pub fn show(&self) {
        println!("Product {}: {}X{}(mm), {}(g), {}(€)",
                 self.name, self.dim_side, self.dim_side, self.weight, self.price);
    }
}

impl PartialEq for Product {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[derive(Clone)]
pub struct Suitcase {
    // in mm
    pub dim_x: i32,
    pub dim_y: i32,
    // grams
    pub max_weight: i32,
    // products with position
    pub products: Vec<(Product, i32, i32)>,
}

impl PartialEq for Suitcase {
    fn eq(&self, other: &Self) -> bool {
        return self.dim_y==other.dim_y && self.dim_x==other.dim_y &&
            self.max_weight==other.max_weight &&
            self.products==other.products
    }
}

impl Suitcase {

    pub fn new() -> Suitcase {
        Suitcase {
            dim_x: 0,
            dim_y: 0,
            max_weight: 0,
            products: Vec::new(),
        }
    }
    pub fn init(dim_x: i32, dim_y: i32, max_weight: i32) -> Suitcase {
        Suitcase {
            dim_x,
            dim_y,
            max_weight,
            products: Vec::new(),
        }
    }

    fn out_of_bounds(&self, product: &Product, x: i32, y: i32) -> bool {
        return x < 0 || y < 0
            || x + product.dim_side > self.dim_x
            || y + product.dim_side > self.dim_y;
    }

    fn collision(&self, product: &Product, x: i32, y: i32) -> Option<&Product> {
        if product.dim_side <= 0 { return None }
        for (p, px, py) in &self.products {
            if x < *px + p.dim_side && *px < x + product.dim_side &&
                y < *py + p.dim_side && *py < y + product.dim_side {
                return Some(p);
            }
        }
        return None;
    }

    pub fn find_all_possible_fits(&self, product: &Product) -> Vec<(i32, i32)> {
        let mut fits = Vec::new();
        for i in 0..self.dim_x - product.dim_side + 1{
            for j in 0..self.dim_y - product.dim_side + 1{
                if self.collision(&product, i, j).is_none() {
                    fits.push((i, j));
                }
            }
        }
        return fits;
    }

    pub fn get_biggest_fit_in(&self, x: i32, y: i32) -> i32 {
        let mut biggest_fit = (self.dim_x-x).min(self.dim_y-y);

        while self.collision(&Product {
            name: ' ',
            dim_side: biggest_fit,
            weight: 0,
            price: 0,
        }, x, y).is_some() {
            biggest_fit -= 1;
        }
        return biggest_fit;
    }

    pub fn does_fit(&self, product: &Product) -> bool {
        for i in 0..self.dim_x - product.dim_side + 1 {
            for j in 0..self.dim_y - product.dim_side + 1 {
                if self.collision(&product, i, j).is_none() {
                    return true;
                }
            }
        }
        return false;
    }

    pub fn get_perimeter(&self) -> i32 {
        let mut perimeter = 0;
        let mut matrix = vec![vec![false; self.dim_x as usize]; self.dim_y as usize];
        for (product, x, y) in &self.products {
            for i in 0..product.dim_side {
                for j in 0..product.dim_side {
                    matrix[(y + i) as usize][(x + j) as usize] = true;
                }
            }
        }
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if !matrix[i][j] {
                    if i == 0 || matrix[i-1][j] {perimeter += 1;}
                    if i == (self.dim_y-1) as usize || matrix[i+1][j] {perimeter += 1;}
                    if j == 0 || matrix[i][j-1] {perimeter += 1;}
                    if j == (self.dim_x-1) as usize || matrix[i][j+1] {perimeter += 1;}
                }
            }
        }
        return perimeter;
    }

    pub fn add_product(&mut self, product: &Product, position: Option<(i32, i32)>) -> bool {
        if self.max_weight < self.get_weight() + product.weight {
            return false;
        }
        let possible_fits = self.find_all_possible_fits(product);
        if possible_fits.len() == 0 {
            return false;
        }
        let (x, y) = match position {
            None => possible_fits[rand::random::<usize>() % possible_fits.len()],
            Some((x, y)) => {
                if self.out_of_bounds(product, x, y) {
                    return false;
                }
                (x, y)
            }
        };
        self.products.push((product.clone(), x, y));
        return true;
    }

    pub fn move_product(&mut self, product: &Product, x: i32, y: i32) -> bool {
        if self.out_of_bounds(product, x, y) || self.collision(product, x, y).is_some() {
            return false;
        }
        if let Some((_, px, py)) =
            self.products.iter_mut().find(|(p, _, _)| p == product) {
            *px = x;
            *py = y;
            return true;
        }
        false
    }

    pub fn replace_product(&mut self, product: &Product, x: i32, y: i32) -> bool {

        if self.out_of_bounds(&product, x, y) {
            return false;
        }

        // Find and remove the existing product at the specified coordinates
        if let Some(i) = self.products.iter().position(|(_, px, py)| *px == x && *py == y) {
            self.products.remove(i);
        } else {
            return false; // No existing product at the specified coordinates
        }

        // Check collision for the new product
        if self.collision(&product, x, y).is_some() ||
            self.max_weight < self.get_weight() + product.weight
        {
            return false; // Collision with the new product
        }

        // Insert the new product at the specified coordinates
        self.products.push((product.clone(), x, y));
        true
    }

    pub fn remove_product(&mut self, product: &Product) -> bool {
        let mut index = 0;
        for (p, _, _) in &self.products {
            if p == product {
                self.products.remove(index);
                return true;
            }
            index += 1;
        }
        return false;
    }

    pub fn get_weight(&self) -> i32 {
        let mut weight = 0;
        for (product, _, _) in &self.products {
            weight += product.weight;
        }
        return weight;
    }

    pub fn get_price(&self) -> i32 {
        let mut price = 0;
        for (product, _, _) in &self.products {
            price += product.price;
        }
        return price;
    }
    pub fn show(&self) {
        println!("Max Weight: {}(g)", self.max_weight);
        println!("Suitcase Dims: {}X{}(mm)", self.dim_x, self.dim_y);
        let mut matrix = vec![vec![' '; self.dim_x as usize]; self.dim_y as usize];
        for (product, x, y) in &self.products {
            for i in 0..product.dim_side {
                for j in 0..product.dim_side {
                    matrix[(y + i) as usize][(x + j) as usize] = product.name;
                }
            }
        }
        show_rect_with_matrix(self.dim_x, self.dim_y, &matrix);
    }
}
#[derive(Clone)]
pub struct Problem {
    pub products: Vec<Product>,
    pub suitcase: Suitcase,
}

impl Problem {
    pub fn show(&self) {
        self.suitcase.show();
        for product in &self.products {
            product.show();
        }
    }

    pub fn remaining_possible_products(&self) -> Vec<Product> {
        let suitcase_products = self.suitcase.products.iter().map(
            |(p, _, _)| p
        ).collect::<Vec<&Product>>();
        return self.products.iter().filter(|product| {
            !suitcase_products.contains(product) &&
                product.weight + self.suitcase.get_weight() <= self.suitcase.max_weight &&
                self.suitcase.does_fit(&product)
        }).cloned().collect::<Vec<Product>>();
    }

    pub fn non_possible_products(&self) -> Vec<Product> {
        let suitcase_products = self.suitcase.products.iter().map(
            |(p, _, _)| p
        ).collect::<Vec<&Product>>();
        return self.products.iter().filter(|product| {
            !suitcase_products.contains(product) &&
                (product.weight + self.suitcase.get_weight() > self.suitcase.max_weight ||
                !self.suitcase.does_fit(&product))
        }).cloned().collect::<Vec<Product>>();
    }
}

fn calculate_max_weight(suitcase: &Suitcase) -> i32 {
    let mut weight = 0;
    for (product, _, _) in &suitcase.products {
        weight += product.weight;
    }
    return weight;
}

pub fn generate_problem(x: i32, y: i32) -> Problem {
    let mut products = Vec::new();
    let max_size = 80*x.min(y)/100;
    let mut remaining_area = x*y;
    let mut index = 0;
    let mut test_suitcase = Suitcase::init(x, y, i32::MAX);

    for i in 0..y {
        for j in 0..x {
            if (index + 65) as u8 as char > '~' {
                println!("Skipping because of char");
                break;
            }
            let biggest_fit = test_suitcase.get_biggest_fit_in(j, i);
            println!("Biggest Fit {}", biggest_fit);
            if biggest_fit < 1 {
                continue;
            }
            let mut product_size = max(
                min(max_size, biggest_fit) -
                    (remaining_area == biggest_fit*biggest_fit) as i32,
                1
            );
            if product_size % 2 == 0 && random::<bool>() { product_size /= 2;}
            else if product_size % 3 == 0 && random::<bool>() { product_size /= 3;}
            println!("Product size {}", product_size);
            let p = Product {
                name: (65 + index) as u8 as char,
                dim_side: product_size,
                weight: 1 + (random::<i32>() % 9).abs(),
                price: 1 + (random::<i32>() % 9).abs()
            };
            if (random::<i32>() % 5).abs() == 0 {
                let other_product = Product {
                    name: (65 + (index + 1)) as u8 as char,
                    dim_side: product_size,
                    weight: 1 + (random::<i32>() % 9).abs(),
                    price: 1 + (random::<i32>() % 9).abs()
                };
                println!("Other Product Added {}mm {}g {}€",
                         other_product.dim_side, other_product.weight, other_product.price);
                products.push(other_product);
                index += 1;
            }

            if test_suitcase.add_product( &p, Some((j, i)) ) {
                println!("Product Added {}mm {}g {}€", p.dim_side, p.weight, p.price);
                remaining_area -= p.dim_side*p.dim_side;
                products.push(p);
                index += 1;
            }
            else {
                println!("Couldn't add product {}mm {}g {}€", p.dim_side, p.weight, p.price);
            }
        }
    }
    println!("Generated Objective: {}€", test_suitcase.get_price());
    test_suitcase.show();
    let mut suitcase = Suitcase::init(x, y, calculate_max_weight(&test_suitcase));
    Problem {
        products,
        suitcase,
    }
}