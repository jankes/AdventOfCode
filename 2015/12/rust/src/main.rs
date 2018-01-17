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

fn is_digit(b: u8) -> bool {
    match b {
        b'0'...b'9' => true,
        _ => false
    }
}

fn is_minus_sign(b: u8) -> bool {
    b == b'-'
}

mod part_2 {
    use std::slice;
    use std::iter::Peekable;
    use self::Sum::{Black, Red};
    use super::{is_digit, is_minus_sign};

    fn sum_value(it: &mut Peekable<slice::Iter<u8>>) -> Sum {
        match it.peek() {
            Some(&&b) => {
                match b {
                    b'{'  => sum_object(it),
                    b'['  => sum_array(it),
                    b'\"' => sum_string(it),
                    b'0'...b'9' | b'-' => sum_number(it),
                    _ => panic!("unexpected value type")                    
                }
            },
            None => Black(0)
        }
    }

    fn sum_object(it: &mut Peekable<slice::Iter<u8>>) -> Sum {
        it.next().expect("should have byte for opening curly brace of object");

        let mut json_obj_sum = Black(0);
        loop {
            skip_string(it);
            it.next().expect("expect byte for colon after object name");

            let current_value_sum = sum_value(it);

            if let (Black(v1), Black(v2)) = (json_obj_sum, current_value_sum) {
                json_obj_sum = Black(v1 + v2);
            } else {
                json_obj_sum = Red;
            }

            let b = it.next().expect("expect comma or curly brace after value in object");
            match *b {
                b',' => continue,
                b'}' => break,
                _    => panic!("unexpected token after object value: {}", *b)
            };
        }
        json_obj_sum
    }

    fn sum_array(it: &mut Peekable<slice::Iter<u8>>) -> Sum {
        Black(0)
    }

    fn sum_string(it: &mut Peekable<slice::Iter<u8>>) -> Sum {
        
        enum State {
            R, E, D, End, Black
        }

        it.next().expect("expect byte for open quote of string");

        let mut state = State::R;
        let mut b = *it.next().expect("incomplete string");
        while b != b'\"' {
            match state {
                State::R => {
                    if b == b'r' {
                        state = State::E;
                    } else {
                        state = State::Black;
                    }
                },
                State::E => {
                    if b == b'e' {
                        state = State::D;
                    } else {
                        state = State::Black;
                    }
                },
                State::D => {
                    if b == b'd' {
                        state = State::End;
                    } else {
                        state = State::Black;
                    }
                },
                State::End => {
                    state = State::Black;
                },
                State::Black => ()
            }
            b = *it.next().expect("incomplete string");
        }
        match state {
            State::End => Red,
            _          => Black(0)
        }
    }

    fn sum_number(it: &mut Peekable<slice::Iter<u8>>) -> Sum {
        let mut digits = Vec::with_capacity(4);
        let is_negative = {
            let first = *it.next().expect("number must have at least one character");
            if is_minus_sign(first) {
                true
            } else {
                digits.push(first - b'0');
                false
            }
        };

        while let Some(&&d) = it.peek() {
            if is_digit(d) {
                it.next();
                digits.push(d - b'0');
            } else {
                break;
            }
        }
        let mut sum = digits.pop().expect("number needs at least one digit") as i32;
        let mut pow = 10;
        while let Some(d) = digits.pop() {
            sum += (d as i32) * pow;
            pow *= 10;
        }
        if is_negative {
            Black(-sum)
        } else {
            Black(sum)
        }
    }

    fn skip_string(it: &mut Peekable<slice::Iter<u8>>) {
        it.next().expect("expect byte for open quote of string");
        while *it.next().expect("incomplete string") != b'\"' {
            continue;
        }
    }

    #[derive(Eq, PartialEq, Debug)]
    enum Sum {
        Black(i32), Red
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn sum_string_1() {
            let s = b"\"test\"";
            assert_eq!(Black(0), sum_string(&mut s.iter().peekable()));
        }

        #[test]
        fn sum_string_2() {
            let s = b"\"red\"";
            assert_eq!(Red, sum_string(&mut s.iter().peekable()));
        }

        #[test]
        fn sum_string_3() {
            let s = b"\"rred\"";
            assert_eq!(Black(0), sum_string(&mut s.iter().peekable()));
        }

        #[test]
        fn sum_string_4() {
            let s = b"\"redd\"";
            assert_eq!(Black(0), sum_string(&mut s.iter().peekable()));
        }

        #[test]
        fn sum_number_1() {
            let num = b"1";
            assert_eq!(Black(1), sum_number(&mut num.iter().peekable()));
        }

        #[test]
        fn sum_number_minus_1() {
            let num = b"-1";
            assert_eq!(Black(-1), sum_number(&mut num.iter().peekable()));
        }

        #[test]
        fn sum_number_14() {
            let num = b"14";
            assert_eq!(Black(14), sum_number(&mut num.iter().peekable()));
        }

        #[test]
        fn sum_number_minus_1002() {
            let num = b"-1002";
            assert_eq!(Black(-1002), sum_number(&mut num.iter().peekable()));
        }

        #[test]
        fn sum_object_1() {
            let obj = b"{\"test\":\"aaa\"}";
            assert_eq!(Black(0), sum_object(&mut obj.iter().peekable()));
        }

        #[test]
        fn sum_object_2() {
            let obj = b"{\"test\":\"aaa\",\"xyz\":\"abc\"}";
            assert_eq!(Black(0), sum_object(&mut obj.iter().peekable()));
        }

        #[test]
        fn sum_object_3() {
            let obj = b"{\"test\":\"aaa\",\"xyz\":\"red\",\"test_2\":\"green\"}";
            assert_eq!(Red, sum_object(&mut obj.iter().peekable()));
        }

        #[test]
        fn sum_object_4() {
            let obj = b"{\"one\":\"green\",\"two\":{\"three\":\"blue\",\"four\":\"yellow\"}}";
            assert_eq!(Black(0), sum_object(&mut obj.iter().peekable()));
        }

        #[test]
        fn sum_object_5() {
            let obj = b"{\"one\":\"green\",\"two\":{\"three\":\"blue\",\"four\":\"red\"}}";
            assert_eq!(Red, sum_object(&mut obj.iter().peekable()));
        }

        #[test]
        fn sum_object_6() {
            let obj = b"{\"one\":\"green\",\"two\":{\"three\":\"blue\",\"four\":\"red\",\"five\":\"purple\"},\"six\":\"brown\"}";
            assert_eq!(Red, sum_object(&mut obj.iter().peekable()));
        }

        #[test]
        fn sum_object_7() {
            let obj = b"{\"a\":1}";
            assert_eq!(Black(1), sum_object(&mut obj.iter().peekable()))
        }

        #[test]
        fn sum_object_8() {
            let obj = b"{\"a\":1,\"b\":3}";
            assert_eq!(Black(4), sum_object(&mut obj.iter().peekable()))
        }

        

    }
}

mod part_1 {
    use super::{is_digit, is_minus_sign};

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
                    else if is_digit(*b) {
                        digits.push(*b);
                        state = State::Number;
                    }
                },
                State::Number => {
                    if is_digit(*b) {
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
