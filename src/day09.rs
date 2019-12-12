use digits_iterator::*;
use itertools::Itertools;
use std::convert::TryFrom;
use std::error::Error;

enum Parameter {
    Position(usize),
    Immediate(i32),
    Relative(usize),
}
impl Parameter {
    fn new(mode: u8, value: i32) -> Result<Parameter, Box<dyn Error>> {
        match mode {
            0 => Ok(Parameter::Position(usize::try_from(value)?)),
            1 => Ok(Parameter::Immediate(value)),
            2 => Ok(Parameter::Relative(usize::try_from(value)?)),
            _ => Err(format!("illegal mode: {}", mode))?,
        }
    }
}

pub fn main() -> Result<(), Box<dyn Error>> {
    Ok(())
}

struct Program {
    instructions: Vec<i32>,
    idx: usize,
    relative_base: usize,
}

impl Program {
    pub fn new(instructions: Vec<i32>) -> Self {
        Program {
            instructions,
            idx: 0,
            relative_base: 0,
        }
    }

    fn get_value(&mut self, mode: u8) -> Result<i32, Box<dyn Error>> {
        let param = Parameter::new(mode, self.instructions[self.idx])?;
        self.idx += 1;

        let value = match param {
            Parameter::Position(i) => self.instructions[i],
            Parameter::Immediate(value) => value,
            Parameter::Relative(i) => self.instructions[self.relative_base + i],
        };

        Ok(value)
    }

    fn get_opcode(&mut self) -> ([u8; 3], i32) {
        let opcode: i32 = self.instructions[self.idx];
        self.idx += 1;

        let mut reversed_digits = opcode.digits().rev().skip(2);

        let mode_1: u8 = reversed_digits.next().unwrap_or(0);
        let mode_2: u8 = reversed_digits.next().unwrap_or(0);
        let mode_3: u8 = reversed_digits.next().unwrap_or(0);

        let opcode = opcode % 100;

        ([mode_1, mode_2, mode_3], opcode)
    }

    fn get_address(&mut self) -> Result<usize, Box<dyn Error>> {
        let target = usize::try_from(self.instructions[self.idx])?;
        self.idx += 1;
        Ok(target)
    }

    pub fn run(&mut self, mut input: Option<i32>) -> Result<Output, Box<dyn Error>> {
        while self.idx < self.instructions.len() {
            let instruction_start_i = self.idx;

            let ([mode_1, mode_2, _mode_3], opcode) = self.get_opcode();
            match opcode {
                1 => {
                    let value_1 = self.get_value(mode_1)?;
                    let value_2 = self.get_value(mode_2)?;
                    let target = self.get_address()?;
                    self.instructions[target] = value_1 + value_2;
                }
                2 => {
                    let value_1 = self.get_value(mode_1)?;
                    let value_2 = self.get_value(mode_2)?;
                    let target = self.get_address()?;
                    self.instructions[target] = value_1 * value_2;
                }
                3 => {
                    if let Some(input_value) = input.take() {
                        let target = self.get_address()?;
                        self.instructions[target] = input_value;
                    } else {
                        self.idx = instruction_start_i;
                        return Ok(Output::WaitingForInput);
                    }
                }
                4 => {
                    let value_1 = self.get_value(mode_1)?;
                    return Ok(Output::Value(value_1));
                }
                5 => {
                    let value_1 = self.get_value(mode_1)?;
                    let value_2 = self.get_value(mode_2)?;

                    if value_1 != 0 {
                        self.idx = usize::try_from(value_2)?;
                    }
                }
                6 => {
                    let value_1 = self.get_value(mode_1)?;
                    let value_2 = self.get_value(mode_2)?;

                    if value_1 == 0 {
                        self.idx = usize::try_from(value_2)?;
                    }
                }
                7 => {
                    let value_1 = self.get_value(mode_1)?;
                    let value_2 = self.get_value(mode_2)?;
                    let target = self.get_address()?;

                    if value_1 < value_2 {
                        self.instructions[target] = 1;
                    } else {
                        self.instructions[target] = 0;
                    }
                }
                8 => {
                    let value_1 = self.get_value(mode_1)?;
                    let value_2 = self.get_value(mode_2)?;
                    let target = self.get_address()?;

                    if value_1 == value_2 {
                        self.instructions[target] = 1;
                    } else {
                        self.instructions[target] = 0;
                    }
                }
                9 => {
                    let value_1 = self.get_value(mode_1)?;
                    self.relative_base += usize::try_from(value_1)?;
                }

                99 => {
                    return Ok(Output::Halted);
                }
                x => Err(format!("illegal opcode: {}", x))?,
            }
        }

        Err("unexpected end")?
    }
}

#[derive(Debug)]
enum Output {
    Value(i32),
    WaitingForInput,
    Halted,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_looped_amplifier_sequence(
        instructions: &Vec<i32>,
        phase_settings: &[i32],
    ) -> Result<i32, Box<dyn Error>> {
        let mut programs = Vec::new();
        for &phase_setting in phase_settings {
            let mut program = Program::new(instructions.clone());
            program.run(Some(phase_setting))?;
            programs.push(program)
        }

        let mut previous_output = 0;
        for amp_id in (0..phase_settings.len()).cycle() {
            let program = programs.get_mut(amp_id).unwrap();
            let output = program.run(Some(previous_output))?;

            match output {
                Output::Value(value) => previous_output = value,
                Output::Halted => return Ok(previous_output),
                _ => (),
            }
        }

        Err("no result")?
    }

    fn run_amplifier_sequence(
        instructions: &Vec<i32>,
        sequence: &[i32],
    ) -> Result<i32, Box<dyn Error>> {
        let mut previous_output = 0;
        for &x in sequence {
            let instructions = instructions.clone();
            let mut program = Program::new(instructions);

            program.run(Some(x))?;
            let output = program.run(Some(previous_output))?;

            if let Output::Value(value) = output {
                previous_output = value;
            }
        }
        Ok(previous_output)
    }

    #[test]
    fn test_input_1() {
        let instructions = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        let sequence = vec![4, 3, 2, 1, 0];

        let res = run_amplifier_sequence(&instructions, &sequence).unwrap();
        assert_eq!(res, 43210);
    }
    #[test]
    fn test_input_2() {
        let instructions = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        let sequence = vec![0, 1, 2, 3, 4];
        let res = run_amplifier_sequence(&instructions, &sequence).unwrap();
        assert_eq!(res, 54321);
    }

    #[test]
    fn test_input_3() {
        let instructions = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        let sequence = vec![1, 0, 4, 3, 2];
        let res = run_amplifier_sequence(&instructions, &sequence).unwrap();
        assert_eq!(res, 65210);
    }

    #[test]
    fn test_loop_input_1() {
        let instructions = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        let sequence = vec![9, 8, 7, 6, 5];
        let res = run_looped_amplifier_sequence(&instructions, &sequence).unwrap();
        assert_eq!(res, 139629729);
    }
}
