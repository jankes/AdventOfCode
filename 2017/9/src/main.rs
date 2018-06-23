use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    let chars = read_to_vec("C:\\Users\\jankes\\Documents\\AdventOfCode\\2017\\9\\char_stream.txt");

    let (total_score, garbage_count) = process_chars(&chars);
    println!("total score = {}", total_score);
    println!("garbage count = {}", garbage_count);
}

fn process_chars(chars: &[u8]) -> (i32, u32) {
    let mut state = State::Group;
    let mut score = 0i32;
    let mut total_score = 0i32;
    let mut garbage_count = 0u32;
    for (i, c) in chars.iter().enumerate() {
        match state {
            State::Group => {
                match c {
                    b'{' => {
                        score += 1;
                    },
                    b'}' => {
                        total_score += score;
                        score -= 1;
                    },
                    b'<' => {
                        state = State::Garbage;
                    }
                    b',' => (),
                    _ => {
                        panic!("expected comma at char index {}, instead got char with value {}", i, c);
                    }
                };
            },
            State::Garbage => {
                match c {
                    b'!' => state = State::Skip,
                    b'>' => state = State::Group,
                    _ => garbage_count += 1
                };
            },
            State::Skip => {
                state = State::Garbage;
            }
        };
    }
    (total_score, garbage_count)
}

enum State {
    Group, Garbage, Skip
}

fn read_to_vec<P: AsRef<Path>>(file: P) -> Vec<u8> {
    let mut raw_data = Vec::<u8>::with_capacity(24 * 1024);
    File::open(file).unwrap()
    .read_to_end(&mut raw_data).expect("should be able to read file to memory");
    raw_data
}

#[cfg(test)]
mod tests {
    use super::process_chars;

    fn calculate_total_score(chars: &[u8]) -> i32 {
        let (total_score, _) = process_chars(chars);
        total_score
    }

    fn calculate_garbage_count(chars: &[u8]) -> u32 {
        let (_, garbage_count) = process_chars(chars);
        garbage_count
    }

    #[test]
    fn s1() {
        assert_eq!(1, calculate_total_score(b"{}"));
    }

    #[test]
    fn s2() {
        assert_eq!(6, calculate_total_score(b"{{{}}}"));
    }

    #[test]
    fn s3() {
        assert_eq!(5, calculate_total_score(b"{{},{}}"));
    }

    #[test]
    fn s4() {
        assert_eq!(16, calculate_total_score(b"{{{},{},{{}}}}"));
    }

    #[test]
    fn s5() {
        assert_eq!(1, calculate_total_score(b"{<a>,<a>,<a>,<a>}"));
    }

    #[test]
    fn s6() {
        assert_eq!(9, calculate_total_score(b"{{<ab>},{<ab>},{<ab>},{<ab>}}"));
    }

    #[test]
    fn s7() {
        assert_eq!(9, calculate_total_score(b"{{<!!>},{<!!>},{<!!>},{<!!>}}"));
    }

    #[test]
    fn s8() {
        assert_eq!(3, calculate_total_score(b"{{<a!>},{<a!>},{<a!>},{<ab>}}"));
    }

    #[test]
    fn g1() {
        assert_eq!(0, calculate_garbage_count(b"<>"));
    }

    #[test]
    fn g2() {
        assert_eq!(17, calculate_garbage_count(b"<random characters>"));
    }

    #[test]
    fn g3() {
        assert_eq!(3, calculate_garbage_count(b"<<<<>"));
    }

    #[test]
    fn g4() {
        assert_eq!(2, calculate_garbage_count(b"<{!>}>"));
    }

    #[test]
    fn g5() {
        assert_eq!(0, calculate_garbage_count(b"<!!>"));
    }

    #[test]
    fn g6() {
        assert_eq!(0, calculate_garbage_count(b"<!!!>>"));
    }

    #[test]
    fn g7() {
        assert_eq!(10, calculate_garbage_count(b"<{o\"i!a,<{i<a>"));
    }
}