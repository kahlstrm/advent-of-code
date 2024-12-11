const USE_TEST_INPUT: bool = false;
static TEST_INPUT: &str = r#"125 17"#;
static INPUT: &str = include_str!("../inputs/day11");

fn count_digits(num: usize) -> u32 {
    num.ilog10() + 1
}
fn blink(line: &[usize]) -> Vec<usize> {
    let mut res = vec![];
    for stone in line {
        if *stone == 0 {
            res.push(1);
        } else if count_digits(*stone) % 2 == 0 {
            let as_str = stone.to_string();
            let (left, right) = as_str.split_at(as_str.len() / 2);
            res.push(left.parse().unwrap());
            res.push(right.parse().unwrap());
        } else {
            res.push(*stone * 2024);
        }
    }
    res
}
// https://adventofcode.com/2024/day/11
fn main() {
    let debug = USE_TEST_INPUT || INPUT.len() == 0;
    let line: Vec<_> = if debug { TEST_INPUT } else { INPUT }
        .trim()
        .split(' ')
        .map(|x| (x.parse::<usize>().unwrap()))
        .collect();
    println!("{line:#?}");
    let mut res = line;
    for i in 0..75 {
        println!("{i}");
        res = blink(&res);
    }
    println!("{}", res.len())
}
