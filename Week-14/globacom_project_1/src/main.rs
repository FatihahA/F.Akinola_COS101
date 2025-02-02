use std::io;
use std::io::Read;

fn main() {
    println!("WELCOME TO GLOBACOM!");
    println!("Please enter your role (Administrator, Project Manager, Employee, Customer, Vendor)");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let user_role = input.trim();

    match user_role.to_lowercase().as_str() {
        "administrator" => {
            println!("Displaying Globacom database..");
            display("globacom_db.sql");
        }
        "project manager" => {
            println!("Displaying Project table..");
            display("project_tb.sql");
        }
        "employee" => {
            println!("Displaying Staff table..");
            display("staff_tb.sql");
        }
        "customer" => {
            println!("Displaying Customer table..");
            display("customer_tb.sql");
        }
        "vendor" => {
            println!("Displaying Dataplan table...");
            display("dataplan_tb.sql");
        }
        _ => {
            println!("Invalid role!");
        }
    }
}

fn display(filename: &str) -> String {
    let mut file = std::fs::File::open(filename).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    println!("{}", contents);
    return contents.to_string();
}