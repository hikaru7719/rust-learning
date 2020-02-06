fn main() {
    print_number(add_number(1), 6);
}

fn print_number(x: i32, y: i32) {
    println!("sum is: {}", x + y);
}

fn add_number(x: i32) -> i32 {
    x + 1
}