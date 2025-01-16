use itertools::Itertools;

use std::convert::TryInto;

advent_of_code::solution!(17);

#[repr(u8)]
#[derive(PartialEq, Eq)]
enum Opcode {
    Adv = 0,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl TryFrom<&u8> for Opcode {
    type Error = ();

    fn try_from(v: &u8) -> Result<Self, Self::Error> {
        match *v {
            op if op == Opcode::Adv as u8 => Ok(Opcode::Adv),
            op if op == Opcode::Bxl as u8 => Ok(Opcode::Bxl),
            op if op == Opcode::Bst as u8 => Ok(Opcode::Bst),
            op if op == Opcode::Jnz as u8 => Ok(Opcode::Jnz),
            op if op == Opcode::Bxc as u8 => Ok(Opcode::Bxc),
            op if op == Opcode::Out as u8 => Ok(Opcode::Out),
            op if op == Opcode::Bdv as u8 => Ok(Opcode::Bdv),
            op if op == Opcode::Cdv as u8 => Ok(Opcode::Cdv),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Registers {
    a: u64,
    b: u64,
    c: u64,
}

fn parse_input(input: &str) -> (Registers, Vec<u8>) {
    let lines: Vec<_> = input.lines().collect();
    let registers = Registers {
        a: lines[0][12..].parse().unwrap(),
        b: lines[1][12..].parse().unwrap(),
        c: lines[2][12..].parse().unwrap(),
    };
    let program = lines[4][9..]
        .split(',')
        .map(|op| op.parse().unwrap())
        .collect();
    (registers, program)
}

fn combo_operand(operand: u8, registers: &Registers) -> u64 {
    match operand {
        0..=3 => operand as u64,
        4 => registers.a,
        5 => registers.b,
        6 => registers.c,
        _ => panic!(),
    }
}

fn emulator_loop(program: &[u8], registers: &mut Registers) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::with_capacity(50);
    let mut ip = 0;

    loop {
        let Some(inst) = program.get(ip) else {
            return out;
        };
        let Some(operand_val) = program.get(ip + 1) else {
            return out;
        };
        let operand = *operand_val;
        let opcode = inst.try_into().unwrap();
        if opcode != Opcode::Jnz {
            ip += 2;
        }

        match opcode {
            Opcode::Adv => {
                registers.a /= 1 << combo_operand(operand, registers);
            }
            Opcode::Bxl => {
                registers.b ^= operand as u64;
            }
            Opcode::Bst => {
                registers.b = combo_operand(operand, registers) % 8;
            }
            Opcode::Jnz => {
                if registers.a != 0 {
                    ip = operand as usize;
                } else {
                    ip += 2;
                }
            }
            Opcode::Bxc => {
                registers.b ^= registers.c;
            }

            Opcode::Out => {
                out.push((combo_operand(operand, registers) % 8) as u8);
            }
            Opcode::Bdv => {
                registers.b = registers.a / (1 << combo_operand(operand, registers));
            }
            Opcode::Cdv => {
                registers.c = registers.a / (1 << combo_operand(operand, registers));
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let (mut registers, program) = parse_input(input);

    Some(
        emulator_loop(&program, &mut registers)
            .into_iter()
            .join(","),
    )
}

fn single_loop(registers: &Registers, program: &[u8]) -> Option<Registers> {
    let mut r = *registers;

    let Some(&out) = program.last() else {
        return Some(r);
    };

    for a in 0..8 {
        r.a = registers.a << 3 | a;
        r.b = r.a % 8;
        r.c = r.a >> (r.b ^ 7);
        r.b ^= r.c;
        if (r.b % 8) as u8 == out {
            if let Some(registers) = single_loop(&r, &program[..program.len() - 1]) {
                return Some(registers);
            }
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut registers, program) = parse_input(input);

    registers.a = 0;
    let register_solution = single_loop(&registers, &program).unwrap();

    // check if found solution is valid
    // let mut register_check = register_solution;
    // let out = emulator_loop(&program, &mut register_check);
    // assert_eq!(program, out);

    Some(register_solution.a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let _result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
    }
}
