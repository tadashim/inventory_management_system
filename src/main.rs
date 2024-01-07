use std::collections::HashMap;
use std::io;
use std::str::FromStr;

mod auth;
use auth::{User, authenticate};

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
        if self.products.contains_key(&product.id) {
            println!("Product with ID {} already exists", product.id);
        } else {
            self.products.insert(product.id, product);
            println!("Product Added Successfully");
        }
    }

    fn edit_product(&mut self, id: u32, new_product: Product) {
        if self.products.contains_key(&id) {
            self.products.insert(id, new_product);
            println!("Product Edited Successfully");
        } else {
            println!("Product with ID {} does not exist", id);
        }
    }

    fn delete_product(&mut self, id: u32) {
        if self.products.remove(&id).is_some() {
            println!("Product Deleted Successfully");
        } else {
            println!("Product with ID {} does not exist", id);
        }
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

fn get_product_details() -> Product {
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

    Product {
        id,
        name,
        description,
        price,
        quantity,
    }
}

fn main() {
    let users = vec![
        User {
            user_id: "admin".to_string(),
            password: "admin".to_string(),
        },
    ];

    let mut inventory = Inventory::new();

    loop {
        println!("Inventory Management System");
        println!("1. Login");
        println!("2. logout");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "1" => {
                println!("Enter User ID: ");
                let user_id: String = read_input();

                println!("Enter Password: ");
                let password: String = read_input();

                match authenticate(&users, &user_id, &password) {
                    Some(_user) => {
                        println!("Login Successful");
                        loop {
                            println!("1. Add Product");
                            println!("2. Edit Product");
                            println!("3. Delete Product");
                            println!("4. List Products");
                            println!("5. Exit");

                            let mut choice = String::new();
                            io::stdin().read_line(&mut choice).expect("Failed to read line");

                            match choice.trim() {
                                "1" => {
                                    let product = get_product_details();
                                    inventory.add_product(product);
                                }
                                "2" => {
                                    let product = get_product_details();
                                    inventory.edit_product(product.id, product);
                                },
                                "3" => {
                                    println!("Enter Product ID: ");
                                    let id: u32 = read_input();
                                    inventory.delete_product(id);
                                },
                                "4" => {
                                    inventory.list_products();
                                },
                                "5" => break,
                                _ => println!("Invalid option, please try again"),
                            }
                        }
                    },
                    None => println!("Invalid credentials"),
                }
            },
            "2" => break,
            _ => println!("Invalid option, please try again"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_product() {
        let mut inventory = Inventory::new();
        let product = Product {
            id: 1,
            name: "Test Product".to_string(),
            description: "Test Description".to_string(),
            price: 10.0,
            quantity: 100,
        };

        inventory.add_product(product);
        assert_eq!(inventory.products.contains_key(&1), true);
    }

    #[test]
    fn test_edit_product() {
        let mut inventory = Inventory::new();
        let product = Product {
            id: 1,
            name: "Test Product".to_string(),
            description: "Test Description".to_string(),
            price: 10.0,
            quantity: 100,
        };

        let new_product = Product {
            id: 1,
            name: "New Test Product".to_string(),
            description: "New Test Description".to_string(),
            price: 20.0,
            quantity: 200,
        };
        
        inventory.add_product(product);
        inventory.edit_product(1, new_product);
        assert_eq!(inventory.products.get(&1).unwrap().name, "New Test Product");
        assert_eq!(inventory.products.get(&1).unwrap().price, 20.0);
        assert_eq!(inventory.products.get(&1).unwrap().quantity, 200);
    }

    #[test]
    fn test_delete_product() {
        let mut inventory = Inventory::new();
        let product = Product {
            id: 1,
            name: "Test Product".to_string(),
            description: "Test Description".to_string(),
            price: 10.0,
            quantity: 100,
        };

        inventory.add_product(product);
        inventory.delete_product(1);
        assert_eq!(inventory.products.contains_key(&1), false);
    }
}
