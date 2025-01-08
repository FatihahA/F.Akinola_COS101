struct Devices{
    brand:String,
    price:f32,
    quantity:u32
}

fn main(){
    let device_1 = Devices{
        brand:String::from("HP"),
        quantity:10,
        price:650_000.0
    };

    let device_2 = Devices{
        brand:String::from("IBM"),
        quantity:6,
        price:755_000.0
    };

    let device_3 = Devices{
        brand:String::from("Toshiba"),
        quantity:10,
        price:550_000.0
    };

    let device_4 = Devices{
        brand:String::from("Dell"),
        quantity:4,
        price:850_000.0
    };

    let total_cost = device_1.price * 3.0 + device_2.price * 3.0 + device_3.price * 3.0 + device_4.price * 3.0;
    println!("The total cost of your devices are {}", total_cost);

}

