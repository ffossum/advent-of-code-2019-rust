fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = input
        .split(",")
        .map(|x| x.trim().parse().unwrap())
        .collect::<Vec<usize>>();

    let part1_ans = {
        let mut input = input.clone();
        input[1] = 12;
        input[2] = 2;
        run_program(input)[0]
    };
    println!("{}", part1_ans);

    let part2_ans = (0..100)
        .flat_map(|noun| (0..100).map(move |verb| (noun, verb)))
        .find(|&(noun, verb)| {
            let mut input = input.clone();
            input[1] = noun;
            input[2] = verb;
            run_program(input)[0] == 19690720
        })
        .map(|(verb, noun)| 100 * verb + noun)
        .unwrap();

    println!("{}", part2_ans);
}

fn run_program(mut input: Vec<usize>) -> Vec<usize> {
    let mut i = 0;

    while i < input.len() {
        let opcode = input[i];
        let op: fn(usize, usize) -> usize = match opcode {
            1 => |a, b| a + b,
            2 => |a, b| a * b,
            99 => break,
            _ => panic!("illegal opcode"),
        };

        let a_idx = input[i + 1];
        let a = input[a_idx];
        let b_idx = input[i + 2];
        let b = input[b_idx];

        let target_idx = input[i + 3];
        input[target_idx] = op(a, b);

        i += 4;
    }

    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_run_program() {
        assert_eq!(run_program(vec![1, 0, 0, 0, 99]), vec![2, 0, 0, 0, 99]);
        assert_eq!(run_program(vec![2, 3, 0, 3, 99]), vec![2, 3, 0, 6, 99]);
        assert_eq!(
            run_program(vec![2, 4, 4, 5, 99, 0]),
            vec![2, 4, 4, 5, 99, 9801]
        );
        assert_eq!(
            run_program(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]),
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
    }
}
