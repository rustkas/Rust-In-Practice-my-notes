use std::io::{self, Error, ErrorKind};

pub fn read_line() -> Result<String, Error> {
  let mut input = String::new();
  match io::stdin().read_line(&mut input) {
      Ok(_) => Ok(input),
      Err(e) => Err(Error::new(ErrorKind::Other, e)),
  }
}


pub fn calculate(input: &str) -> f64 {
  let mut parts = input.split_whitespace(); // Разбиваем строку на части по пробелам
  let num1_str = parts.next().expect("Expected a number");
  let operator = parts.next().expect("Expected an operator");
  let num2_str = parts.next().expect("Expected a number");

  let num1: f64 = num1_str.parse().expect("Failed to parse number");
  let num2: f64 = num2_str.parse().expect("Failed to parse number");

  match operator {
      "+" => num1 + num2,
      "-" => num1 - num2,
      "*" => num1 * num2,
      "/" => num1 / num2,
      _ => panic!("Invalid operator"),
  }
}
