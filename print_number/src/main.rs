fn main() {
    let x = 5;
    if x == 5 {
        println!("x is 5");
    } else if x == 6 {
        println!("x is 6");
    } else {
        println!("other");
    }
    print_number(add_number(1), 6);

    for z in 0..10 {
        println!("{}", z);
    }

    for (i, j) in (5..10).enumerate() {
        println!("i = {}, j = {}", i, j);
    }

    let done = false;
    loop_fn(done);
}

fn print_number(x: i32, y: i32) {
    println!("sum is: {}", x + y);
}

fn add_number(x: i32) -> i32 {
    x + 1
}

fn loop_fn(mut done: bool) {
    let mut a = 5;
    while !done {
        a += a - 3;

        println!("{}", a);
        if a % 5 == 0 {
            done = true;
        }
    }
}
