pub fn tuples1() {
    println!("Example tuples, get 2 data tuples from private/local method");
    fn get_data() -> (String, i32) {
        return (String::from("Opan"), 1);
    }

    let (data, number) = get_data();
    println!("data:{}, number:{}", data, number);
}


pub fn tuples2() {
    println!("Example tuples, modify data tuples (with closures) from private/local method");

    let current_stock = 10;

    let a = |product: String, qty: i32| -> (String, i32, i32) {
        let stock = current_stock - qty;
        return (product, qty, stock);
    };

    let (product, qty, stock) = a(String::from("Rokok"), 2);

    println!("Product {}, out {}, current stock {}", product, qty, stock);
}


