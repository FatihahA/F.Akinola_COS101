fn main() {
    let mut wood = 35;
    bush(&mut wood);
    wood *= 2;
    println!("The value of wood is: {}", wood);
}

fn bush(plank: &mut i32) {
    *plank /= 5;
    println!("The value of plank is: {}", *plank);
}
