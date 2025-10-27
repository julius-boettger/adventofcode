// doesn't work, too slow, didn't find answer after 12 hours

type Reg = u64;
#[derive(Debug)]
struct Registers {
    a: Reg,
    b: Reg,
    c: Reg
}

fn combo_operand(operand: &u8, regs: &Registers) -> Reg {
    match operand {
        0 ..= 3 => (*operand).into(),
        4 => regs.a,
        5 => regs.b,
        6 => regs.c,
        _ => panic!()
    }
}

fn execute_instruction(opcode: &u8, operand: &u8, regs: &mut Registers, out: &mut Vec<u8>, pc: &mut u8) {
    match opcode {
        0 => {
            regs.a /= 1 << combo_operand(operand, regs);
            *pc += 2;
        },
        1 => {
            regs.b ^= *operand as Reg;
            *pc += 2;
        },
        2 => {
            regs.b = combo_operand(operand, regs) % 8;
            *pc += 2;
        },
        3 => {
            if regs.a != 0 {
                *pc = *operand;
            } else {
                *pc += 2;
            }
        },
        4 => {
            regs.b ^= regs.c;
            *pc += 2;
        },
        5 => {
            out.push((combo_operand(operand, regs) % 8) as u8);
            *pc += 2;
        },
        6 => {
            regs.b = regs.a / (1 << combo_operand(operand, regs));
            *pc += 2;
        },
        7 => {
            regs.c = regs.a / (1 << combo_operand(operand, regs));
            *pc += 2;
        }
        _ => panic!()
    }
}

fn main() {
    let input = advent_of_code::input!();
    let input_lines: Vec<&str> = input.lines().collect();

    let mut regs = Registers { a: 0, b: 0, c: 0 };
    regs.b = input_lines[1].replace("Register B: ", "").parse().unwrap();
    regs.c = input_lines[2].replace("Register C: ", "").parse().unwrap();

    let program: Vec<u8> = input_lines[4]
        .replace("Program: ", "")
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let mut initial_a: Reg = 0;
    'a_loop: loop {

        if initial_a % 1e8 as Reg == 0 {
            println!("attempting a = {}", initial_a);
        }

        regs.a = initial_a;
        regs.b = 0;
        regs.c = 0;

        // output of program
        let mut out: Vec<u8> = vec![];

        // program counter
        let mut pc: u8 = 0;

        'pc_loop: while pc < (program.len() - 1) as u8 {
            let opcode = program[pc as usize];
            let operand = program[(pc + 1) as usize];
            execute_instruction(&opcode, &operand, &mut regs, &mut out, &mut pc);

            // only continue execution if output matches program (so far)
            for i in 0 .. out.len() {
                if out[i] != program[i] {
                    break 'pc_loop;
                }
            }

            // output is the same as program!
            if out.len() == program.len() {
                break 'a_loop;
            }
        }

        initial_a += 1;
    }

    println!("match found: a = {}", initial_a);
}