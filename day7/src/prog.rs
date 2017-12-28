#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Prog {
    pub name: String,
    pub weight: u32,
    pub subs: Vec<String>,
}

impl Prog {
    pub fn new() -> Prog {
        Prog {
            name: String::from("unnamed"),
            weight: 0,
            subs: vec![]
        }
    }

    pub fn from_string(s: &str) -> Prog {
        let mut tmp: Prog = Prog::new();
        let mut v = s.split_whitespace()
            .map(|s| String::from(s))
            .collect::<Vec<String>>();
        tmp.name = v[0].clone();
        tmp.weight = v[1]
            .replace("(", "").replace(")", "")
            .parse::<u32>().unwrap();
        if v.len() > 3 {
            tmp.subs = v.drain(3..)
                .map(|s| String::from(s.replace(",", "").trim()))
                .collect();
        }
        return tmp;
    }
}