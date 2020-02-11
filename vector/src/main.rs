fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    println!("{}", v[2]);

    for i in &v {
        println!("reference to {}", i);
    }

    for i in &mut v {
        println!("mutable reference to {}", i);
    }

    for i in v {
        println!("{}", i);
    }

    let s = "foo
  bar";
    assert_eq!("foo\n  bar", s);
    let s2 = "foo\
   bar";
    assert_eq!("foobar", s2);

    let mut s3 = "hello".to_string();
    println!("{}", s3);

    s3.push_str(", world");
    println!("{}", s3);

    take_slice(&s3);

    let hachiko = "忠犬ハチ公";

    for h in hachiko.as_bytes() {
        print!("{}", h);
    }
    println!("");
    for h in hachiko.chars() {
        print!("{}", h);
    }
    println!("");
    let dog = "hachiko";
    let hachi = &dog[0..5];
}

fn take_slice(slice: &str) {
    println!("got {}", slice);
}
