use std::io::{self, Read};
use std::collections::HashSet;

fn has_duplicates(passphrase: &Vec<String>) -> bool {
    if let Some((first, rest)) = passphrase.split_first() {
        if rest.contains(first) {
            return true;
        } else {
            return has_duplicates(&rest.to_vec());
        }
    }
    return false;
}

fn has_anagrams(passphrase: &Vec<String>) -> bool {
    if let Some((first, rest)) = passphrase.split_first() {
        if contains_anagram_of(first, rest.to_vec()) {
            return true;
        } else {
            return has_anagrams(&rest.to_vec());
        }
    }
    return false;
}

fn contains_anagram_of(word: &String, v: Vec<String>) -> bool {
    for w in v.into_iter() {
        if is_anagram(word, &w) {
            return true;
        }
    }
    return false;
}

fn is_anagram(word1: &String, word2: &String) -> bool {
    if word1.len() != word2.len() {
        return false;
    }

    let mut hs1: HashSet<char> = HashSet::new();
    let mut hs2: HashSet<char> = HashSet::new();

    for c in word1.chars() {
        hs1.insert(c);
    }
    for c in word2.chars() {
        hs2.insert(c);
    }
    return hs1 == hs2;
}

fn read_input() -> Vec<Vec<String>> {
	let mut buffer: String = String::new();
	io::stdin().read_to_string(&mut buffer).unwrap();
    buffer.lines()
        .map(|l| l.split_whitespace()
            .map(|s| String::from(s))
            .collect())
        .collect()
}

fn part_1(input: &Vec<Vec<String>>) -> u32 {
    input
        .into_iter()
        .filter(|line| !has_duplicates(line))
        .count() as u32
}

fn part_2(input: &Vec<Vec<String>>) -> u32 {
    input
        .into_iter()
        .filter(|line| !has_anagrams(line))
        .count() as u32
}

fn main() {
    let input = read_input();
    println!("In part 1 {} passphrases are invalid", part_1(&input));
    println!("In part 2 {} passphrases are invalid", part_2(&input));
}
