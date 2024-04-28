pub struct Product {
    pub name: char,
    // in mm (square box)
    pub dim_side: i32,
    // grams
    pub weight: i32,
    // €
    pub price: i32,
}

pub struct Suitcase {
    // in mm
    pub dim_x: i32,
    pub dim_y: i32,
    // grams
    pub max_weight: i32,
}

pub struct Problem {

    pub products: Vec<Product>,
    pub suitcase: Suitcase,
}

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
impl Problem {
    pub fn show(&self) {
        println!("Max Weight: {}(g)", self.suitcase.max_weight);
        println!("Suitcase Dims: {}X{}(mm)", self.suitcase.dim_x, self.suitcase.dim_y);
        show_rect(self.suitcase.dim_x, self.suitcase.dim_y, ' ');
        for product in &self.products {
            println!("Product {}: {}X{}(mm), {}(g), {}(€)",
                     product.name, product.dim_side, product.dim_side,
                     product.weight, product.price);
            show_rect(product.dim_side, product.dim_side, product.name);
        }
    }
}