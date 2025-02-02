use std::io::Read;

fn main(a: String){
    let mut file = std::fs::File::open(a).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
    return a.to_string;
}
