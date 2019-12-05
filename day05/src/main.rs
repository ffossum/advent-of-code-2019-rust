use std::error::Error;
use std::convert::TryFrom;

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
    let input = std::fs::read_to_string("input.txt")?;
    let input = input
        .split(",")
        .map(|x| x.trim().parse().unwrap())
        .collect::<Vec<i32>>();

    run_program(input)?;

    Ok(())
}

fn run_program(mut input: Vec<i32>) -> Result<Vec<i32>, Box<dyn Error>> {
    let mut i = 0;

    while i < input.len() {
        let opcode: i32 = input[i];

        let mode_1 = u8::try_from((opcode % 1_000 / 100) % 100)?;
        let mode_2 = u8::try_from((opcode % 10_000 / 1_000) % 10)?;
        let _mode_3 = u8::try_from(opcode / 10_000)?;
        let opcode = opcode % 100;

        match opcode % 100 {
            1 => {
                let param_1 = Parameter::new(mode_1, input[i + 1])?;
                let param_2 = Parameter::new(mode_2, input[i + 2])?;
                let target = usize::try_from(input[i + 3])?;

                let value_1 = param_1.get_value(&input);
                let value_2 = param_2.get_value(&input);

                input[target] = value_1 + value_2;

                i += 4;
            },
            2 => {
                let param_1 = Parameter::new(mode_1, input[i + 1])?;
                let param_2 = Parameter::new(mode_2, input[i + 2])?;
                let target = usize::try_from(input[i + 3])?;

                let value_1 = param_1.get_value(&input);
                let value_2 = param_2.get_value(&input);

                input[target] = value_1 * value_2;

                i += 4;
            },
            3 => {

                let target = usize::try_from(input[i + 1])?;
                let value_1 = 1;
                input[target] = value_1;

                i += 2;
            },
            4 => {
                let param_1 = Parameter::new(mode_1, input[i + 1])?;
                let value_1 = param_1.get_value(&input);
                println!("{}", value_1);
                i += 2;
            }
            99 => {
                println!("HALT");
                break
            }
            x => panic!(format!("illegal opcode: {}", x)),
        };
    }

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_run_program() {
        assert_eq!(run_program(vec![1002,4,3,4,33_i32]).unwrap(), vec![1002,4,3,4,99]);
    }
}
