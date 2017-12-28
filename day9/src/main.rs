use std::io::{self, Read};

fn read_input() -> String {
    let mut buffer: String = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Couldn't read!");
    return buffer;
}

fn part_1(input: &String) -> u32 {
    let mut group_level: u32 = 0;
    let mut inside_garbage: bool = false;
    let mut after_exclamation: bool = false;
    let mut sum: u32 = 0;

    for c in input.chars() {
        if after_exclamation {
            after_exclamation = false;
        } else if inside_garbage {
            if c == '>' {
                inside_garbage = false;
            } else if c == '!' {
                after_exclamation = true;
            }
        } else {
            if c == '{' {
                group_level += 1;
            } else if c == '}' {
                sum += group_level;
                group_level -= 1;
            } else if c == '<' {
                inside_garbage = true;
            }
        }
    }
    return sum;
}

fn part_2(input: &String) -> u32 {
    let mut group_level: u32 = 0;
    let mut inside_garbage: bool = false;
    let mut after_exclamation: bool = false;
    let mut sum: u32 = 0;
    let mut garbage_characters_count: u32 = 0;

    for c in input.chars() {
        if after_exclamation {
            after_exclamation = false;
        } else if inside_garbage {
            if c == '>' {
                inside_garbage = false;
            } else if c == '!' {
                after_exclamation = true;
            } else {
                garbage_characters_count += 1;
            }
        } else {
            if c == '{' {
                group_level += 1;
            } else if c == '}' {
                sum += group_level;
                group_level -= 1;
            } else if c == '<' {
                inside_garbage = true;
            }
        }
    }
    return garbage_characters_count;
}

fn main() {
    let input: String = read_input();

    println!("The score was: {}", part_1(&input));
    println!("There was {} garbage characters", part_2(&input));
}
