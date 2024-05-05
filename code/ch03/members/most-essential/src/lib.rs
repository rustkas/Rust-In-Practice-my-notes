#[allow(unused_variables, unused_mut, unused_assignments)]
#[test]
fn variables() {
    let x = 5;
    let mut y = 10;
    let z;
    z = 15;

    let a: i32 = 20;
    let mut b: f64 = 3.14;

    let x = 5;
    let x: f64 = x as f64;

    let mut x = 5;
    let x = x; // x is now immutable

    let x = 10;
    let x = 5;
}

#[allow(dead_code)]
#[test]
fn constants() {
    const MAX_POINTS: u32 = 100_000;
    const GRAVITATIONAL_CONSTANT: f64 = 6.674e-11;
    const PI: f64 = 3.1415926535897932384626433;
    const AVG_MONTHLY_RAINFALL: f32 = 3.0;
    const US_POPULATION: u64 = 331_000_000;
    const MAX_SPEED: u8 = 255;
    const MAX_TEMPERATURE: i8 = 100;
}

#[allow(dead_code, unused_variables)]
#[test]
fn constants_vs_immutable_variables() {
    // This is a constant defined with the `const` keyword.
    // It must be annotated with its type and is defined in the global scope.
    const MAX_POINTS: u32 = 100_000;

    // This is an immutable variable defined with the `let` keyword.
    // It does not need to be annotated with its type, because the Rust compiler can infer it from the value being assigned.
    // It is defined inside a function, so it is not in the global scope.
    let current_points = 50_000;
    // The value of an immutable variable can be read, but it cannot be modified.
    // This line of code would produce an error, because current_points is an immutable variable.
    // current_points = 75_000;
    // However, it is possible to shadow an immutable variable with a new `let` binding with the same name.
    // This creates a new variable that shadows the old one, effectively "re-assigning" the value.
    let current_points = 75_000;
    // The value of a constant can also be read, but it cannot be modified.
    // This line of code would produce an error, because MAX_POINTS is a constant.
    // MAX_POINTS = 75_000;
}

#[allow(dead_code, unused_variables)]
#[test]
fn necessity_of_constant_against_variables() {
    // This is a constant defined with the `const` keyword.
    // It must be annotated with its type and is defined in the global scope.
    const PI: f64 = 3.14159265358979323846264338327950288419716939937510;
    // This is an immutable variable defined with the `let` keyword.
    // It does not need to be annotated with its type, because the Rust compiler can infer it from the value being assigned.
    // It is defined inside a function, so it is not in the global scope.
    let radius = 2.0;
    // The value of PI cannot be modified, because it is a constant.
    // This line of code would produce an error, because PI is a constant.
    // PI = 3.14;
    // The value of radius can be read, but it cannot be modified.
    // This line of code would produce an error, because radius is an immutable variable.
    // radius = 2.5;
    // However, it is possible to shadow the immutable variable radius with a new `let` binding with the same name.
    // This creates a new variable that shadows the old one, effectively "re-assigning" the value.
    let radius = 2.5;
}

#[allow(dead_code, unused_variables)]
#[test]
fn functions() {
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }
    let sum = add(5, 10);

    fn longest_string(strings: Vec<String>) -> String {
        let mut longest = strings[0].clone();
        for s in strings {
            if s.len() > longest.len() {
                longest = s;
            }
        }
        longest
    }

    let longest = longest_string(vec![
        "H".to_string(),
        "He".to_string(),
        "Hel".to_string(),
        "Hello".to_string(),
    ]);
    assert_eq!("Hello".to_string(), longest);
}

#[test]
fn function_generator() {
    fn count_up_to(limit: u32) -> impl Iterator<Item = u32> {
        let mut current = 0;
        std::iter::from_fn(move || {
            if current < limit {
                current += 1;
                Some(current)
            } else {
                None
            }
        })
    }

    let limit = 5;
    let mut counter = count_up_to(limit);

    println!("Counting up to {}:", limit);
    while let Some(number) = counter.next() {
        println!("{}", number);
    }
}

#[test]
fn type_of_functions() {
    // Free function
    println!("The sum of 3 and 4 is {}", add(3, 4));

    // Method function
    struct Point {
        x: i32,
        y: i32,
    }
    impl Point {
        fn distance_from_origin(&self) -> f64 {
            let pow1 = self.x.pow(2);
            let value1 = pow1 + self.y.pow(2);
            (value1 as f64).sqrt()
        }
    }
    let point = Point { x: 5, y: 10 };
    println!(
        "The distance from the origin is {}",
        point.distance_from_origin()
    );

    // Closure function
    let add_two = |x| x + 2;
    println!("3 plus 2 is {}", add_two(3));

    // Generator function
    fn count_up_from(start: i32) -> impl Iterator<Item = i32> {
        (start..).into_iter()
    }

    let mut generator = count_up_from(5);
    println!("{}", generator.next().unwrap());
    println!("{}", generator.next().unwrap());

    // Free function
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    // Closure function
    let add_two = |x| x + 2;
    println!("3 plus 2 is {}", add_two(3));
}

#[test]
fn passing_function_as_agrument() {
    fn apply_twice<F>(f: F, x: i32) -> i32
    where
        F: Fn(i32) -> i32,
    {
        f(f(x))
    }

    let double = |x| x * 2;
    let _y = apply_twice(double, 5);
}

