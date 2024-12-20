use std::{
    fs::File,
    io::{BufRead, BufReader},
    process::Output,
    usize,
};

use clap::Parser;
use rustc_hash::FxHashSet;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE", required = true)]
    file: String,
}
#[derive(Default, Debug)]
struct CPU {
    a: i64,
    b: i64,
    c: i64,
    out: Vec<i64>,
    pc: usize,
    program: Vec<i64>,
}
fn main() {
    let cli = Cli::parse();
    let file_path = cli.file;

    let mut cpu = CPU {
        ..Default::default()
    };
    cpu.startup(&file_path);

    // part one
    //println!("CPU: {:?}", cpu);
    cpu.run();
    cpu.print_output();

    //part two
    let mut cpu = CPU {
        ..Default::default()
    };
    cpu.startup(&file_path);
    let a = cpu.find_quine();
    println!("A: {}", a)
}

#[derive(Debug)]
enum Instruction {
    ADV,
    BXL,
    BST,
    JNZ,
    BXC,
    OUT,
    BDV,
    CDV,
}

impl TryFrom<i64> for Instruction {
    type Error = ();
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        match v {
            x if x == Instruction::ADV as i64 => Ok(Instruction::ADV),
            x if x == Instruction::BXL as i64 => Ok(Instruction::BXL),
            x if x == Instruction::BST as i64 => Ok(Instruction::BST),
            x if x == Instruction::JNZ as i64 => Ok(Instruction::JNZ),
            x if x == Instruction::BXC as i64 => Ok(Instruction::BXC),
            x if x == Instruction::OUT as i64 => Ok(Instruction::OUT),
            x if x == Instruction::BDV as i64 => Ok(Instruction::BDV),
            x if x == Instruction::CDV as i64 => Ok(Instruction::CDV),
            _ => Err(()),
        }
    }
}

impl CPU {
    fn startup(&mut self, file_path: &String) {
        let file = File::open(file_path).unwrap();
        let content = BufReader::new(file);
        let lines: Vec<String> = content
            .lines()
            .map(|line| line.expect("Something went wrong"))
            .collect();
        self.a = lines[0]
            .split(": ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>()[1]
            .parse::<i64>()
            .unwrap();
        self.b = lines[1]
            .split(": ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>()[1]
            .parse::<i64>()
            .unwrap();
        self.c = lines[2]
            .split(": ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>()[1]
            .parse::<i64>()
            .unwrap();
        self.program = lines[4]
            .split(": ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>()[1]
            .split(',')
            .map(|i| i.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
    }

    fn run(&mut self) {
        while (self.pc as usize) < self.program.len() - 1 {
            //println!("PC: {}", self.pc);
            let current_instruction =
                Instruction::try_from(self.program[self.pc as usize]).unwrap();
            match current_instruction {
                Instruction::ADV => self.adv(self.program[self.pc + 1]),
                Instruction::BXL => self.bxl(self.program[self.pc + 1]),
                Instruction::BST => self.bst(self.program[self.pc + 1]),
                Instruction::JNZ => self.jnz(self.program[self.pc + 1]),
                Instruction::BXC => self.bxc(self.program[self.pc + 1]),
                Instruction::OUT => self.out(self.program[self.pc + 1]),
                Instruction::BDV => self.bdv(self.program[self.pc + 1]),
                Instruction::CDV => self.cdv(self.program[self.pc + 1]),
            }
        }
    }

    fn find_quine(&mut self) -> usize {
        let mut quines = FxHashSet::default();
        quines.insert(0);
        for instruction in self.program.clone().iter().rev() {
            let mut new_quines = FxHashSet::default();
            for curr in quines {
                for i in 0..8 {
                    let new = (curr << 3) + i;

                    let out = self.run_single_out(new);
                    //println!("Run({}): {}", new, out);
                    if out == *instruction {
                        new_quines.insert(new);
                    }
                }
            }
            quines = new_quines;
        }
        println!("Quines: {:?}", quines);
        return *quines.iter().min().unwrap() as usize;
    }

    fn run_single_out(&mut self, initial_a: i64) -> i64 {
        self.pc = 0;
        self.b = 0;
        self.c = 0;
        self.out.clear();

        self.a = initial_a;
        self.run();
        //self.print_output();
        return self.out[0];
    }

    fn print_output(&self) {
        print!("Output: [");
        for i in 0..self.out.len() - 1 {
            print!("{},", self.out[i])
        }
        print!("{}]\n", self.out[self.out.len() - 1])
    }

    fn adv(&mut self, operand: i64) {
        self.a = self.a / ((2 as i64).pow(self.get_combo_operand(operand).unwrap() as u32));
        self.pc += 2;
    }

    fn bxl(&mut self, operand: i64) {
        self.b = self.b ^ operand;
        self.pc += 2;
    }

    fn bst(&mut self, operand: i64) {
        self.b = self.get_combo_operand(operand).unwrap() % 8;
        self.pc += 2;
    }

    fn jnz(&mut self, operand: i64) {
        if self.a != 0 {
            self.pc = operand as usize;
        } else {
            self.pc += 2;
        }
    }

    fn bxc(&mut self, _operand: i64) {
        self.b = self.b ^ self.c;
        self.pc += 2;
    }

    fn out(&mut self, operand: i64) {
        self.out.push(self.get_combo_operand(operand).unwrap() % 8);
        self.pc += 2;
    }

    fn bdv(&mut self, operand: i64) {
        self.b = self.a / ((2 as i64).pow(self.get_combo_operand(operand).unwrap() as u32));
        self.pc += 2;
    }

    fn cdv(&mut self, operand: i64) {
        self.c = self.a / ((2 as i64).pow(self.get_combo_operand(operand).unwrap() as u32));
        self.pc += 2;
    }

    fn get_combo_operand(&self, operand: i64) -> Result<i64, &str> {
        match operand {
            0..=3 => return Ok(operand),
            4 => return Ok(self.a),
            5 => return Ok(self.b),
            6 => return Ok(self.c),
            _ => return Err("Match not defined"),
        }
    }
}
