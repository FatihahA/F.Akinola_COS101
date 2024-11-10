use std::io;

fn main() 
{
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    
    println!("Enter the value for a: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f64 = input1.trim().parse().expect("Not a valid number");
    
    
    println!("Enter the value for b: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f64 = input2.trim().parse().expect("Not a valid number");
    
    
    println!("Enter the value for c:");
    io::stdin().read_line(&mut input3).expect("Failed to read line");
    let c:f64 = input3.trim().parse().expect("Please enter a valid number");


      let discriminant = b * b - 4.0 * a * c;

     // Two real roots
     if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("The roots are real: {} and {}", root1, root2);

       // One real root 
    } else if discriminant == 0.0 {
        let _root = -b / (2.0 * a);
       
       // No real roots 
    } else {
         println!("No real roots");

}
}  