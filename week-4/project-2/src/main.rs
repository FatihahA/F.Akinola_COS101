use std::io;

fn main() {
   

    let mut input1 = String::new();
    let mut input2 = String::new();


    println!("Enter the experience of the employee (in years):");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let experience:u32 = input1.trim().parse().expect("Not a valid number");


    println!("Enter the age of the employee:");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let age:u32 = input2.trim().parse().expect("Not a valid number");

    
    let incentive = if experience >= 3 {  // Experienced employees have worked for 3 or more
        if age >= 40 {
            1_560_000
        } else if age >= 30 {
            1_480_000
        } else if age >= 28 {
            1_300_000
        }  else {
            1_300_000
        }
    } else { 
        100_000
    };

    
    println!("The annual incentive for the employee is: N {}", incentive);
}

