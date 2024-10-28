fn main() {
 let t:f64 = 450000.00;         //t - Toshiba
 let m:f64 = 1500000.00;       //m - Mac
 let h:f64 = 750000.00;       //h - HP
 let d:f64 = 2850000.00;     //d - Dell
 let a:f64 = 250000.00;     //a - Acer
 let qty_t:f64 = 2.00;     //qty represents quantity 
 let qty_m:f64 = 1.00;
 let qty_h:f64 = 3.00;
 let qty_d:f64 = 3.00;
 let qty_a:f64 = 1.00;
  let sum = (t * qty_t) + (m * qty_m) + (h * qty_h) + (d * qty_d) + (a * qty_a);
  println!("The sum of P.M. Okeke and sons ltd sales is {}", sum);
   let average = sum / (qty_t + qty_m + qty_h + qty_d + qty_a);
   println!("The average of P.M. Okeke and sons ltd sales is {}", average);

 }