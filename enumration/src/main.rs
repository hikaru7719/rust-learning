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
}

enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
}
