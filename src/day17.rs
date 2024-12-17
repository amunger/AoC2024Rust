use std::usize;

use crate::file_input;

struct Interpreter {
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
    ip: i8,
    program: Vec<i8>,
}

impl Interpreter {
    fn print(&self) {
        println!("a: {}\nb: {}\nc: {}\nip: {}", self.reg_a, self.reg_b, self.reg_c, self.ip);
        println!("{:?}", self.program);
        let dash_count = (self.ip as usize) * 3 + 1;
        print!("{}", "-".repeat(dash_count));
        print!("^");
    }
}

fn parse_input(input: &Vec<String>) -> Interpreter {
    let a = input[0].split(":").collect::<Vec<&str>>()[1].trim();
    let b = input[1].split(":").collect::<Vec<&str>>()[1].trim();
    let c = input[2].split(":").collect::<Vec<&str>>()[1].trim();
    let program_str = input[4].split(':').collect::<Vec<&str>>()[1];
    let program = program_str.split(",").map(|x| x.trim().parse::<i8>().unwrap()).collect::<Vec<i8>>();

    Interpreter { 
        reg_a: a.parse().unwrap(),
        reg_b: b.parse().unwrap(),
        reg_c: c.parse().unwrap(),
        ip: 2,  
        program }
}

pub fn solve(){
    let mut intepreter = parse_input(&file_input::read_input());
    intepreter.print();
}