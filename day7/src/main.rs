extern crate regex;

mod prog;

use std::io::{self, Read};
use std::collections::{HashSet, HashMap};
use prog::Prog;

fn read_input() -> String {
    let mut buffer: String = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    return buffer;
}

fn parse_input(input: String) -> HashMap<String, Prog> {
    input.lines()
        .map(|line| Prog::from_string(line))
        .map(|p| (p.name.clone(), p))
        .collect()
}

fn part_1(progs: &HashMap<String, Prog>) -> Prog {
    let mut owners: HashSet<Prog> = HashSet::new();
    let mut owned: HashSet<Prog> = HashSet::new();

    for p in progs.values() {
        if p.subs.len() != 0 {
            owners.insert(p.clone());
            for s in p.clone().subs {
                owned.insert(progs.get(&s).unwrap().clone());
            }
        }
    }

    return owners.difference(&owned).next().unwrap().clone();
}

fn get_weight(prog: &Prog, weights: &mut HashMap<String, u32>, progs: &HashMap<String, Prog>) -> u32 {
    let mut weight: u32 = prog.weight;
    if prog.subs.len() != 0 {
        weight += prog.subs.iter()
            .fold(0u32, |sum, sub| sum + get_weight(progs.get(sub).unwrap(), weights, progs));
    }
    weights.insert(prog.name.clone(), weight);
    return weight;
}

fn get_wrong_weight(prog: &Prog, weights: &HashMap<String, u32>, progs: &HashMap<String, Prog>) -> u32 {
    if prog.subs.len() != 0 {
        if !has_symmetric_sub_weights(prog, weights) {
            let sub_weights: Vec<&u32> = prog.subs.iter()
                .map(|s| weights.get(s).expect("!!!"))
                .collect();

            let sub_weight: &u32 = match sub_weights[1..].contains(&sub_weights[0]) {
                true => weights.get(&prog.subs[0]).expect("Couldn't get prog weight"),
                false => weights.get(&prog.subs[1]).expect("Couldn't get prog weight")
            };
            
            /*for s in &prog.subs {
                println!("sub weight {} has weight {}", s, weights.get(s).expect("Couldn't find sub weight!"));
            }*/

            //println!("prog = {:?}", prog);
            //println!("sub_weight = {}", sub_weight);
            let faulty_prog_name = prog.subs.iter()
                .filter(|&sub| weights.get(sub).expect("Couldn't get sub-weight") != sub_weight)
                .next().expect("Couldn't get name");
            let faulty_prog: &Prog = progs.get(faulty_prog_name).expect("Couldn't get program");

            if !has_symmetric_sub_weights(faulty_prog, weights) {
                return get_wrong_weight(faulty_prog, weights, progs);
            } else {
                //println!("Faulty program = {:?}", faulty_prog);
                let faulty_weight = weights.get(&faulty_prog.name).expect("Couldn't get prog weight").clone();
                //println!("Faulty weight = {}", faulty_weight);
                return faulty_prog.weight + sub_weight - faulty_weight;
            }

        }
    }
    panic!("Something went wrong!");
}

fn has_symmetric_sub_weights(prog: &Prog, weights: &HashMap<String, u32>) -> bool {
    let mut hs: HashSet<u32> = HashSet::new();
    let res = prog.subs.iter()
        .filter(|sub| hs.insert(weights.get(sub.clone()).expect("Couldn't get sub weight").clone()))
        .count() == 1;
    //println!("{} has symetric sub-weights? {}", prog.name, res);
    return res;
}

fn part_2(progs: &HashMap<String, Prog>, bottom_program: &Prog) -> u32 {
    let mut weights: HashMap<String, u32> = HashMap::new();
    get_weight(bottom_program, &mut weights, progs);
    /*for w in &weights {
        println!("{:?}", w);
    }*/
    get_wrong_weight(bottom_program, &weights, progs)
}

fn main() {
    let all_progs: HashMap<String, Prog> = parse_input(read_input());
    let bottom_program: Prog = part_1(&all_progs);
    println!("The bottom program for part 1 is: {:?}", 
        bottom_program.name);
    println!("The faulty program should have weight {}", 
        part_2(&all_progs, &bottom_program));
}
