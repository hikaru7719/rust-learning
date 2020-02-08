use std::cell::Cell;
use std::cell::RefCell;
use std::sync::Arc;

fn main() {
    let mut x = 5;
    let mut y = &mut x;
    let (mut z, v) = (5, 6);
    let a = Arc::new(5);
    let b = a.clone();
    let c = RefCell::new(42);
    let d = c.borrow_mut();
    let mut point = Point { x: 5, y: 6 };
    point.x = 10;
    let cell_point = CellPoint {
        x: 5,
        y: Cell::new(6),
    };
    cell_point.y.set(7);
    println!("{}", point.x);
    println!("{:?}", cell_point.y);

    let mut point3d = Point3d { x: 0, y: 0, z: 0 };
    point3d = Point3d { y: 1, ..point3d };

    let black = Color(0, 0, 0);
    let origin = Pointer(0, 0, 0);

    let length = Inches(10);
    let Inches(integer_length) = length;
    println!("length is {} inches", integer_length);
}

struct Color(i32, i32, i32);
struct Pointer(i32, i32, i32);
struct Inches(i32);

struct Point {
    x: i32,
    y: i32,
}

struct CellPoint {
    x: i32,
    y: Cell<i32>,
}

struct Point3d {
    x: i32,
    y: i32,
    z: i32,
}
