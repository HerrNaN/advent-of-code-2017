use std::io::{self, Read};

fn get_minmax_diff(numbers: &mut Iterator<Item = &u32>, min_max: (u32, u32)) -> Result<(u32, u32), String> {
    Ok(match numbers.next() {
        None => min_max,
        Some(num) => {
            get_minmax_diff(numbers, match num {
                num if *num > min_max.1 => (min_max.0, *num),
                num if *num < min_max.0 => (*num, min_max.1),
                _ => min_max,
            })?
        },
    })
}

fn get_even_diff(row: &Vec<u32>) -> Result<u32, String> {

    for i in 0..row.len() {
        for j in i+1..row.len() {
            if row[i] % row[j] == 0 {
                return Ok(row[i] / row[j]);
            }
        }
    }
    Err(String::from("Could not find two number that divide eachother!"))
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).ok().expect("Could not read input");

    let mut min_max_sum: u32 = 0;
    let mut div_sum: u32 = 0;

    for l in buffer.lines() {
        let mut numbers: Vec<u32> =
            l.split_whitespace()
            .map(|x| x.parse::<u32>().ok().expect("Could not parse number"))
            .collect();

        numbers.sort();
        numbers.reverse();

        div_sum += get_even_diff(&mut numbers).unwrap();

        let min_max_res = get_minmax_diff(&mut numbers.iter(), (u32::max_value(),0)).ok().expect("Could not get ");
        min_max_sum += min_max_res.1 - min_max_res.0;
    }
    println!("The min_max sum is: {}", min_max_sum);
    println!("The division sum is: {}", div_sum);
}
