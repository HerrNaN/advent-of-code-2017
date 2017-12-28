use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operation {
    Inc,
    Dec,
}

fn get_op(s: &str) -> Option<Operation> {
    match s {
        "inc" => Some(Operation::Inc),
        "dec" => Some(Operation::Dec),
        _ => None
    }
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instruction{
    reg: String,
    op: Operation,
    amount: i32,
    bool_expr: String
}

impl Instruction {
    fn new() -> Instruction {
        Instruction {
            reg: "unnamed".to_string(),
            op: Operation::Inc,
            amount: 0i32,
            bool_expr: "no expression set".to_string()
        }
    }

    pub fn from_string(s: &str) -> Instruction {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let mut instr: Instruction = Instruction::new();

        instr.reg = parts[0].to_string();
        instr.op = get_op(parts[1]).expect("Couldn't parse operation!");
        instr.amount = parts[2].parse::<i32>().expect("Couldn't parse amount!");
        instr.bool_expr = parts[4..].join(" ");
        
        return instr.clone();
    }

    fn evaluate_bool_expr(&self, registers: &mut HashMap<String, i32>) -> bool {
        let parts: &Vec<&str> = &self.bool_expr.split_whitespace().collect();
        let tmp = registers.clone();
        let first_val: i32 = match tmp.get(&parts[0].to_string()) {
            Some(val) => val.clone(),
            None => {
                registers.insert(parts[0].to_string(), 0);
                0
            }
        };
        let second_val: i32 = parts[2].parse::<i32>().expect("Couldn't parse string!");
        match parts[1] {
            "<"     => first_val < second_val,
            ">"     => first_val > second_val,
            ">="    => first_val >= second_val,
            "<="    => first_val <= second_val,
            "=="    => first_val == second_val,
            "!="    => first_val != second_val,
            &_ => panic!("Boolean operation not recognized!")
        }

    }

    pub fn exec(&self, registers: &mut HashMap<String, i32>) -> Option<i32>{
        if self.evaluate_bool_expr(registers) {
            let tmp = registers.clone();
            let previous_val: i32 = match tmp.get(&self.reg) {
                Some(val) => val.clone(),
                None => {
                    registers.insert(self.reg.clone(), 0);
                    0
                }
            };

            let new_val: i32 = match &self.op {
                &Operation::Inc => previous_val + self.amount.clone(),
                &Operation::Dec => previous_val - self.amount.clone(),
            };
            registers.insert(self.reg.clone(), new_val);
            return Some(new_val);
        } else {
            return None;
        }
    }
}
