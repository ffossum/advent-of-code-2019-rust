use std::iter;

pub fn main() {
    let input = get_input();
    let output = fft_100(&input);

    let day1_ans = output.into_iter().take(8).collect::<Vec<_>>();
    println!("{:?}", day1_ans)
}

fn fft_100(input: &[i32]) -> Vec<i32> {
    let mut result = fft(input);
    for _ in 0..99 {
        result = fft(&result);
    }
    result
}

fn fft(input: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    for n in 1..=input.len() {
        let sum: i32 = input.iter().zip(pattern(n)).map(|(a, b)| a * b).sum();
        let last_digit = sum.abs() % 10;
        result.push(last_digit);
    }
    result
}

fn pattern(n: usize) -> impl Iterator<Item = i32> {
    iter::repeat(0)
        .take(n)
        .chain(iter::repeat(1).take(n))
        .chain(iter::repeat(0).take(n))
        .chain(iter::repeat(-1).take(n))
        .cycle()
        .skip(1)
}

fn get_digits(s: &str) -> Vec<i32> {
    s.chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect::<Vec<i32>>()
}

fn get_input() -> Vec<i32> {
    get_digits(
        "59719811742386712072322509550573967421647565332667367184388997335292349852954113343804787102604664096288440135472284308373326245877593956199225516071210882728614292871131765110416999817460140955856338830118060988497097324334962543389288979535054141495171461720836525090700092901849537843081841755954360811618153200442803197286399570023355821961989595705705045742262477597293974158696594795118783767300148414702347570064139665680516053143032825288231685962359393267461932384683218413483205671636464298057303588424278653449749781937014234119757220011471950196190313903906218080178644004164122665292870495547666700781057929319060171363468213087408071790"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fft() {
        let input = get_digits("80871224585914546619083218645595");
        let output = fft_100(&input);
        assert_eq!(
            output.iter().copied().take(8).collect::<Vec<_>>(),
            get_digits("24176176")
        )
    }
}
