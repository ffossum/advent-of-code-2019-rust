use digits_iterator::*;
use itertools::Itertools;
use std::convert::TryFrom;
use std::error::Error;

enum Parameter {
    Position(usize),
    Immediate(i32),
}
impl Parameter {
    fn new(mode: u8, value: i32) -> Result<Parameter, Box<dyn Error>> {
        match mode {
            0 => Ok(Parameter::Position(usize::try_from(value)?)),
            1 => Ok(Parameter::Immediate(value)),
            _ => Err(format!("illegal mode: {}", mode))?,
        }
    }
    fn get_value(&self, input: &[i32]) -> i32 {
        match *self {
            Parameter::Position(i) => input[i],
            Parameter::Immediate(value) => value,
        }
    }
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let instructions = vec![
        3, 8, 1001, 8, 10, 8, 105, 1, 0, 0, 21, 34, 51, 76, 101, 114, 195, 276, 357, 438, 99999, 3,
        9, 1001, 9, 3, 9, 1002, 9, 3, 9, 4, 9, 99, 3, 9, 101, 4, 9, 9, 102, 4, 9, 9, 1001, 9, 5, 9,
        4, 9, 99, 3, 9, 1002, 9, 4, 9, 101, 3, 9, 9, 102, 5, 9, 9, 1001, 9, 2, 9, 1002, 9, 2, 9, 4,
        9, 99, 3, 9, 1001, 9, 3, 9, 102, 2, 9, 9, 101, 4, 9, 9, 102, 3, 9, 9, 101, 2, 9, 9, 4, 9,
        99, 3, 9, 102, 2, 9, 9, 101, 4, 9, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9,
        9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3,
        9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1002,
        9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 99, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9,
        4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3,
        9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9,
        2, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 99, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9,
        4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9,
        102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2,
        9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9,
        3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102,
        2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4,
        9, 3, 9, 102, 2, 9, 9, 4, 9, 99, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9,
        101, 1, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1001, 9, 2,
        9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9,
        3, 9, 1002, 9, 2, 9, 4, 9, 99,
    ];

    let amplifiers = vec![0, 1, 2, 3, 4];
    let res = amplifiers
        .iter()
        .permutations(5)
        .filter_map(|sequence| run_amplifier_sequence(instructions.clone(), &sequence).ok())
        .max();

    println!("{:?}", res);
    Ok(())
}

fn run_amplifier_sequence(
    instructions: Vec<i32>,
    sequence: &[&i32],
) -> Result<i32, Box<dyn Error>> {
    let mut output = 0;
    for &x in sequence {
        let outputs = run_program(&mut instructions.clone(), vec![output, *x])?;
        output = outputs.first().copied().ok_or("expected input")?;
    }
    Ok(output)
}

fn get_opcode(instructions: &[i32], i: &mut usize) -> ([u8; 3], i32) {
    let opcode: i32 = instructions[*i];
    *i += 1;

    let reversed_digits = opcode.digits().rev().collect::<Vec<u8>>();

    let mode_1: u8 = reversed_digits.get(2).copied().unwrap_or(0);
    let mode_2: u8 = reversed_digits.get(3).copied().unwrap_or(0);
    let mode_3: u8 = reversed_digits.get(4).copied().unwrap_or(0);

    let opcode = opcode % 100;

    ([mode_1, mode_2, mode_3], opcode)
}

fn get_value(instructions: &[i32], i: &mut usize, mode: u8) -> Result<i32, Box<dyn Error>> {
    let param = Parameter::new(mode, instructions[*i])?;
    *i += 1;
    Ok(param.get_value(instructions))
}
fn get_target(instructions: &[i32], i: &mut usize) -> Result<usize, Box<dyn Error>> {
    let target = usize::try_from(instructions[*i])?;
    *i += 1;
    Ok(target)
}

fn run_program(
    instructions: &mut Vec<i32>,
    mut inputs: Vec<i32>,
) -> Result<Vec<i32>, Box<dyn Error>> {
    let mut i = 0;

    let mut output = Vec::new();

    while i < instructions.len() {
        let ([mode_1, mode_2, _mode_3], opcode) = get_opcode(&instructions, &mut i);
        match opcode {
            1 => {
                let value_1 = get_value(&instructions, &mut i, mode_1)?;
                let value_2 = get_value(&instructions, &mut i, mode_2)?;
                let target = get_target(&instructions, &mut i)?;
                instructions[target] = value_1 + value_2;
            }
            2 => {
                let value_1 = get_value(&instructions, &mut i, mode_1)?;
                let value_2 = get_value(&instructions, &mut i, mode_2)?;
                let target = get_target(&instructions, &mut i)?;
                instructions[target] = value_1 * value_2;
            }
            3 => {
                let target = get_target(&instructions, &mut i)?;
                instructions[target] = inputs.pop().ok_or("no input to consume")?;
            }
            4 => {
                let value_1 = get_value(&instructions, &mut i, mode_1)?;
                output.push(value_1);
            }
            5 => {
                let value_1 = get_value(&instructions, &mut i, mode_1)?;
                let value_2 = get_value(&instructions, &mut i, mode_2)?;

                if value_1 != 0 {
                    i = usize::try_from(value_2)?;
                }
            }
            6 => {
                let value_1 = get_value(&instructions, &mut i, mode_1)?;
                let value_2 = get_value(&instructions, &mut i, mode_2)?;

                if value_1 == 0 {
                    i = usize::try_from(value_2)?;
                }
            }
            7 => {
                let value_1 = get_value(&instructions, &mut i, mode_1)?;
                let value_2 = get_value(&instructions, &mut i, mode_2)?;
                let target = get_target(&instructions, &mut i)?;

                if value_1 < value_2 {
                    instructions[target] = 1;
                } else {
                    instructions[target] = 0;
                }
            }
            8 => {
                let value_1 = get_value(&instructions, &mut i, mode_1)?;
                let value_2 = get_value(&instructions, &mut i, mode_2)?;
                let target = get_target(&instructions, &mut i)?;

                if value_1 == value_2 {
                    instructions[target] = 1;
                } else {
                    instructions[target] = 0;
                }
            }

            99 => {
                break;
            }
            x => Err(format!("illegal opcode: {}", x))?,
        }
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_1() {
        let instructions = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        let sequence = vec![4, 3, 2, 1, 0];
        let res = run_amplifier_sequence(instructions, sequence).unwrap();
        assert_eq!(res, 43210);
    }
    #[test]
    fn test_input_2() {
        let instructions = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        let sequence = vec![0, 1, 2, 3, 4];
        let res = run_amplifier_sequence(instructions, sequence).unwrap();
        assert_eq!(res, 54321);
    }

    #[test]
    fn test_input_3() {
        let instructions = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        let sequence = vec![1, 0, 4, 3, 2];
        let res = run_amplifier_sequence(instructions, sequence).unwrap();
        assert_eq!(res, 65210);
    }
}
