
fn main() {
	let input = vec!(b'1', b'1', b'1', b'3', b'1', b'2', b'2', b'1', b'1', b'3');    

    let output = look_and_say_n_times(&input, 40);
    println!("n = 40, length = {}", output.len());

    let output = look_and_say_n_times(&input, 50);
    println!("n = 50, length = {}", output.len());
}

fn look_and_say_n_times(input: &[u8], n: u8) -> Vec<u8> {
	let mut input = Vec::from(input);    
    let mut output = Vec::with_capacity(0);
    for _ in 0..n {
        output = look_and_say(&input);
        input = output.clone();
    }
    output
}

fn look_and_say(input: &[u8]) -> Vec<u8> {
    let mut output = Vec::with_capacity(input.len());
    let mut i = 0usize;
    while i < input.len() {
        let cur = input[i];
        let mut count = 1;
        while i + 1 < input.len() && input[i + 1] == cur {
            count += 1;
            i += 1;
        }
        for b in count.to_string().as_bytes() {
            output.push(*b);
        }
        output.push(cur);
        i += 1;
    }
    output
}

#[cfg(test)]
mod tests {
    use ::look_and_say;

    #[test]
    fn test_1() {
        let input = vec!(b'1');
        let output = look_and_say(&input);
        assert_eq!(vec!(b'1', b'1'), output);
    }

    #[test]
    fn test_11() {
        let input = vec!(b'1', b'1');
        let output = look_and_say(&input);
        assert_eq!(vec!(b'2', b'1'), output);
    }

    #[test]
    fn test_21() {
        let input = vec!(b'2', b'1');
        let output = look_and_say(&input);
        assert_eq!(vec!(b'1', b'2', b'1', b'1'), output);
    }
    
    #[test]
    fn test_1211() {
        let input = vec!(b'1', b'2', b'1', b'1');
        let output = look_and_say(&input);
        assert_eq!(vec!(b'1', b'1', b'1', b'2', b'2', b'1'), output);
    }

    #[test]
    fn test_111221() {
        let input = vec!(b'1', b'1', b'1', b'2', b'2', b'1');
        let output = look_and_say(&input);
        assert_eq!(vec!(b'3', b'1', b'2', b'2', b'1', b'1'), output);
    }
}
