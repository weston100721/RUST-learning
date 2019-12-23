use std::string::String;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(larger.can_hold(&smaller));
    }


    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }

    #[test]
    #[should_panic]
    fn get_guess() {
        let g = Guess::new(333);
    }

    #[test]
    #[should_panic("Guess value must be less than or equal to 100")]
    fn get_guess2() {
        let g = Guess::new(333);
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(v: i32) -> Guess {
        if v < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.",
                   v);
        } else if v > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.",
                   v);
        }

        Guess {
            value: v,
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}