#[test]
fn match_expression() {
    let grades = ["A", "B", "C", "D", "E", "F", "FX"];
    for grade in grades {
        match grade {
            "A" | "B" | "C" | "D" | "E" | "F" => println!("passed"),
            "FX" => println!("failed"),
            _ => println!("unknown"),
        }
    }
}
#[test]
fn understanding_traits() {
    trait Eq {
        fn eq(&self, other: &Self) -> bool;
    }

    impl Eq for i32 {
        fn eq(&self, other: &i32) -> bool {
            *self == *other
        }
    }

    let x = 5;
    let y = 10;
    assert!(Eq::eq(&x, &y) == false);
}

#[test]
fn implement_traits() {
    struct Point {
        x: i32,
        y: i32,
    }
    trait Distance {
        fn distance(&self, other: &Self) -> f64;
    }
    impl Distance for Point {
        fn distance(&self, other: &Point) -> f64 {
            let dx = (other.x - self.x) as f64;
            let dy = (other.y - self.y) as f64;
            (dx * dx + dy * dy).sqrt()
        }
    }

    let p1 = Point { x: 0, y: 0 };
    let p2 = Point { x: 3, y: 4 };
    let d = p1.distance(&p2);
    println!("The distance between p1 and p2 is {}", d);
}

#[test]
fn implement_traits2() {
    struct Rectangle {
        width: u32,
        height: u32,
    }
    trait HasArea {
        fn area(&self) -> u32;
    }
    impl HasArea for Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }
    fn print_area<T: HasArea>(shape: T) {
        println!("The area is {}", shape.area());
    }

    let rect = Rectangle {
        width: 10,
        height: 20,
    };
    print_area(rect);
}

#[test]
fn unions() {
    union MyUnion {
        i: i32,
        f: f32,
    }

    let mut u = MyUnion { i: 10 };
    unsafe {
        println!("u.i = {}", u.i);
        u.f = 3.14;
        println!("u.f = {}", u.f);
    }
}

#[allow(dead_code)]
#[test]
fn type_of_unions() {
    {
        union MyUnion {
            i: i32,
            f: f32,
            b: bool,
        }
    }

    {
        #[derive(Copy, Clone)]
        enum Color {
            Red,
            Green,
            Blue,
        }
        union MyUnion {
            c: Color,
            i: i32,
        }
    }

    {
        #[derive(Copy, Clone)]
        struct Point {
            x: i32,
            y: i32,
        }
        union MyUnion {
            p: Point,
            i: i32,
        }
    }
    {
        union MyUnion {
            a: [i32; 4],
            i: i32,
        }
    }

    {
        let array = [1, 2, 3, 4, 5, 6, 7, 8];
        union MyUnion {
            a: [i32; 8],
            i: i32,
        }
        let mut u = MyUnion { a: array };
        unsafe {
            println!("u.a = {:?}", u.a);
            u.i = 3;
            println!("u.f = {}", u.i);
        }
    }
}

#[test]
fn union_modify() {
    union MyUnion {
        i: i32,
        f: f32,
    }

    // Create an instance of the union with the `i` field set to 10
    let mut u = MyUnion { i: 10 };
    // Access and print the value of the `i` field
    unsafe {
        println!("u.i = {}", u.i);
    }
    // Modify the value of the `f` field
    u.f = 3.14;

    // Access and print the value of the `f` field
    unsafe {
        println!("u.f = {}", u.f);
    }
}

#[test]
fn struct_definition() {
    struct Point {
        x: i32,
        y: i32,
    }
    impl Point {
        fn distance_from_origin(&self) -> f64 {
            let x = self.x as f64;
            let y = self.y as f64;
            (x * x + y * y).sqrt()
        }
    }
    let p = Point { x: 10, y: 20 };
    let distance = p.distance_from_origin();

    println!("Distance = {distance}");
}

#[allow(dead_code)]
#[test]
fn derive_stuct() {
    #[derive(Debug)]
    struct BaseStruct {
        field1: i32,
        field2: i32,
    }
    impl BaseStruct {
        fn sum(&self) -> i32 {
            self.field1 + self.field2
        }
    }

    #[derive(Debug)]
    struct DerivedStruct {
        base: BaseStruct,
        field3: i32,
    }

    let base = BaseStruct {
        field1: 1,
        field2: 2,
    };
    let derived = DerivedStruct { base, field3: 3 };
    println!("{:?}", derived);

    let base = BaseStruct {
        field1: 1,
        field2: 2,
    };
    let derived = DerivedStruct { base, field3: 3 };

    println!("The value of field1 is {}", derived.base.field1);
    println!("The value of field2 is {}", derived.base.field2);

    let _base_sum = derived.base.sum();
}

#[allow(dead_code)]
#[test]
fn derive_stuct2() {
    trait BaseTrait {
        fn sum(&self) -> i32;
    }

    struct DerivedStruct<'a> {
        base: &'a dyn BaseTrait,
        field3: i32,
    }

    struct BaseStruct {
        field1: i32,
        field2: i32,
    }
    impl BaseTrait for BaseStruct {
        fn sum(&self) -> i32 {
            self.field1 + self.field2
        }
    }

    let base = BaseStruct {
        field1: 1,
        field2: 2,
    };
    let base_ref: &dyn BaseTrait = &base;
    let derived = DerivedStruct {
        base: base_ref,
        field3: 3,
    };
    let _base_sum = derived.base.sum();
}
