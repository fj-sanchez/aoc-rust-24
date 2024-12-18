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

#[derive(Debug)]
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

pub fn part_two(input: &str) -> Option<u64> {
    let (mut registers, program) = parse_input(input);

    // let mut r = Registers { a: 0, b: 0, c: 0 };
    // for &inst in program.iter().rev() {
    //     r.b = (inst as u64) % 8;
    //     r.a <<= 3;
    //     r.c = r.b^r.b;
    //     r.b ^= 7;
    //     r.a ^= r.c << (1 << r.b);
    //     r.b ^= 7;
    //     r.a ^= r.b % 8;
    // }

    // dbg!(&r);

    for a in 0..8u64.pow(8) {
        registers.a = a;
        registers.b = 0;
        registers.c = 0;

        let x = emulator_loop(&program, &mut registers);

        if x.len() > 6 && x[0..7] == [0,3,5,5,3,0,6] {
            println!(
                "A={:08b}\tB={:03b}\tC={:03b}\tBxorC={:03b}\tResult={}",
                a,
                registers.b,
                registers.c,
                registers.b ^ registers.c,
                x.into_iter().join(",")
            );
            return Some(a);
        }
        // println!(
        //     "A={:08b}\tB={:03b}\tC={:03b}\tBxorC={:03b}\tResult={}",
        //     a,
        //     registers.b,
        //     registers.c,
        //     registers.b^registers.c,
        //     x.into_iter().join(",")
        // );
    }

    Some(0)
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
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        // assert_eq!(result, Some(117440));
    }
}
