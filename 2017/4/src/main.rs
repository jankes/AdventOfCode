use std::borrow::Borrow;
use std::collections::HashSet;
use std::fs;

fn main() {    
    let valid_passphrase_count = count_valid_passphrases(
        fs::read_to_string("C:\\Users\\jankes\\Documents\\AdventOfCode\\2017\\4\\passphrases.txt")
            .expect("couldn't read passphrases file to a string")
            .borrow()
    );
    println!("There are {} valid passphrases", valid_passphrase_count);
}

fn count_valid_passphrases(all_phrases: &str) -> u16 {
    let mut set = HashSet::<&str>::with_capacity(20);
    all_phrases.lines()
    .map(|phrase| {
        set.clear();
        for word in phrase.split_whitespace() {
            if !set.insert(word) { // If the set did have this value present, false is returned
                return 0u16;
            }
        }
        return 1u16;
    })
    .sum::<u16>()
}
