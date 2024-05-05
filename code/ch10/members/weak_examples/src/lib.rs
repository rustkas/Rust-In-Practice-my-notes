#![allow(dead_code)]
#![allow(unused_imports)]

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[test]
fn ex01() {
    struct Node {
        value: i32,
        parent: Weak<RefCell<Node>>,
        children: Vec<Rc<RefCell<Node>>>,
    }

    let a = Rc::new(RefCell::new(Node {
        value: 5,
        parent: Weak::new(),
        children: Vec::new(),
    }));
    let b = Rc::new(RefCell::new(Node {
        value: 6,
        parent: Rc::downgrade(&a),
        children: Vec::new(),
    }));

    a.borrow_mut().children.push(Rc::clone(&b));
}

#[test]
fn ex02() {
    struct Node {
        value: i32,
        parent: Option<Weak<RefCell<Node>>>,
        children: Vec<Rc<RefCell<Node>>>,
    }
    fn main() {
        let a = Rc::new(RefCell::new(Node {
            value: 5,
            parent: None,
            children: Vec::new(),
        }));
        let b = Rc::new(RefCell::new(Node {
            value: 6,
            parent: Some(Rc::downgrade(&a)),
            children: Vec::new(),
        }));
        a.borrow_mut().children.push(Rc::clone(&b));
    }
}
