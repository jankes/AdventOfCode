use std::error::Error;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;

fn main() {
    if let Ok(input) = read_input("C:\\Users\\jankes\\Documents\\AdventOfCode\\2015\\12\\input.json") {
        let sum = ::part_1::sum_numbers(&input);
        println!("sum of numbers = {}", sum);
    }
}

fn read_input<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, Box<Error>> {
    let mut file = OpenOptions::new()
	               .read(true)
				   .open(path)?;
    
    let mut input = Vec::new();
    file.read_to_end(&mut input)?;
    Ok(input)
}

mod part_1 {
    pub fn sum_numbers(input: &[u8]) -> i32 {
        let mut state = State::Other;
        let mut is_negative = false;
        let mut digits = Vec::with_capacity(4);
        let mut sum = 0;
        for b in input {
            match state {
                State::Other => {
                    if is_minus_sign(*b) {
                        is_negative = true;
                        state = State::Number;
                    }
                    else if is_number(*b) {
                        digits.push(*b);
                        state = State::Number;
                    }
                },
                State::Number => {
                    if is_number(*b) {
                        digits.push(*b);
                    } else {
                        sum = update_sum(sum, is_negative, &digits);
                        digits.clear();
                        is_negative = false;
                        state = State::Other;
                    }
                }
            }        
        }
        if !digits.is_empty() {
            sum = update_sum(sum, is_negative, &digits);
        }
        return sum;

        fn update_sum(sum: i32, is_negative: bool, digits: &[u8]) -> i32 {
            if is_negative {
                sum - add_digits(&digits)
            } else {
                sum + add_digits(&digits)
            }
        }
    }

    fn add_digits(digits: &[u8]) -> i32 {
        let len = digits.len();
        if len == 0 {
            return 0;
        }

        let mut sum = (digits[len - 1] - b'0') as i32;
        if len == 1 {
            return sum;
        }

        let mut pow = 1;
        let mut i = len - 2;
        loop {
            sum += 10 * pow * (digits[i] - b'0') as i32;
            pow *= 10;
            if i == 0 {
                break;
            }
            i -= 1;
        }
        sum
    }

    fn is_number(b: u8) -> bool {
        match b {
            b'0'...b'9' => true,
            _ => false
        }
    }

    fn is_minus_sign(b: u8) -> bool {
        b == b'-'
    }

    enum State {
        Number, Other
    }

    #[cfg(test)]
    mod tests {
        use super::{add_digits, sum_numbers};

        #[test]
        fn add_digits_empty() {
            assert_eq!(0, add_digits(&[]));
        }

        #[test]
        fn add_digits_1() {
            assert_eq!(1, add_digits(b"1"));
        }

        #[test]
        fn add_digits_7() {
            assert_eq!(7, add_digits(b"7"));
        }

        #[test]
        fn add_digits_90() {
            assert_eq!(90, add_digits(b"90"));
        }

        #[test]
        fn add_digits_31() {
            assert_eq!(31, add_digits(b"31"));
        }

        #[test]
        fn add_digits_418() {
            assert_eq!(418, add_digits(b"418"));
        }

        //
        //

        #[test]
        fn sum_numbers_empty() {
            assert_eq!(0, sum_numbers(b""));
        }

        #[test]
        fn sum_numbers_1aa() {
            assert_eq!(1, sum_numbers(b"1aa"));
        }

        #[test]
        fn sum_numbers_aa1() {
            assert_eq!(1, sum_numbers(b"aa1"));
        }

        #[test]
        fn sum_numbers_1() {
            assert_eq!(1, sum_numbers(b"1"));
        }
        
        #[test]
        fn sum_numbers_aa10() {
            assert_eq!(10, sum_numbers(b"aa10"));
        }

        #[test]
        fn sum_numbers_10_11_12() {
            assert_eq!(33, sum_numbers(b"[10,11,12]"));
        }

        #[test]
        fn sum_numbers_minus_1() {
            assert_eq!(-1, sum_numbers(b"-1"));
        }

        #[test]
        fn sum_numbers_minus_1_1() {
            assert_eq!(0, sum_numbers(b"-1, 1"));
        }

        #[test]
        fn sum_numbers_input_copied_1() {
            assert_eq!(429, sum_numbers(b"\"yellow\",141,[42,197],[-12,61,"));
        }
    }
}
