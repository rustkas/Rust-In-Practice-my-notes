#![allow(dead_code)]

mod my_module {
    pub fn foo() {
        println!("Hello, Foo!");
    }
}

mod my_lib {
    pub mod module_a {
        pub fn foo() {}
    }

    pub mod module_b {
        pub fn bar() {}
    }
}

fn main() {
    use crate::my_module::foo;
    foo();

    use my_lib::module_a;
    module_a::foo();

    use my_lib::module_b;
    module_b::bar();
}
