fn main() {
    let x: i32 = 5;
    let y: f64 = 3.14;
    let _name: &str = "John";

    fn add(x: i32, y: i32) -> i32 {
        x + y
    }
    add(x, y as i32);

    let x = 5;
    if x > 0 {
        println!("x is positive");
    } else {
        println!("x is negative or zero");
    }

    loop {
        println!("This loop will run forever");
        if x > 0 {
            break;
        } else {
            continue;
        }
    }

    let y = 5;
    match y {
        1 => println!("y is 1"),
        2 => println!("y is 2"),
        _ => println!("y is something else"),
    }
    #[allow(dead_code)]
    struct Point {
        x: i32,
        y: i32,
    }
    let _p = Point { x: 5, y: 10 };

    trait Animal {
        fn make_sound(&self) -> &'static str;
    }

    struct Dog;
    impl Animal for Dog {
        fn make_sound(&self) -> &'static str {
            "bark"
        }
    }
    let dog = Dog;
    println!("The dog says {}", dog.make_sound());
}
