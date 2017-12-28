use std::io::{self, Read};

fn read_input() -> Vec<usize> {
    let mut buffer: String = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Couldn't read!");
    buffer.split(",")
        .map(|s| s.parse::<usize>().expect("Couldn't parse into u8!"))
        .collect()
}

/* Maybe create own reverse with slices?
 * Maybe make use of rotate when it gets into std? (or use nightly?)
 */
fn reverse(list: &Vec<&u8>, current_pos: usize, length: usize) {
    let mut new_pos: usize = current_pos + length;
    let mut slice: &Vec<&u8> = &vec![];
    if new_pos < list.len() {
        slice.extend(list[current_pos..new_pos].iter());
    } else {
        new_pos = new_pos % list.len();
        // 
    }
}

fn part_1(lengths: &Vec<usize>, list: &mut Vec<&u8>) -> u32 {
    unimplemented!();
}

fn main() {
    let input: Vec<usize> = read_input();
    let mut list: Vec<&u8> = (0..6).iter().map(|&i| i).collect();
    //println!("{:?}", input);
    //println!("{:?}", list);
    println!("The product of the first two numbers are: {}", part_1(&input, &mut list));

}
