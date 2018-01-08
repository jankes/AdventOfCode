
fn main() {
    let pw = Password([b'v', b'z', b'b', b'x', b'k', b'g', b'h', b'b']);

    let pw2 = next_password(pw); 
    // Three different ways to call From/Into on &Password to get &str
    //let pw2str: &str = std::convert::From::from(&pw);
    //let pw2str = std::convert::Into::<&str>::into(&pw);
    {
        let pw2str: &str = (&pw2).into();
        println!("next password is: {}", pw2str);
    }

    let pw3 = next_password(pw2);
    {
        let pw3str: &str = (&pw3).into();
        println!("next password is: {}", pw3str);
    }
}

fn next_password(mut pw: Password) -> Password {
    pw.increment();
    while !pw.is_valid() {
        pw.increment();
    }
    pw
}

#[derive(Eq, PartialEq, Debug)]
struct Password([u8; 8]);

impl<'a> From<&'a Password> for &'a str {
    fn from(p: &'a Password) -> &'a str {
        std::str::from_utf8(&p.0).expect("password must at least have valid utf8 characters")
    }
}

impl<'a> From<&'a str> for Password {
    fn from(s: &'a str) -> Password {
        let bytes = s.as_bytes();
        if bytes.len() != 8 {
            panic!("password must be exactly 8 letters");
        }
        let mut p = [0u8; 8];
        for (i, b) in bytes.iter().enumerate() {
            if !(b'a' <= *b && *b <= b'z') {
                panic!("password must only contain lowercase letters");
            }
            p[i] = *b;
        }
        Password(p)
    }
}

impl Password {
    fn increment(&mut self) {
        let mut index = self.0.len() - 1;
        while index != usize::max_value() {
            let letter = self.0.get_mut(index).unwrap(); 
            if *letter < b'z' {
                *letter += 1;
                return;
            } else {
                *letter = b'a';
                index = index.wrapping_sub(1);
            }
        }
        // overflow check?
        //if index == usize::max_value() {
        //    panic!("password overflow!");
        //}
    }

    fn is_valid(&self) -> bool {
        self.has_increasing_straight_of_three_letters() &&
        !self.has_invalid_letter() &&
        self.has_two_different_nonoverlapping_pairs()
    }

    fn has_invalid_letter(&self) -> bool {
        for c in self.0.iter() {
            if *c == b'i' || *c == b'o' || *c == b'l' {
                return true;
            }
        }
        false
    }

    fn has_increasing_straight_of_three_letters(&self) -> bool {
        for i in 0..self.0.len() - 2 {
            if self.0[i] + 1 == self.0[i + 1] && self.0[i + 1] + 1 == self.0[i + 2] {
                return true;
            }
        }
        false
    }

    fn has_two_different_nonoverlapping_pairs(&self) -> bool {
        let mut first_letter_first_pair = 0u8;
        let mut i = 0;
        while i < self.0.len() - 1 {
            if self.0[i] == self.0[i + 1] {
                if first_letter_first_pair == 0u8 {
                    first_letter_first_pair = self.0[i];
                    i += 2;
                    while i < self.0.len() && self.0[i] == first_letter_first_pair {
                        i += 1;
                    }
                    continue;
                } else if first_letter_first_pair != self.0[i] {
                    return true;
                }
            }
            i += 1;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use ::Password;

    #[test]
    fn inc_aaaaaaaa() {
        let mut pw = Password::from("aaaaaaaa");
        pw.increment();
        assert_eq!(Password::from("aaaaaaab"), pw);
    }

    #[test]
    fn inc_aaaaaaaz() {
        let mut pw = Password::from("aaaaaaaz");
        pw.increment();
        assert_eq!(Password::from("aaaaaaba"), pw);
    }

    #[test]
    fn inc_aaaayzzz() {
        let mut pw = Password::from("aaaayzzz");
        pw.increment();
        assert_eq!(Password::from("aaaazaaa"), pw);
    }

    //

    #[test]
    fn increasing_straight_abcdefgh() {
        let pw = Password::from("abcdefgh");
        assert!(pw.has_increasing_straight_of_three_letters());
    }

    #[test]
    fn increasing_straight_acncdeyu() {
        let pw = Password::from("acncdeyu");
        assert!(pw.has_increasing_straight_of_three_letters());
    }

    #[test]
    fn increasing_straight_oauitxyz() {
        let pw = Password::from("oauitxyz");
        assert!(pw.has_increasing_straight_of_three_letters());
    }

    //

    #[test]
    fn increasing_straight_aabbccdd() {
        let pw = Password::from("aabbccdd");
        assert!(!pw.has_increasing_straight_of_three_letters());
    }

    #[test]
    fn increasing_straight_abdcdfgi() {
        let pw = Password::from("abdcdfgi");
        assert!(!pw.has_increasing_straight_of_three_letters());
    }

    //

    #[test]
    fn pairs_aabbcdef() {
        let pw = Password::from("aabbcdef");
        assert!(pw.has_two_different_nonoverlapping_pairs());
    }

    #[test]
    fn pairs_zxaabbcd() {
        let pw = Password::from("zxaabbcd");
        assert!(pw.has_two_different_nonoverlapping_pairs());
    }

    #[test]
    fn pairs_zaarbbcd() {
        let pw = Password::from("zaarbbcd");
        assert!(pw.has_two_different_nonoverlapping_pairs());
    }

    #[test]
    fn pairs_aacbbcde() {
        let pw = Password::from("aacbbcde");
        assert!(pw.has_two_different_nonoverlapping_pairs());
    }

    #[test]
    fn pairs_aaaabbfg() {
        let pw = Password::from("aaaabbfg");
        assert!(pw.has_two_different_nonoverlapping_pairs());
    }

    #[test]
    fn pairs_aaaabbbb() {
        let pw = Password::from("aaaabbbb");
        assert!(pw.has_two_different_nonoverlapping_pairs());
    }

    #[test]
    fn pairs_aacaabgg() {
        let pw = Password::from("aacaabgg");
        assert!(pw.has_two_different_nonoverlapping_pairs());
    }

    #[test]
    fn pairs_bbccddee() {
        let pw = Password::from("bbccddee");
        assert!(pw.has_two_different_nonoverlapping_pairs());
    }

    #[test]
    fn pairs_arccstaa() {
        let pw = Password::from("arccstaa");
        assert!(pw.has_two_different_nonoverlapping_pairs());
    }

    //

    #[test]
    fn pairs_abcdefgh() {
        let pw = Password::from("abcdefgh");
        assert!(!pw.has_two_different_nonoverlapping_pairs());
    }

    #[test]
    fn pairs_aaaaaaaa() {
        let pw = Password::from("aaaaaaaa");
        assert!(!pw.has_two_different_nonoverlapping_pairs());
    }

    #[test]
    fn pairs_aabcdaaz() {
        let pw = Password::from("aabcdaaz");
        assert!(!pw.has_two_different_nonoverlapping_pairs());
    }


}