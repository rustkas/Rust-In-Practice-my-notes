#![allow(dead_code)]
#![allow(unused_imports)]

use std::sync::Arc;
use std::thread;

#[test]
fn ex01() {
    let a = Arc::new(5);
    let b = a.clone();
    let handle = thread::spawn(move || {
        println!("a = {}, b = {}", a, b);
    });
    handle.join().unwrap();
}

#[test]
fn ex02() {
    #[derive(Debug)]
    struct Point {
        x: f64,
        y: f64,
    }

    let p = Arc::new(Point { x: 1.0, y: 2.0 });
    let q = p.clone();
    let handle = thread::spawn(move || {
        println!("p = {:?}, q = {:?}", p, q);
    });
    handle.join().unwrap();
}
