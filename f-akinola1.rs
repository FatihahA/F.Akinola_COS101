use std::io;

fn main(){
   println!("\nStudent Council Voter System");

   let candidatenumber:u32 = 0;
   for mut candidatenumber in 0..=50{

   let mut name = String::new();
   let mut email = String::new();
   let mut department = String::new();
   let mut state_of_origin = String::new();
   let mut input1 = String::new();
   let mut input2 = String::new();
   let mut input3 = String::new();

   println!("\nPlease enter your name. ");
   io::stdin().read_line(&mut name).expect("Failed to read input");

   println!("\nPlease enter your email. ");
   io::stdin().read_line(&mut email).expect("Failed to read input");

   println!("\nWhich department are you in. ");
   io::stdin().read_line(&mut department).expect("Failed to read input");

   println!("\nWhere is your state of origin. ");
   io::stdin().read_line(&mut state_of_origin).expect("Failed to read input");


   
   println!("\nWhat year are you in(use only nuumbers e.g 1,2,3... ");
   io::stdin().read_line(&mut input1).expect("Failed to read input");
   let year:i32 = input1.trim().parse().expect("Not a valid number");
   println!("You are in your {} year", year);

   println!("Are you a class rep(true(yes)/false(no)");
   io::stdin().read_line(&mut input2).expect("nothing inputed");
   let class_rep: bool = input2.trim().parse().expect("pick true or false");

   println!("\nWhat is your CGPA ");
   io::stdin().read_line(&mut input3).expect("Failed to read input");
   let cgpa:f64 = input3.trim().parse().expect("Not a valid number");
   println!("Your CGPA is: {}", cgpa);


   if class_rep == true && year> 1 && cgpa> 4.0 {
   println!("Your name is: {}", name);
   println!("Your email is: {}", email);
   println!("Your are in {} department", department);
   println!("Your state of origin is: {}", state_of_origin);
   println!("\nYou are eligible to vote");
   candidatenumber+= 1;
   println!("\nElibible candidate {}", candidatenumber);

}
 else{
 	println!("You are illegible to vote");
 };
}
}


   
