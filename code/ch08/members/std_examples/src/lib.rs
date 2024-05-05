#[test]
fn hashmap_example() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert("Alice", 10);
    scores.insert("Bob", 3);
    scores.insert("Charlie", 5);
    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }
}

#[test]
fn error_example() {
    use std::error::Error;
    use std::fs;
    fn read_file(filename: &str) -> Result<String, Box<dyn Error>> {
        let current_dir = std::env::current_dir()?;
        println!("Current dir: {:?}", current_dir);
        let contents = fs::read_to_string(filename)?;
        Ok(contents)
    }

    let contents = read_file("my_file.txt");
    match contents {
        Ok(result) => println!("{result}"),
        Err(err) => eprintln!("{err}"),
    }
}

#[test]
fn std_net_example01() {
    use std::io::{Read, Write};
    use std::net::TcpListener;

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buf = [0; 1024];
        stream.read(&mut buf).unwrap();
        let response = "HTTP/1.1 200 OK\r\n\r\nHello, world!";
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

#[test]
fn std_sync_example01() {
    use std::sync::{Arc, Mutex};
    use std::thread;

    let data = Arc::new(Mutex::new(0));
    let data_clone = data.clone();
    let handle = thread::spawn(move || {
        let mut data = data_clone.lock().unwrap();
        *data += 1;
    });
    let data_clone = data.clone();
    let handle2 = thread::spawn(move || {
        let mut data = data_clone.lock().unwrap();
        *data += 1;
    });
    handle.join().unwrap();
    handle2.join().unwrap();
    println!("{result}", result = data.lock().unwrap());
}

#[test]
fn std_time() {
    use std::time::{SystemTime, UNIX_EPOCH};

    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let in_ms = since_the_epoch.as_millis();
    println!("{}", in_ms);
}

#[test]
fn std_fmt() {
    use std::fmt;
    struct Point {
        x: i32,
        y: i32,
    }
    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }
    let p = Point { x: 3, y: 4 };
    println!("{}", p);
}

#[test]
fn std_str() {
    use std::str;

    let bytes = [104, 101, 108, 108, 111];
    let s = str::from_utf8(&bytes).unwrap();
    println!("{}", s);
}

#[test]
fn std_iter() {
    use std::iter;

    let v: Vec<i32> = iter::repeat(5).take(10).collect();
    println!("{:?}", v);
}

#[test]
fn std_ops() {
    use std::ops::Add;

    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }
    impl Add for Point {
        type Output = Point;
        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    let p1 = Point { x: 3, y: 4 };
    let p2 = Point { x: 5, y: 6 };
    let p3 = p1 + p2;
    assert_eq!(p3, Point { x: 8, y: 10 });
}

#[test]
fn std_fs() {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    let file_open = File::open("my_file.txt");
    let file;
    match file_open {
        Ok(file_input) => file = file_input,
        Err(err) => {
            eprintln!("{err}");
            panic!("{err}")
        }
    }

    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    let _ = reader.read_to_string(&mut contents);
    println!("{}", contents);
}

#[test]
fn primitive_types() {
    {
        let x: i32 = 5;
        let y: f64 = 3.14;
        let z: bool = true;
        let c: char = 'a';
        println!("x = {}", x);
        println!("y = {}", y);
        println!("z = {}", z);
        println!("c = {}", c);
    }

    {
        let x: i32 = 5;
        let y: f64 = 3.14;
        let z = x + y as i32;
        println!("z = {}", z);
        let a = [1, 2, 3, 4, 5];
        println!("{} = {}", stringify!(a[2]), a[2]);
        println!("{} = {}", stringify!(a.len()), a.len());
        let b = &a[1..3];
        println!("b = {:?}", b);
    }
}

#[test]
fn slices() {
    let a = [1, 2, 3, 4, 5];
    let s = &a[1..3];
    println!("{s:?}");
}

#[test]
fn macroses() {
    macro_rules! println {
        ($($arg:tt)*) => (print!("{}\n", format_args!
        ($($arg)*)))
        }
    println!("Hello, world!");

    macro_rules! define_function {
        ($name:ident, $body:expr) => {
            fn $name() {
                $body
            }
        };
    }
    define_function!(foo, println!("This is the body of the foo function"));

    foo();

    macro_rules! helpcode {
        () => {
            println!("For help with your code, try these resources:");
            println!("- The Rust documentation: https://doc.rust-lang.org/stable/");
            println!(
                "- The Rust programming language book: https://doc.rust-lang.org/stable/book/"
            );
            println!("- The Rust reference: https://doc.rustlang.org/stable/reference/");
            println!("- Stack Overflow: https://stackoverflow.com/");
            println!("- The Rust subreddit: https://www.reddit.com/r/rust/");
        };
    }
    helpcode!();
}
