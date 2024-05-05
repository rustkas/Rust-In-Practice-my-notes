#![allow(unused_imports)]
#![allow(dead_code)]

use std::cell::RefCell;

#[test]
fn ex01() {
    use std::rc::Rc;

    struct Node {
        value: i32,
        next: Option<Rc<RefCell<Node>>>,
    }

    let a = Rc::new(RefCell::new(Node {
        value: 5,
        next: None,
    }));
    let b = Rc::new(RefCell::new(Node {
        value: 6,
        next: Some(Rc::clone(&a)),
    }));
    let c = Rc::new(RefCell::new(Node {
        value: 7,
        next: Some(Rc::clone(&b)),
    }));
    
    a.borrow_mut().next = Some(Rc::clone(&c));
}
