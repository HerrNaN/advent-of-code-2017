use std::io::{self, Read};

fn main()
{
	let mut buffer: Vec<u8> = Vec::new();
	io::stdin().read_to_end(&mut buffer).unwrap();

	let input = buffer.iter()
		// Filter out incorrect inputs
		.filter(|&x| *x > 48 && *x <= 58)
		// Convert ascii values to numbers
		.map(|&x| x - 48)
		.collect::<Vec<u8>>();

	// Offset from one element to the element it will be compared to
	let offset = input.len() / 2;
	//let offset = 1;
	
	let mut sum: u32 = 0;
	for i in 0..input.len()
	{
		// Check if element matches the next one
		if input[i] == input[(i + offset) % input.len()] 
		{
			println!(
				"Found double. {} at {} equals {} at {}",
				input[i],
				i,
				input[(i+1) % input.len()],
				(i + offset) % input.len()
			);

			sum += input[i] as u32;
		}
	}
	println!("Sum is {}", sum);
}