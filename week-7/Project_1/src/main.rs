use std::io;
fn main() {
    println!("MTH 101 AREA AND VOLUME CALCULATOR");
    println!("Please select what you would like to calculate in terms of numbers e.g 1.2,3...:");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Surface Area of Cube");
    println!("5. Volume of Cylinder");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let choice: u32 = choice.trim().parse().expect("Please enter a valid number");

    match choice {
        1 => area_of_trapezium(),
        2 => area_of_rhombus(), 
        3 => area_of_parallelogram(),
        4 => area_of_cube(),
        5 => volume_of_cylinder(),
        _ => println!("Invalid choice! Please select a valid option."),
    }
}

fn area_of_trapezium() {
    let mut base1 = String::new();
    let mut base2 = String::new();
    let mut height = String::new();

    println!("Enter the height of the trapezium:");
    io::stdin().read_line(&mut height).expect("Failed to read input");
    let height: f64 = height.trim().parse().expect("Please enter a valid number");

    println!("Enter the length of the first base:");
    io::stdin().read_line(&mut base1).expect("Failed to read input");
    let base1: f64 = base1.trim().parse().expect("Please enter a valid number");

    println!("Enter the length of the second base:");
    io::stdin().read_line(&mut base2).expect("Failed to read input");
    let base2: f64 = base2.trim().parse().expect("Please enter a valid number");

    let area = height / 2.0 * (base1 + base2);
    println!("The area of the trapezium is: {}", area);
}

fn area_of_rhombus() {
    let mut diagonal_1 = String::new();
    let mut diagonal_2 = String::new();

    println!("Enter the length of the first diagonal:");
    io::stdin().read_line(&mut diagonal_1).expect("Failed to read input");
    let diagonal_1: f64 = diagonal_1.trim().parse().expect("Please enter a valid number");

    println!("Enter the length of the second diagonal:");
    io::stdin().read_line(&mut diagonal_2).expect("Failed to read input");
    let diagonal_2: f64 = diagonal_2.trim().parse().expect("Please enter a valid number");

    let area = 0.5 * diagonal_1 * diagonal_2;
    println!("The area of the rhombus is: {}", area);
}

fn area_of_parallelogram() {
    let mut base = String::new();
    let mut altitude = String::new();

    println!("Enter the base length of the parallelogram:");
    io::stdin().read_line(&mut base).expect("Failed to read input");
    let base: f64 = base.trim().parse().expect("Please enter a valid number");

    println!("Enter the altitude (height) of the parallelogram:");
    io::stdin().read_line(&mut altitude).expect("Failed to read input");
    let altitude: f64 = altitude.trim().parse().expect("Please enter a valid number");

    let area = base * altitude;
    println!("The area of the parallelogram is: {}", area);
}

fn area_of_cube() {
    let mut side = String::new();

    println!("Enter the length of a side of the cube:");
    io::stdin().read_line(&mut side).expect("Failed to read input");
    let side: f64 = side.trim().parse().expect("Please enter a valid number");

    let area = 6.0 * side * side;
    println!("The surface area of the cube is: {}", area);
}

fn volume_of_cylinder() {
    let mut radius = String::new();
    let mut height = String::new();

    println!("Enter the radius of the cylinder:");
    io::stdin().read_line(&mut radius).expect("Failed to read input");
    let radius: f64 = radius.trim().parse().expect("Please enter a valid number");

    println!("Enter the height of the cylinder:");
    io::stdin().read_line(&mut height).expect("Failed to read input");
    let height: f64 = height.trim().parse().expect("Please enter a valid number");

    let volume = 3.142 * radius * radius * height;
    println!("The volume of the cylinder is: {}", volume);
}