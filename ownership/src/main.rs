fn main() {
    let vector = vec![1, 2, 3];
    let vector2 = vec![1, 2, 3];
    let answer = foo(&vector, &vector2);
    let v = 1;
    let v2 = v;
    println!("v is : {}", v);

    let a = 5;
    let _y = double(a);
    println!("{}", a);

    let mut x = 5;
    {
        let y = &mut x;
        *y += 1;
    }
    println!("{}", x);

    let y = &5;
    let f = Foo { x: y };
    println!("{}", f.x());
}

fn double(x: i32) -> i32 {
    x * 2
}

fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    42
}

struct Foo<'a> {
    x: &'a i32,
}

impl<'a> Foo<'a> {
    fn x(&self) -> &'a i32 {
        self.x
    }
}
