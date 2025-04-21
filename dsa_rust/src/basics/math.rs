// Basic math

// Count Digits

fn count_digits(input: i64) -> i64 {
    let mut input_copy = input;
    let mut count = 0;

    while input_copy != 0 {
        count += 1;
        input_copy = input_copy / 10;
    }

    count
}

fn count_digit_with_log(input: i64) -> i64 {
    if input == 0 {
        return 1;
    }

    ((input as f64).log10() + 1.0) as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_digits() {
        assert_eq!(count_digits(9012), 4);
        assert_eq!(count_digits(2), 1);
        assert_eq!(count_digits(8901341), 7);
    }

    #[test]
    fn test_count_digits_with_log() {
        assert_eq!(count_digit_with_log(9012), 4);
        assert_eq!(count_digit_with_log(2), 1);
        assert_eq!(count_digit_with_log(8901341), 7);
    }
}
