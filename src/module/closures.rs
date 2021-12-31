pub fn closures1() {
    println!("Example closures, 1 param (i32)");

    let new_val = |val: i32| fun_inc_i32_not_mut(val);
    println!("new_val {}", new_val(10));

    fn fun_inc_i32_not_mut(val: i32) -> i32 {
        let new_number = val + 1;
        return new_number;
    }
}


pub fn closures2() {
    println!("Example closures, No Param");

    let new_val = || fun_print_string("test");
    new_val();

    fn fun_print_string(val: &str) {
        println!("Data is {}", val);
    }
}


pub fn closures3() {
    println!("Example closures, repeat print");

    let log = |i: i32| println!("print as closures of {}", i);

    for i in 0..19 {
        log(i);
    }
}


