#[test]
fn ex01() {
    let x = 5; // x is a stack-allocated variable with the value 5
    let y = &x; // y is a reference to x
    println!("{y}");
}

#[test]
fn ex02() {
    fn add_one(x: &mut i32) {
        *x += 1;
    }

    let mut x = 5;
    add_one(&mut x);
    println!("{}", x); // prints 6
}
