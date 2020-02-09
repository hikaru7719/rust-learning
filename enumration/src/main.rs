fn main() {
    let x: Message = Message::Move { x: 3, y: 7 };

    enum BoardGameTrun {
        Move { squares: i32 },
        Pass,
    }
    let y: BoardGameTrun = BoardGameTrun::Move { squares: 1 };
    let m = Message::Write("Hello World".to_string());

    let a = 5;
    match a {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        _ => println!("something else"),
    }

    let b = 1;
    match b {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let origin = Point { x: 0, y: 0 };

    match origin {
        Point { x: x1, y: y1 } => println!("({},{})", x1, y1),
    }

    // match some_value {
    //     Ok(value) => println!("got a value: {}", value),
    //     Err(_) => println!("an error occurred"),
    // }

    let c = OptionalTuple::Value(5, 3, 2);
    match c {
        OptionalTuple::Value(..) => println!("Got a tuple!"),
        OptionalTuple::Missng => println!("No such luck"),
    }

    let mut d = 5;
    match d {
        ref r => println!("Got a reference to {}", r),
    }

    let e = 1;
    match e {
        1...5 => println!("one through five"),
        _ => println!("anything"),
    }

    let g = 1;
    match g {
        e @ 1...5 => println!("got a range element {}", e),
        _ => println!("anything"),
    }
}

enum OptionalTuple {
    Value(i32, i32, i32),
    Missng,
}

enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
}

struct Point {
    x: i32,
    y: i32,
}
