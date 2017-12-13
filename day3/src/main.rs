use std::collections::HashMap;

fn count_steps(number: i64) -> i64 {

    let mut prev_i: i64 = 1;
    let mut i: i64 = 1;
    let mut size: i64;

    loop {
        size = (2 * i + 1).pow(2);
        if size >= number && number > prev_i {
            break;
        }
        prev_i = size;
        i += 1;
    }
    
    let diff = size - number;
    let offs = diff % (2 * i);
    let quadrant = diff / (2 * i);

    println!("Diff: {}    Offset: {}    Corner: {}", diff, offs, quadrant);

    let coord = match quadrant {
        0 => ( i - offs, -i       ),
        1 => (-i       , -i + offs),
        2 => (-i + offs,  i       ),
        3 => ( i       ,  i - offs),
        _ => { panic!("Shits broken, yo"); }
    };

    println!("Number is on layer {} with coordinates ({}, {})", i, coord.0, coord.1);

    let steps = coord.0.abs() + coord.1.abs();
    println!("Number of steps: {}", steps);

    return steps;
}

fn find_larger_tile(target: u64) -> u64 {
    let mut tiles: HashMap<(i32, i32), u64> = HashMap::new();

    tiles.insert((0, 0), 1);

    let mut coord = (0, 0);
    let mut dir = (1, 0);
    let mut turns: u32 = 0;
    let mut value: u64 = 1;
    let mut max_steps: u32 = 1;
    let mut steps = 1;

    while value < target {
        coord.0 += dir.0;
        coord.1 += dir.1;

        value = 0;

        //print!("stepped to ({}, {})", coord.0, coord.1);

        for xm in -1..2 {
            for ym in -1..2 {
                let adjacent = (
                    coord.0 + xm,
                    coord.1 + ym,
                );
                match tiles.get(&adjacent) {
                    None => {},
                    Some(tile) => {
                        //print!(", found adjacent ({}, {})->{}", adjacent.0, adjacent.1, tile);

                        value += tile;
                    }
                }
            }
        }

        //println!(", has value {}", value);

        tiles.insert(coord, value);

        steps -= 1;
        if steps == 0 {
            turns += 1;
            if turns % 2 == 0 {
                max_steps += 1;
            }
            steps = max_steps;
            dir = match dir {
                ( 1,  0) => ( 0,  1),
                ( 0,  1) => (-1,  0),
                (-1,  0) => ( 0, -1),
                ( 0, -1) => ( 1,  0),
                _ => { panic!("oh noes"); }
            }
        }
    }

    println!("First value larger than {} is {}", target, value);

    return value;
}

fn main() {
    let input = 265149;

    count_steps(input);
    find_larger_tile(input as u64);
}
