use std::fs;

pub fn solve() -> (u32, u32) {
    let mut ops = parse_input();
    // Replace op codes
    ops[1] = 12;
    ops[2] = 2;

    let first_solution = run_computer(&ops)[0];

    let mut ops = parse_input();

    for noun in 0..=99 {
        for verb in 0..=99 {
            ops[1] = noun;
            ops[2] = verb;
            let second_solution = run_computer(&ops)[0];
            if second_solution == 19690720 {
                return (first_solution, noun * 100 + verb);
            }
        }
    }

    (first_solution, first_solution)
}

#[derive(Debug)]
enum Instruction {
    Add,
    Multiply,
    Abort,
}

impl Instruction {
    fn from_u32(number: u32) -> Option<Instruction> {
        match number {
            1 => Some(Instruction::Add),
            2 => Some(Instruction::Multiply),
            99 => Some(Instruction::Abort),
            _ => None,
        }
    }
}

fn op_params(opcodes: &Vec<u32>, offset: usize) -> (u32, u32, usize) {
    let input_1 = opcodes[opcodes[offset + 1] as usize];
    let input_2 = opcodes[opcodes[offset + 2] as usize];
    let output_location = opcodes[offset + 3] as usize;

    (input_1, input_2, output_location)
}

fn run_computer(opcodes: &Vec<u32>) -> Vec<u32> {
    let mut opcodes = opcodes.clone();
    let opcode_size = 4;
    for offset in (0..opcodes.len()).step_by(opcode_size) {
        let instruction = Instruction::from_u32(opcodes[offset]).unwrap();
        match instruction {
            Instruction::Add => {
                let (input_1, input_2, output_location) = op_params(&opcodes, offset);
                opcodes[output_location] = input_1 + input_2
            }
            Instruction::Multiply => {
                let (input_1, input_2, output_location) = op_params(&opcodes, offset);
                opcodes[output_location] = input_1 * input_2
            }
            Instruction::Abort => break,
        }
    }

    opcodes
}

fn parse_input() -> Vec<u32> {
    let input_content = fs::read_to_string("src/day2/input.txt").unwrap();
    input_content
        .split(",")
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample1() {
        let input = vec![1, 0, 0, 0, 99];
        assert_eq!(run_computer(&input), vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_sample2() {
        let input = vec![2, 3, 0, 3, 99];
        assert_eq!(run_computer(&input), vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_sample3() {
        let input = vec![2, 4, 4, 5, 99, 0];
        assert_eq!(run_computer(&input), vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn test_sample4() {
        let input = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        assert_eq!(run_computer(&input), vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
