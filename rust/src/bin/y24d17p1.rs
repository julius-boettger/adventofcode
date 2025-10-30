type Reg = u32;
#[derive(Debug)]
struct Registers {
    a: Reg,
    b: Reg,
    c: Reg
}

#[allow(clippy::enum_glob_use)]
use Opcode::*;
#[derive(num_enum::TryFromPrimitive, Debug)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
enum Opcode {
    /// a = a / 2.pow((combo operand))
    ADV,
    /// b = b ^ operand (XOR)
    BXL,
    /// b = (combo operand) % 8
    BST,
    /// if a != 0 { pc = operand }
    JNZ,
    /// b = b ^ c (XOR)
    BXC,
    /// out.push((combo operand) % 8)
    OUT,
    /// b = a / 2.pow((combo operand))
    BDV,
    /// c = a / 2.pow((combo operand))
    CDV
}

fn combo_operand(operand: u8, regs: &Registers) -> Reg {
    match operand {
        0 ..= 3 => operand.into(),
        4 => regs.a,
        5 => regs.b,
        6 => regs.c,
        _ => panic!()
    }
}

fn execute_instruction(opcode: &Opcode, operand: u8, regs: &mut Registers, out: &mut Vec<u8>, pc: &mut u8) {
    match opcode {
        ADV => {
            regs.a /= (2 as Reg).pow(combo_operand(operand, regs));
            *pc += 2;
        },
        BXL => {
            regs.b ^= Reg::from(operand);
            *pc += 2;
        },
        BST => {
            regs.b = combo_operand(operand, regs) % 8;
            *pc += 2;
        },
        JNZ => {
            if regs.a != 0 {
                *pc = operand;
            } else {
                *pc += 2;
            }
        },
        BXC => {
            regs.b ^= regs.c;
            *pc += 2;
        },
        OUT => {
            out.push((combo_operand(operand, regs) % 8) as u8);
            *pc += 2;
        },
        BDV => {
            regs.b = regs.a / (2 as Reg).pow(combo_operand(operand, regs));
            *pc += 2;
        },
        CDV => {
            regs.c = regs.a / (2 as Reg).pow(combo_operand(operand, regs));
            *pc += 2;
        }
    }
}

#[advent_of_code::main]
fn main() {
    let input_lines: Vec<&str> = include_str!("../../input/24/17.txt").lines().collect();

    let mut regs = Registers { a: 0, b: 0, c: 0 };
    regs.a = input_lines[0].replace("Register A: ", "").parse().unwrap();
    regs.b = input_lines[1].replace("Register B: ", "").parse().unwrap();
    regs.c = input_lines[2].replace("Register C: ", "").parse().unwrap();

    let program: Vec<u8> = input_lines[4]
        .replace("Program: ", "")
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    // program counter
    let mut pc: u8 = 0;
    // output of program
    let mut out: Vec<u8> = vec![];

    while pc < u8::try_from(program.len() - 1).unwrap() {
        let opcode = Opcode::try_from(program[pc as usize]).unwrap();
        let operand = program[(pc + 1) as usize];
        execute_instruction(&opcode, operand, &mut regs, &mut out, &mut pc);
    }

    println!("{}", out
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(","));
}