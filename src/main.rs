#[derive(Debug, Clone, PartialEq)]
pub struct ZeroError;

pub fn fizzbuzz(a: u32) -> Result<String, ZeroError> {
    let multiple_three = a % 3 == 0;
    let multiple_five = a % 5 == 0;

    if a == 0 {
        Err(ZeroError)
    } else if multiple_three && multiple_five {
        Ok("FizzBuzz".to_string())
    } else if multiple_three {
        Ok("Fizz".to_string())
    } else if multiple_five {
        Ok("Buzz".to_string())
    } else {
        Ok(a.to_string())
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod fizzbuzz_tests {
    use super::*;

    #[test]
    fn test_when_input_is_zero_return_err() {
        assert_eq!(fizzbuzz(0), Err(ZeroError));
    }

    #[test]
    fn test_when_input_is_one_return_one() {
        assert_eq!(fizzbuzz(1), Ok("1".to_string()));
    }

    #[test]
    fn test_when_input_is_two_return_two() {
        assert_eq!(fizzbuzz(2), Ok("2".to_string()));
    }

    #[test]
    fn test_when_input_is_three_return_fizz() {
        assert_eq!(fizzbuzz(3), Ok("Fizz".to_string()));
    }

    #[test]
    fn test_when_input_is_six_return_fizz() {
        assert_eq!(fizzbuzz(6), Ok("Fizz".to_string()));
    }

    #[test]
    fn test_when_input_is_five_return_buzz() {
        assert_eq!(fizzbuzz(5), Ok("Buzz".to_string()));
    }

    #[test]
    fn test_when_input_is_fifteen_return_fizzbuzz() {
        assert_eq!(fizzbuzz(15), Ok("FizzBuzz".to_string()));
    }
}
