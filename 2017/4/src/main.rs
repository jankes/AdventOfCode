use std::collections::HashSet;
use std::fs;

fn main() {    
    let all_passphrases = fs::read_to_string("C:\\Users\\jankes\\Documents\\AdventOfCode\\2017\\4\\passphrases.txt")
                              .expect("couldn't read passphrases file to a string");

    let valid_passphrase_count = count_valid_passphrases_part_1(&all_passphrases);
    println!("part 1: there are {} valid passphrases", valid_passphrase_count);

    let valid_passphrase_count = count_valid_passphrases_part_2(&all_passphrases);
    println!("part 2: there are {} valid passphrases", valid_passphrase_count);
}

fn count_valid_passphrases_part_1(all_phrases: &str) -> u16 {
    let mut set = HashSet::<&str>::with_capacity(20);
    all_phrases
    .lines()
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

fn count_valid_passphrases_part_2(all_phrases: &str) -> u16 {
    let mut words = Vec::<&[u8]>::with_capacity(20);
    all_phrases
    .lines()
    .map(|phrase| {
        words.clear();
        for word in phrase.split_whitespace().map(|p| p.as_bytes()) {
            words.push(word);
        }
        for (i, first_word) in words.iter().enumerate() {
            for second_word in words.iter().skip(i + 1) {
                if are_anagrams(first_word, second_word) {
                    return 0;
                }
            }
        }
        return 1;
    })
    .sum::<u16>()
}

fn are_anagrams(first_word: &[u8], second_word: &[u8]) -> bool {
    if first_word.len() != second_word.len() {
        return false;
    }
    let mut first_word_sorted = Vec::with_capacity(first_word.len());
    first_word_sorted.extend_from_slice(first_word);
    first_word_sorted.sort();

    let mut second_word_sorted = Vec::with_capacity(second_word.len());
    second_word_sorted.extend_from_slice(second_word);
    second_word_sorted.sort();

    return first_word_sorted == second_word_sorted
}