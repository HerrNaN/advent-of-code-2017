mod instruction;

use std::io::{self, Read};
use std::collections::{HashMap};

use instruction::{Instruction, Operation};

fn read_input() -> String {
    let mut buffer: String = String::new();
    io::stdin().read_to_string(&mut buffer);
    return buffer;
}


fn part_1(input: &String, registers: &mut HashMap<String, i32>) -> i32 {
    for instruction in input.lines() {
        let instr: Instruction = Instruction::from_string(instruction);
        instr.exec(registers);
    }
    /*for (key, val) in registers.iter() {
        println!("key: {} val: {}", key, val);
    }*/
    registers.values().max().expect("Couldn't find a maximum value!").clone()
}

fn part_2(input: &String, registers: &mut HashMap<String, i32>) -> i32 {
    let mut max_val_ever: i32 = i32::min_value();
    for instruction in input.lines() {
        let instr: Instruction = Instruction::from_string(instruction);
        let new_val = match instr.exec(registers) {
            Some(x) => x,
            None => max_val_ever
        };
        if new_val > max_val_ever {
            max_val_ever = new_val;
        }
    }
    /*for (key, val) in registers.iter() {
        println!("key: {} val: {}", key, val);
    }*/
    max_val_ever
}

fn main() {
    let mut registers: HashMap<String, i32> = HashMap::new();
    let mut input: String = read_input();
    println!("The largest value in any register is: {}", part_1(&input, &mut registers.clone()));
    println!("The largest value ever encountered was: {}", part_2(&input, &mut registers.clone()));
}
