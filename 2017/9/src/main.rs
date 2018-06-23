use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    let chars = read_to_vec("C:\\Users\\jankes\\Documents\\AdventOfCode\\2017\\9\\char_stream.txt");

    let total_score = process_chars(&chars);
    println!("total score = {}", total_score);
}

fn process_chars(chars: &[u8]) -> i32 {
    let mut state = State::Group;
    let mut score = 0i32;
    let mut total_score = 0i32;
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
                    _ => ()
                };
            },
            State::Skip => {
                state = State::Garbage;
            }
        };
    }
    total_score
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

    #[test]
    fn t1() {
        assert_eq!(1, process_chars(b"{}"));
    }

    #[test]
    fn t2() {
        assert_eq!(6, process_chars(b"{{{}}}"));
    }

    #[test]
    fn t3() {
        assert_eq!(5, process_chars(b"{{},{}}"));
    }

    #[test]
    fn t4() {
        assert_eq!(16, process_chars(b"{{{},{},{{}}}}"));
    }

    #[test]
    fn t5() {
        assert_eq!(1, process_chars(b"{<a>,<a>,<a>,<a>}"));
    }

    #[test]
    fn t6() {
        assert_eq!(9, process_chars(b"{{<ab>},{<ab>},{<ab>},{<ab>}}"));
    }

    #[test]
    fn t7() {
        assert_eq!(9, process_chars(b"{{<!!>},{<!!>},{<!!>},{<!!>}}"));
    }

    #[test]
    fn t8() {
        assert_eq!(3, process_chars(b"{{<a!>},{<a!>},{<a!>},{<ab>}}"));
    }
}