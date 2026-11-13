//  This is the file contains the lesson we have learn from the references and borrowing
//  we are building a student management system

struct Product {
    name: String,
    price: u32,
    stock: u32,
}

fn main() {
    let mut product = Product {
        name: String::from("Apple"),
        price: 32,
        stock: 32,
    };

    product_display(&product);

    update_stock(&mut product, "Samsung", 50000, 10);

    product_display(&product);

    let invetory_value = calculate_inventory(&product);
    println!("Invetory value: {}", invetory_value);
}

fn product_display(product: &Product) {
    println!("Product name: {}", product.name);
    println!("Price of the {} is {}", product.name, product.price);
    println!("{} stocks are {}", product.name, product.stock);
}

fn update_stock(product: &mut Product, name: &str, price: u32, stock: u32) {
    product.name = String::from(name);
    product.price = price;
    product.stock = stock;
}

fn calculate_inventory(product: &Product) -> u32 {
    product.price * product.stock
}
