#[test]
fn ex01() {
    let b = Box::new(5);
    println!("b = {b}");
}

#[test]
fn ex02() {
    let a = 5;
    let b = Box::new(a);
    let c = *b;
    println!("a = {a}, b = {b}, c = {c}");
}
#[allow(dead_code)]
#[test]
fn ex03() {
    #[derive(Debug)]
    struct Point {
        x: f64,
        y: f64,
    }
    let p = Box::new(Point { x: 1.0, y: 2.0 });
    println!("p = {:?}", p);
}
