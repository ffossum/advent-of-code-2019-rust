use digits_iterator::*;
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

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("day05/input.txt")?;
    let input = input
        .split(",")
        .map(|x| x.trim().parse().unwrap())
        .collect::<Vec<i32>>();

    run_program(input.clone(), 1)?;
    println!("");
    run_program(input, 5)?;

    Ok(())
}

fn get_opcode(input: &[i32], i: &mut usize) -> ([u8; 3], i32) {
    let opcode: i32 = input[*i];
    *i += 1;

    let reversed_digits = opcode.digits().rev().collect::<Vec<u8>>();

    let mode_1: u8 = reversed_digits.get(2).copied().unwrap_or(0);
    let mode_2: u8 = reversed_digits.get(3).copied().unwrap_or(0);
    let mode_3: u8 = reversed_digits.get(4).copied().unwrap_or(0);

    let opcode = opcode % 100;

    ([mode_1, mode_2, mode_3], opcode)
}

fn get_value(input: &[i32], i: &mut usize, mode: u8) -> Result<i32, Box<dyn Error>> {
    let param = Parameter::new(mode, input[*i])?;
    *i += 1;
    Ok(param.get_value(input))
}
fn get_target(input: &[i32], i: &mut usize) -> Result<usize, Box<dyn Error>> {
    let target = usize::try_from(input[*i])?;
    *i += 1;
    Ok(target)
}

fn run_program(mut input: Vec<i32>, system_id: i32) -> Result<Vec<i32>, Box<dyn Error>> {
    let mut i = 0;
    while i < input.len() {
        let ([mode_1, mode_2, _mode_3], opcode) = get_opcode(&input, &mut i);
        match opcode {
            1 => {
                let value_1 = get_value(&input, &mut i, mode_1)?;
                let value_2 = get_value(&input, &mut i, mode_2)?;
                let target = get_target(&input, &mut i)?;
                input[target] = value_1 + value_2;
            }
            2 => {
                let value_1 = get_value(&input, &mut i, mode_1)?;
                let value_2 = get_value(&input, &mut i, mode_2)?;
                let target = get_target(&input, &mut i)?;
                input[target] = value_1 * value_2;
            }
            3 => {
                let target = get_target(&input, &mut i)?;
                input[target] = system_id;
            }
            4 => {
                let value_1 = get_value(&input, &mut i, mode_1)?;
                println!("{}", value_1);
            }
            5 => {
                let value_1 = get_value(&input, &mut i, mode_1)?;
                let value_2 = get_value(&input, &mut i, mode_2)?;

                if value_1 != 0 {
                    i = usize::try_from(value_2)?;
                }
            }
            6 => {
                let value_1 = get_value(&input, &mut i, mode_1)?;
                let value_2 = get_value(&input, &mut i, mode_2)?;

                if value_1 == 0 {
                    i = usize::try_from(value_2)?;
                }
            }
            7 => {
                let value_1 = get_value(&input, &mut i, mode_1)?;
                let value_2 = get_value(&input, &mut i, mode_2)?;
                let target = get_target(&input, &mut i)?;

                if value_1 < value_2 {
                    input[target] = 1;
                } else {
                    input[target] = 0;
                }
            }
            8 => {
                let value_1 = get_value(&input, &mut i, mode_1)?;
                let value_2 = get_value(&input, &mut i, mode_2)?;
                let target = get_target(&input, &mut i)?;

                if value_1 == value_2 {
                    input[target] = 1;
                } else {
                    input[target] = 0;
                }
            }

            99 => {
                println!("HALT");
                break;
            }
            x => panic!(format!("illegal opcode: {}", x)),
        }
    }
    Ok(input)
}
