fn main() {
    let plus_one = |x: i32| x + 1;
    assert_eq!(2, plus_one(1));
    let plus_two = |x: i32| {
        let mut result: i32 = x;
        result += 1;
        result += 1;
        result
    };
    assert_eq!(4, plus_two(2));

    let mut num = 5;
    {
        let plus_one = |x: i32| x + num;
    }
    let y = &mut num;

    let mut num2 = 5;
    let own_num = move |x: i32| x + num;

    let b = Baz;
    Foo::f(&b);
    Bar::f(&b);

    assert_eq!(10, <Fuga as Hoge>::hoge());
    assert_eq!(20, Fuga::hoge());
}

trait Foo {
    fn f(&self);
}

trait Bar {
    fn f(&self);
}

struct Baz;

impl Foo for Baz {
    fn f(&self) {
        println!("Baz implements Foo!!!");
    }
}

impl Bar for Baz {
    fn f(&self) {
        println!("Baz implements Bar!!!");
    }
}

trait Hoge {
    fn hoge() -> i32;
}

struct Fuga;

impl Fuga {
    fn hoge() -> i32 {
        20
    }
}

impl Hoge for Fuga {
    fn hoge() -> i32 {
        10
    }
}
