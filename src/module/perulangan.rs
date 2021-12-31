pub fn loop1(start: i32, end: i32) {
    println!("\n\nExample for loop with parameter start | end\n");
    for i in start..end {
        println!("loop for {}", i);
    }
}

pub fn loop2() {
    println!("Example loop, break with condition (1==10)");
    let mut i = 0;
    loop {
        i += 1;
        println!("loop {}", i);
        if i == 10 { break; }
    }
}

pub fn loop3() {
    println!("Example loop break with condition (1==10) and get return value");
    let mut i = 0;
    let result = loop {
        i += 1;
        println!("loop {}", i);
        if i == 7 { break i; }
    };
    println!("Result from loop {}", result);
}