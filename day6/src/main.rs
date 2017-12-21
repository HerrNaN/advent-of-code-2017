use std::io::{self, Read};
use std::collections::{HashSet, HashMap};

fn read_input() -> Vec<u32> {
    let mut buffer: String = String::new();
	io::stdin().read_to_string(&mut buffer).unwrap();
    buffer
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn find_max_index(v: &Vec<u32>) -> usize {
    let clone: Vec<u32> = v.clone();
    let mut max: u32 = 0;
    let mut max_index: usize = 0;
    for i in 0..clone.len() {
        if v[i] > max {
            max = clone[i];
            max_index = i;
        }
    }
    return max_index;
}

fn distribute_from(buffer: &u32, index: &usize, v: &mut Vec<u32>) {
    let mut ind: usize = index.clone();
    let n: usize = buffer.clone() as usize;
    let v_len: usize = v.len();
    for _i in 0..n {
        v[ind % v_len] += 1;
        ind += 1;
    }
}

fn part_1(input: &mut Vec<u32>) -> u32 {
    let mut set: HashSet<Vec<u32>> = HashSet::new();
    let mut cycles: u32 = 0;
    let mut buffer: u32 = 0;
    let mut max_index: usize = 0; 
    loop {
        cycles += 1;
        max_index = find_max_index(input);
        buffer = input[max_index];
        input[max_index] = 0;
        distribute_from(&buffer, &(max_index + 1), input);
        if !set.insert(input.clone()) {
            return cycles;
        }
    }
}

fn part_2(input: &mut Vec<u32>) -> u32 {
    let mut set: HashMap<Vec<u32>, u32> = HashMap::new();
    let mut cycles: u32 = 0;
    let mut buffer: u32 = 0;
    let mut max_index: usize = 0; 
    loop {
        cycles += 1;
        max_index = find_max_index(input);
        buffer = input[max_index];
        input[max_index] = 0;
        distribute_from(&buffer, &(max_index + 1), input);
        if set.contains_key(&input.clone()) {
            return cycles - set.get(&input.clone()).unwrap();
        }
        set.insert(input.clone(), cycles);
    }
}

fn main() {
    let input: Vec<u32> = read_input();
    println!("{} cycles must be completed in part 1", part_1(&mut input.clone()));
    println!("{} cycles will be repeated in part 2", part_2(&mut input.clone()));
}
