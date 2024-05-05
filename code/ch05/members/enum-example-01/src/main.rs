use enum_example_01::{classify_person, Person};

fn main() {
    let p1 = Person { age: 17, job: None };
    let p2 = Person {
        age: 35,
        job: Some("Software engineer".to_string(),
        ),
    };
    let p3 = Person {
        age: 70,
        job: Some("Retired".to_string()),
    };
    println!("{:?} is a {:?}", p1.job, classify_person(p1.clone()));
    println!("{:?} is a {:?}", p2.job, classify_person(p2.clone()));
    println!("{:?} is a {:?}", p3.job, classify_person(p3.clone()));
}
