use std::io::{self, Read};

fn read_input() -> Vec<u8> {
	let mut buffer: Vec<u8> = Vec::new();
	io::stdin().read_to_end(&mut buffer).unwrap();

	buffer.iter()
		// Filter out incorrect inputs
		.filter(|&x| *x > 48 && *x <= 58)
		// Convert ascii values to numbers
		.map(|&x| x - 48)
		.collect::<Vec<u8>>()
}

fn sum_with_offset(input: &Vec<u8>, offset: usize) -> u32 {
	let mut sum: u32 = 0;
	for i in 0..input.len() {
		if input[i] == input[(i + offset) % input.len()] {
			sum += input[i] as u32;
		}
	}
	return sum;
}

fn part_1(input: &Vec<u8>) -> u32 {
	sum_with_offset(input, 1)
}

fn part_2(input: &Vec<u8>, offset: usize) -> u32 {
	sum_with_offset(input, offset)
}

fn main() {
	let input: Vec<u8> = read_input();

	println!("The sum for part 1 is: {}", part_1(&input));
	println!("The sum for part 2 is: {}", part_2(&input, input.len() / 2));
}