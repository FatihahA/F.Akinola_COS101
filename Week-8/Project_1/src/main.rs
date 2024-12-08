use std::io;

fn main() {
    println!("PUBLIC SERVICE APS LEVEL CHECKER");
    println!("Which profession do you belong to - Answer using numbers e.g 1, 2, 3...");
    println!("1. Office Administrator");
    println!("2. Academic");
    println!("3. Lawyer");
    println!("4. Teacher");

     let mut profession = String::new();
     io::stdin().read_line(&mut profession).expect("Failed to read input");
     let profession: u32 = profession.trim().parse().expect("Please enter a valid number");

     match profession {
        1 => office_administrator(),
        2 => academic(),
        3 => lawyer(),
        4 => teacher(),
        _ => println!("Invalid choice! Please select a valid option."),
    }
}

fn office_administrator() {

    println!("How many years of experience do you have - Answer using numbers e.g 1, 2, 3...");
    println!("0. 1 to 2 years");
    println!("1. 3 to 5 years");
    println!("2. 6 to 8 years");
    println!("3. 9 to 10 years");
    println!("4. 11 to 13 years");
    println!("5. 14 or more years");

    let office_admin_level = vec!["APS 1-2", "APS 3-5", "APS 5-8","EL1 8-10","EL2 10-13", "SES"];
    let mut input1 = String::new();
    println!("Enter your experience level in years - a value between (0 - 5) ");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    let index:usize = input1.trim().parse().expect("Invalid Input");
    let ch: char = office_admin_level[index];
    print!("{} is your position for the expeience in N0 [{}]\n",ch, index);
}

fn academic() {

    println!("How many years of experience do you have - Answer using numbers e.g 1, 2, 3...");
    println!("0. 1 to 2 years");
    println!("1. 3 to 5 years");
    println!("2. 6 to 8 years");
    println!("3. 9 to 10 years");
    println!("4. 11 to 13 years");
    println!("5. 14 or more years");

    let academic_level = vec!["N/A", "APS 3-5", "APS 5-8","EL1 8-10","EL2 10-13", "SES"];
    let mut input2 = String::new();
    println!("Enter your experience level in years - a value between (0 - 5) ");
    std::io::stdin().read_line(&mut input2).expect("Failed to read input");
    let index1:usize = input2.trim().parse().expect("Invalid Input");
    let ch1: char = academic_level[index1];
    print!("{} is your position for the experience in N0 [{}]\n",ch1, index1);
}

fn lawyer() {

    println!("How many years of experience do you have - Answer using numbers e.g 1, 2, 3...");
    println!("0. 1 to 2 years");
    println!("1. 3 to 5 years");
    println!("2. 6 to 8 years");
    println!("3. 9 to 10 years");
    println!("4. 11 to 13 years");
    println!("5. 14 or more years");

    let lawyer_level = vec!["APS 1-2", "APS 3-5", "APS 5-8","EL1 8-10","EL2 10-13", "SES"];
    let mut input3 = String::new();
    println!("Enter your experience level in years - a value between (0 - 5) ");
    std::io::stdin().read_line(&mut input3).expect("Failed to read input");
    let index2:usize = input3.trim().parse().expect("Invalid Input");
    let ch2: char = lawyer_level[index2];
    print!("{} is your position for the experience in N0 [{}]\n",ch2, index2);
}

fn teacher() {

    println!("How many years of experience do you have - Answer using numbers e.g 1, 2, 3...");
    println!("0. 1 to 2 years");
    println!("1. 3 to 5 years");
    println!("2. 6 to 8 years");
    println!("3. 9 to 10 years");
    println!("4. 11 to 13 years");
    println!("5. 14 or more years");

    let teacher_level = vec!["APS 1-2", "APS 3-5", "APS 5-8","EL1 8-10","EL2 10-13", "SES"];
    let mut input4 = String::new();
    println!("Enter your experience level in years - a value between (0 - 5) ");
    std::io::stdin().read_line(&mut input4).expect("Failed to read input");
    let index3:usize = input4.trim().parse().expect("Invalid Input");
    let ch3: char = teacher_level[index3];
    print!("{} is your position for the experience in N0 [{}]\n",ch3, index3);
}





