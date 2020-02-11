fn main() {
    let c = Circle::new(0.0, 0.0, 2.0);
    println!("{}", c.area());

    let d = c.grow(2.0).area();
    println!("{}", d);

    let builder_circle = CircleBuilder::new().x(1.0).y(1.0).radius(2.0).finalize();
    println!("{}", builder_circle.x);
    println!("{}", builder_circle.y);
    println!("{}", builder_circle.area());
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }

    fn grow(&self, increment: f64) -> Circle {
        Circle {
            x: self.x,
            y: self.y,
            radius: self.radius + increment,
        }
    }

    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            x: x,
            y: y,
            radius: radius,
        }
    }
}

struct CircleBuilder {
    x: f64,
    y: f64,
    radius: f64,
}

impl CircleBuilder {
    fn new() -> CircleBuilder {
        CircleBuilder {
            x: 0.0,
            y: 0.0,
            radius: 1.0,
        }
    }

    fn x(&mut self, coodinate: f64) -> &mut CircleBuilder {
        self.x = coodinate;
        self
    }

    fn y(&mut self, coodinate: f64) -> &mut CircleBuilder {
        self.y = coodinate;
        self
    }

    fn radius(&mut self, radius: f64) -> &mut CircleBuilder {
        self.radius = radius;
        self
    }

    fn finalize(&self) -> Circle {
        Circle {
            x: self.x,
            y: self.y,
            radius: self.radius,
        }
    }
}
