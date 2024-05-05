#![allow(dead_code)]
#![allow(unused_imports)]

use std::rc::Rc;

#[test]
fn ex01() {
    let a = Rc::new(5);
    let b = a.clone();
    println!("a = {}, b = {}", a, b);
}

#[test]
fn ex02() {
    #[derive(Debug)]
    struct Point {
        x: f64,
        y: f64,
    }

    let p = Rc::new(Point { x: 1.0, y: 2.0 });
    let q = p.clone();
    println!("p = {:?},\n\rq = {:?}", p, q);
}
