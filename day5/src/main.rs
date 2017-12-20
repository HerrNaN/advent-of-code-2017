use std::io::{self, Read};

fn read_input() -> Vec<i32>{
    let mut buffer: String = String::new();
	io::stdin().read_to_string(&mut buffer).unwrap();
    buffer
        .lines()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn part_1(input: &mut Vec<i32>) -> u32 {
    let mut current_index: i32 = 0;
    let list_length: i32 = input.len() as i32;
    let mut jumps: u32 = 0;

    loop {
        jumps += 1;
        match current_index + input[current_index as usize] {
            x if x < list_length => {
                input[current_index as usize] += 1;
                current_index += input[current_index as usize] - 1;
            },
            x if x >= list_length => {
                break;
            },
            _ => panic!("Something went terrebly wrong!")
        }
    }
    return jumps;
}

fn part_2(input: &mut Vec<i32>) -> u32 {
    let mut current_index: i32 = 0;
    let list_length: i32 = input.len() as i32;
    let mut jumps: u32 = 0;

    loop {
        jumps += 1;
        match current_index + input[current_index as usize] {
            x if x < list_length => {
                if input[current_index as usize] > 2 {
                    input[current_index as usize] -= 1;
                    current_index += input[current_index as usize] + 1;
                } else {
                    input[current_index as usize] += 1;
                    current_index += input[current_index as usize] - 1;
                }
                
            },
            x if x >= list_length => {
                break;
            },
            _ => panic!("Something went terrebly wrong!")
        }
    }
    return jumps;
}

fn main() {
    let input = read_input();

    println!("It takes {} steps for part 1", part_1(&mut input.clone()));
    println!("It takes {} steps for part 2", part_2(&mut input.clone()))
}
