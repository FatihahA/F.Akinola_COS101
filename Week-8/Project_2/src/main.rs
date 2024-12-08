use std::io;

fn main() {
    let mut developers = Vec::new();
    let mut input = String::new();

    println!("How many developers do you want to enter?");
    io::stdin().read_line(&mut input).unwrap();
    let count: usize = input.trim().parse().unwrap();

    for i in 1..=count {
        println!("Enter the name of developer {}:", i);
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let name = input.trim().to_string();

        println!("Enter the years of experience for {}:", name);
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let years_of_experience: u32 = input.trim().parse().unwrap();

        developers.push((name, years_of_experience));
    }

    // Finding the developer with the highest experience
    let mut top_developer = &developers[0];
    for dev in &developers {
        if dev.1 > top_developer.1 {
            top_developer = dev;
        }
    }

    println!(
        "The developer with the highest experience is {} with {} years.",
        top_developer.0, top_developer.1
    );
}

