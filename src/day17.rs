use std::usize;

use crate::file_input;

struct Interpreter {
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
    ip: i8,
    program: Vec<i8>,
    output: Vec<i64>
}

impl Interpreter {
    fn print(&self) {
        println!("\na: {}\nb: {}\nc: {}\nip: {}", self.reg_a, self.reg_b, self.reg_c, self.ip);
        println!("{:?}", self.program);
        let dash_count = (self.ip as usize) * 3 + 1;
        print!("{}", "-".repeat(dash_count));
        println!("^\n");
        self.print_output();
    }

    fn print_registers(&self) {
        println!("a:{:>15} b: {:>15} c: {:>15}", self.reg_a, self.reg_b, self.reg_c);
    }

    fn print_output(&self) {
        println!("output: {}", self.output.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","));
    }

    pub fn execute(&mut self) {
        while self.ip < self.program.len() as i8 {
            //self.print();
            //self.print_registers();
            let opcode = self.program[self.ip as usize];
            match opcode {
                0 => {
                    let operand = self.combo_operand();
                    let divisor: i64 = 2_i64.pow(operand as u32);
                    //println!("adv, {:?} / {:?}", self.reg_a, divisor);
                    self.reg_a = self.reg_a / divisor;
                }
                1 => {
                    let operand = self.literal_operand() as i64;
                    let xor = self.reg_b ^ operand;
                    //println!("bxl, {:?} ^ {:?}", self.reg_b, operand);
                    self.reg_b = xor;
                }
                2 => {
                    let operand = self.combo_operand();
                    self.reg_b = operand % 8;
                    //println!("bst, {:?} % 8", operand);
                }
                3 => {
                    if self.reg_a != 0 {
                        self.ip = self.literal_operand();    
                        //println!("jnz, {:?}", self.ip);
                        continue;
                    }
                    //println!("jnz, no-op");
                }
                4 => {
                    //println!("bxc, {:?} ^ {:?}", self.reg_b, self.reg_c);
                    let xor = self.reg_b ^ self.reg_c;
                    self.reg_b = xor;
                }
                5 => {
                    let operand = self.combo_operand();
                    let mod8 = operand % 8;
                    self.output.push(mod8);
                    //println!("{:?} out, {:?} % 8", mod8, operand);
                }
                6 => {
                    let operand = self.combo_operand();
                    let divisor: i64 = 2_i64.pow(operand as u32);
                    //println!("bdv, {:?} / {:?}", self.reg_a, divisor);
                    self.reg_b = self.reg_a / divisor;
                }
                7 => {
                    let operand = self.combo_operand();
                    let divisor: i64 = 2_i64.pow(operand as u32);
                    //println!("cdv, {:?} / {:?}", self.reg_a, divisor);
                    self.reg_c = self.reg_a / divisor;
             }
                _ => { panic!("Invalid opcode")}
            }
            self.ip = self.ip + 2;   
        }
    }

    fn literal_operand(&self) -> i8 {
        let operand = self.program[(self.ip + 1) as usize];
        operand
    }

    fn combo_operand(&self) -> i64 {
        let operand = self.program[(self.ip + 1) as usize];
        match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => panic!("Invalid operand")
        }
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
        program,
        output: vec![]}
}

pub fn solve(){
    let mut intepreter = parse_input(&file_input::read_input());
    intepreter.execute();
    intepreter.print();
}

pub fn solvept2(){
    let answer = vec![2,4,1,5,7,5,4,5,0,3,1,6,5,5,3,0];
    let mut candidites = vec![0, 1, 2, 3, 4, 5, 6, 7];
    let mut new_candidates = vec![];
    //               109157309200000:
    for i in answer.iter().rev() {
        for c in candidites.iter() {
            let mut intepreter = parse_input(&file_input::read_input());
            println!("{:?}", c);
            intepreter.reg_a = *c;
            intepreter.execute();
            intepreter.print_output();
            if intepreter.output[0] == *i {
                new_candidates.push(*c * 8);
            }
        }
        candidites = new_candidates.clone();
        println!("{:?}", candidites);
        new_candidates.clear();
    }

    println!("{:?}", candidites);
}

//2,4,1,5,7,5,4,5,0,3,1,6,5,5,3,0
//6,5,1,5,7,5,4,5,0,3,1,6,5,5,3,0

// 109019930332928 - 109019930329592
