#![allow(dead_code)]

use std::fmt;

#[derive(Debug, Clone)]
pub enum AgeCategory {
    Minor,
    Adult,
    Senior,
}

#[derive(Debug, Clone)]
pub struct Person {
    pub age: u8,
    pub job: Option<String>,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let job_str = match &self.job {
            Some(job) => job.as_str(),
            None => "Unemployed",
        };
        write!(f, "Person: age={}, job={}", self.age, job_str)
    }
}

pub fn classify_person(person: Person) -> AgeCategory {
    match person.age {
        age if age < 18 => AgeCategory::Minor,
        age if age < 65 => AgeCategory::Adult,
        _ => AgeCategory::Senior,
    }
}

// Matching on multiple patterns
#[test]
fn match_example_01() {
    let x = 3;
    match x {
        1 | 2 | 3 => println!("x is 1, 2, or 3"),
        _ => println!("x is something else"),
    }
}

// Matching on ranges
#[test]
fn match_example_02() {
    let x = 3;
    match x {
        1..=5 => println!("x is between 1 and 5"),
        _ => println!("x is something else"),
    }
}

// Matching on a reference
#[test]
fn match_example_03() {
    let x = 3;
    match &x {
        &y if y > 3 => println!("x is a reference to a value greater than 3"),
        &y if y < 3 => println!("x is a reference to a value less than 3"),
        _ => println!("x is a reference to a value equal to 3"),
    }
}

// Matching on a destructured value
#[test]
fn match_example_04() {
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 3, y: 4 };
    match p {
        Point { x, y } => println!("x: {}, y: {}", x, y),
    }
}

// Matching on a guard
#[test]
fn match_example_05() {
    let x = 3;
    match x {
        y if y % 2 == 0 => println!("x is even"),
        y if y % 2 == 1 => println!("x is odd"),
        _ => println!("x is something else"),
    }
}
