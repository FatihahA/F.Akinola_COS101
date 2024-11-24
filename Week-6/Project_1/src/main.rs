use std::io;

fn main() {
    // Displaying the menu
    println!("Menu:");
    println!("P = Poundo Yam/Edinkaiko Soup - ₦3,200");
    println!("F = Fried Rice & Chicken - ₦3,000");
    println!("A = Amala & Ewedu Soup - ₦2,500");
    println!("E = Eba & Egusi Soup - ₦2,000");
    println!("W = White Rice & Stew - ₦2,500");

    // Getting input from the user for food type and quantity
    let mut food_type = String::new();
    println!("Enter the type of food you want to order in terms of letters e.g P, F, A, E, W: ");
    io::stdin().read_line(&mut food_type).expect("Failed to read input");
    let food_type = food_type.trim().to_uppercase();

    let mut amount = String::new();
    println!("Enter the quantity you would like to order: ");
    io::stdin().read_line(&mut amount).expect("Failed to read input");
    let quantity: u32 = amount.trim().parse().expect("Invalid quantity");

    // Defining the prices
    let price = match food_type.as_str() {
        "P" => 3200,
        "F" => 3000,
        "A" => 2500,
        "E" => 2000,
        "W" => 2500,
        _ => {
            println!("Invalid food choice");
            return;
        }
    };

    // Calculating total cost
    let mut total_cost = price * quantity;

    if total_cost > 10_000 {
        total_cost = (total_cost as f64 * 0.95) as u32; // Applying 5% discount
    }

    println!("The total charges for your order are: ₦{}", total_cost);
} 