use std::collections::HashMap;
use std::io;
use std::str::FromStr;

struct Product {
    id: u32,
    name: String,
    description: String,
    price: f32,
    quantity: i32,
}

struct Inventory {
    products: HashMap<u32, Product>,
}

impl Inventory {
    fn new() -> Inventory {
        Inventory {
            products: HashMap::new(),
        }
    }

    fn add_product(&mut self, product: Product) {
        self.products.insert(product.id, product);
    }

    fn edit_product(&mut self, id: u32, new_product: Product) {
        self.products.insert(id, new_product);
    }

    fn delete_product(&mut self, id: u32) {
        self.products.remove(&id);
    }

    fn list_products(&self) {
        for (id, product) in &self.products {
            println!("ID: {} - {}: {} - {} - Quantity: {}", id, product.name, product.description, product.price, product.quantity);
        }
    }
}

fn read_input<T: FromStr>() -> T {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    match input.trim().parse() {
        Ok(value) => value,
        Err(_) => panic!("Invalid input"),
    }
}

fn main() {
    let mut inventory = Inventory::new();

    loop {
        println!("Inventory Management System");
        println!("1. Add Product");
        println!("2. Edit Product");
        println!("3. Delete Product");
        println!("4. List Products");
        println!("5. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "1" => {
                println!("Enter Product ID: ");
                let id: u32 = read_input();

                println!("Enter Product Name: ");
                let name: String = read_input();

                println!("Enter Product Description: ");
                let description: String = read_input();

                println!("Enter Product Price: ");
                let price: f32 = read_input();

                println!("Enter Product Quantity: ");
                let quantity: i32 = read_input();

                let product = Product {
                    id,
                    name,
                    description,
                    price,
                    quantity,                    
                };

                inventory.add_product(product);

                println!("Product Added Successfully");
            }
            "2" => {
                println!("Enter Product ID: ");
                let id: u32 = read_input();

                println!("Enter Product Name: ");
                let name: String = read_input();

                println!("Enter Product Description: ");
                let description: String = read_input();

                println!("Enter Product Price: ");
                let price: f32 = read_input();

                println!("Enter Product Quantity: ");
                let quantity: i32 = read_input();

                let product = Product {
                    id,
                    name,
                    description,
                    price,
                    quantity,                    
                };

                inventory.edit_product(id, product);

                println!("Product Edited Successfully");
            },
            "3" => {
                println!("Enter Product ID: ");
                let id: u32 = read_input();

                inventory.delete_product(id);

                println!("Product Deleted Successfully");
            },
            "4" => {
                inventory.list_products();
            },
            "5" => break,
            _ => println!("Invalid option, please try again"),
        }
    }
}
