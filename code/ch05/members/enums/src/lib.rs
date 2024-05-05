#[allow(dead_code)]
#[test]
fn defining_01() {
    enum Suit {
        Spades,
        Hearts,
        Diamonds,
        Clubs,
    }
    let _my_suit = Suit::Spades;

    enum Card {
        Ace(Suit),
        Two(Suit),
        Three(Suit),
        // ...
    }
    let _my_card = Card::Ace(Suit::Spades);
}

#[allow(dead_code)]
#[test]
fn pattern_matching() {
    #[derive(Debug)]
    enum Suit {
        Spades,
        Hearts,
        Diamonds,
        Clubs,
    }
    #[derive(Debug)]
    enum Card {
        Ace(Suit),
        Two(Suit),
        Three(Suit),
        // ...
    }

    use Suit::*;
    let my_suit = Spades;
    match my_suit {
        Spades => println!("The suit is spades"),
        Hearts => println!("The suit is hearts"),
        Diamonds => println!("The suit is diamonds"),
        Clubs => println!("The suit is clubs"),
    }
    use Card::*;
    let my_card = Ace(Suit::Spades);
    match my_card {
        Ace(suit) => println!("The card is an ace of{:?}", suit),
        Two(suit) => println!("The card is a two of {:?}", suit),
        Three(suit) => println!("The card is a three of {:?}", suit),
        // ...
    }
}

#[allow(dead_code)]
#[test]
fn enum_pattern_matching() {
    enum HttpStatus {
        Ok,
        NotFound,
        InternalServerError,
    }
    fn handle_request(status: HttpStatus) {
        use HttpStatus::*;
        match status {
            Ok => println!("Request succeeded"),
            NotFound => println!("Resource not found"),
            InternalServerError => println!("An internal server error occurred"),
        }
    }
}

#[allow(dead_code)]
#[test]
fn pattenr_matching_example() {
    #[derive(Debug, Clone)]
    struct Person {
        age: u8,
        job: Option<String>,
    }
    fn classify_person(person: Person) -> &'static str {
        if person.age < 18 {
            return "Minor";
        } else if person.age < 65 {
            return "Adult";
        } else {
            return "Senior";
        }
    }
    let person = Person {
        age: 43,
        job: Some("Software Developer".to_string()),
    };
    classify_person(person.clone());

    #[derive(Debug)]
    enum AgeCategory {
        Minor,
        Adult,
        Senior,
    }

    fn classify_person2(person: Person) -> AgeCategory {
        match person.age {
            age if age < 18 => AgeCategory::Minor,
            age if age < 65 => AgeCategory::Adult,
            _ => AgeCategory::Senior,
        }
    }
    let age_category = classify_person2(person);
    println!("{:?}", age_category);
}
