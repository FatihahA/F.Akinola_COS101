fn main() {
 let p:f64 = 210000.0;
 let r:f64 = 5.0;
 let n:i32 = 3;
 let a = p * ( 1.0 - (r / 100.0)).powi(n as i32);
 println!("Amount is {}", a);
 let ci = a - p;
 println!("Compound Interest is {}", ci);
 let d = -1.0 * ci;
 println!("Depreciation after three years is {}", d );

 }