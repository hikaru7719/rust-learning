fn main() {
    let x: Option<i32> = Some(5);
    let y: Option<f64> = Some(5.0f64);
    let int_origin = Point { x: 0, y: 0 };
    let float_origin = Point { x: 0.0, y: 0.0 };
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn swap(&mut self) {
        std::mem::swap(&mut self.x, &mut self.y);
    }
}
