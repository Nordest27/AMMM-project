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
#[derive(Clone)]
pub struct Suitcase {
    // in mm
    pub dim_x: i32,
    pub dim_y: i32,
    // grams
    pub max_weight: i32,
    // products with position
    products: Vec<(Product, i32, i32)>,
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
        for (p, px, py) in &self.products {
            if x < *px + p.dim_side && *px < x + product.dim_side &&
                y < *py + p.dim_side && *py < y + product.dim_side {
                return Some(p);
            }
        }
        return None;
    }

    fn find_all_possible_fits(&self, product: &Product) -> Vec<(i32, i32)> {
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
    pub fn add_product(&mut self, product: &Product, position: Option<(i32, i32)>) -> bool {
        let possible_fits = self.find_all_possible_fits(product);
        if possible_fits.len() == 0 {
            return false;
        }
        let (x, y) = match position {
            None => possible_fits[0],
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
}