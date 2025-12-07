// doesn't work, too slow, didn't find answer after 12 hours

type Reg = u64;
#[derive(Debug)]
struct Registers {
    a: Reg,
    b: Reg,
    c: Reg
}

const fn combo_operand(operand: u8, regs: &Registers) -> Reg {
    match operand {
        0 ..= 3 => operand as Reg,
        4 => regs.a,
        5 => regs.b,
        6 => regs.c,
        _ => unreachable!()
    }
}

fn execute_instruction(opcode: u8, operand: u8, regs: &mut Registers, out: &mut Vec<u8>, pc: &mut u8) {
    match opcode {
        0 => {
            // same as regs.a /= 2.pow(combo_operand(operand, regs))
            regs.a >>= combo_operand(operand, regs);
            *pc += 2;
        },
        1 => {
            regs.b ^= Reg::from(operand);
            *pc += 2;
        },
        2 => {
            // same as modulo 8
            regs.b = combo_operand(operand, regs) & 7;
            *pc += 2;
        },
        3 => {
            if regs.a != 0 {
                *pc = operand;
            } else {
                *pc += 2;
            }
        },
        4 => {
            regs.b ^= regs.c;
            *pc += 2;
        },
        5 => {
            // same as modulo 8
            out.push((combo_operand(operand, regs) & 7) as u8);
            *pc += 2;
        },
        6 => {
            // same as regs.b = regs.a / 2.pow(combo_operand(operand, regs))
            regs.b = regs.a >> combo_operand(operand, regs);
            *pc += 2;
        },
        7 => {
            // same as regs.c = regs.a / 2.pow(combo_operand(operand, regs))
            regs.c = regs.a >> combo_operand(operand, regs);
            *pc += 2;
        }
        _ => unreachable!()
    }
}

#[advent_of_code::main("24/17")]
fn main() {
    let input_lines: Vec<&str> = INPUT.lines().collect();

    let mut regs = Registers { a: 0, b: 0, c: 0 };
    regs.b = input_lines[1].replace("Register B: ", "").parse().unwrap();
    regs.c = input_lines[2].replace("Register C: ", "").parse().unwrap();

    let program: Vec<u8> = input_lines[4]
        .replace("Program: ", "")
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let mut out = Vec::new();
    let mut pc: u8;
    let mut initial_a: Reg = 0;
    'a_loop: loop {

        // TODO: remove, only for timing purposes
        if initial_a == 100_000_000 {
            break;
        }

        regs.a = initial_a;
        regs.b = 0;
        regs.c = 0;
        pc = 0;
        out.clear(); // keep capacity

        'pc_loop: while pc < u8::try_from(program.len() - 1).unwrap() {
            let opcode = program[pc as usize];
            let operand = program[(pc + 1) as usize];
            execute_instruction(opcode, operand, &mut regs, &mut out, &mut pc);

            if !out.is_empty() {
                // only continue execution if output matches program (so far)
                let last_index = out.len() - 1;
                if out[last_index] != program[last_index] {
                    break 'pc_loop;
                }

                // output is the same as program!
                if out.len() == program.len() {
                    break 'a_loop;
                }
            }
        }

        initial_a += 1;
    }

    println!("match found: a = {initial_a}");
}