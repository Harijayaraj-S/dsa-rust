// Basic

// Write a program to print whether a number is even or odd, also take input from the user.
// Take name as input and print a greeting message for that particular name.
// Write a program to input principal, time, and rate (P, T, R) from the user and find Simple Interest.
// Take in two numbers and an operator (+, -, *, /) and calculate the value. (Use if conditions)
// Take 2 numbers as input and print the largest number.
// Input currency in rupees and output in USD.
// To find out whether the given String is Palindrome or not.
// To find Armstrong Number between two given number.

pub fn odd_or_even(number: i32) -> String {
    if number == 0 {
        return "0".to_string();
    } else if number % 2 == 0 {
        return "even".to_string();
    } else {
        return "odd".to_string();
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub fn simple_interest(p: i32, t: i32, r: i32) -> i32 {
    (p * t * r) / 100
}

pub fn calculator(number1: i32, number2: i32, operator: String) -> i32 {
    if operator == "+".to_string() {
        number1 + number2
    } else if operator == "-".to_string() {
        number1 - number2
    } else if operator == "*".to_string() {
        number1 * number2
    } else if operator == "/".to_string() {
        number1 / number2
    } else {
        return 0;
    }
}

pub fn largest_number(number1: i32, number2: i32) -> i32 {
    if number1 > number2 {
        number1
    } else if number2 > number1 {
        number2
    } else {
        0
    }
}

pub fn currency_converter(rupees: f64) -> f64 {
    rupees * 83.51
}

pub fn palindrome(word: String) -> bool {
    let length = word.len();
    let char_str: Vec<char> = word.chars().collect();
    let mut i = 0;
    while i < length / 2 {
        if char_str[i] != char_str[length - 1 - i] {
            return false;
        }
        i += 1;
    }
    true
}

pub fn armstrong_number(number: i32) -> bool {
    let mut sum = 0;
    let mut num = number;
    while num > 0 {
        sum += (num % 10).pow(3);
        num /= 10;
    }
    sum == number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_odd_or_even() {
        assert_eq!(odd_or_even(0), "0");
        assert_eq!(odd_or_even(2), "even");
        assert_eq!(odd_or_even(1), "odd");
    }

    #[test]
    fn test_greeting() {
        assert_eq!(greeting("peter"), "Hello peter!");
    }

    #[test]
    fn test_simple_interest() {
        assert_eq!(simple_interest(19, 10, 20), 38);
    }

    #[test]
    fn test_calculator() {
        assert_eq!(calculator(19, 10, "+".to_string()), 29);
        assert_eq!(calculator(19, 10, "-".to_string()), 9);
        assert_eq!(calculator(19, 10, "*".to_string()), 190);
        assert_eq!(calculator(19, 10, "/".to_string()), 1);
        assert_eq!(calculator(19, 10, "$".to_string()), 0);
    }

    #[test]
    fn test_largest_number() {
        assert_eq!(largest_number(19, 10), 19);
        assert_eq!(largest_number(10, 19), 19);
        assert_eq!(largest_number(10, 10), 0);
    }

    #[test]
    fn test_currency_converter() {
        assert_eq!(currency_converter(20.0), 1670.2);
    }

    #[test]
    fn test_palindrome() {
        assert!(!palindrome("rust".to_string()));
        assert!(palindrome("madam".to_string()));
    }

    #[test]
    fn test_armstrong_number() {
        assert!(armstrong_number(153));
        assert!(!armstrong_number(234));
    }
}
