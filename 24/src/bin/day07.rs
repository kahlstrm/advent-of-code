use aoc_2024::count_digits;

static TEST_INPUT: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

static INPUT: &str = include_str!("../inputs/day07");
fn is_correct<const WITH_CONCAT: bool>(line: &(usize, Vec<usize>)) -> bool {
    //println!("{total}: for vec {:?},i is {}, cur is {}", numbers, acc, i);
    fn _is_correct<const WITH_CONCAT: bool>(
        line: &(usize, Vec<usize>),
        acc: usize,
        i: usize,
    ) -> bool {
        let (total, numbers) = line;
        if i == numbers.len() {
            return acc == *total;
        }
        let cur = numbers[i];
        let first_res = _is_correct::<WITH_CONCAT>(line, acc + cur, i + 1)
            || _is_correct::<WITH_CONCAT>(line, acc * cur, i + 1);
        if !WITH_CONCAT {
            return first_res;
        }
        let concatted = acc * 10_usize.pow(count_digits(cur)) + cur;
        return first_res || _is_correct::<WITH_CONCAT>(line, concatted, i + 1);
    }
    return _is_correct::<WITH_CONCAT>(line, 0, 0);
}
// https://adventofcode.com/2024/day/7
fn main() {
    let lines = if INPUT.len() == 0 { TEST_INPUT } else { INPUT }
        .lines()
        .filter_map(|l| match l.trim() {
            "" => None,
            other => {
                let (first, rest) = other.split_once(":").unwrap();
                let result: usize = first.parse().unwrap();
                let numbers: Vec<_> = rest
                    .trim()
                    .split_whitespace()
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect();
                Some((result, numbers))
            }
        });
    let mut sum = 0;
    let mut sum2 = 0;
    for line in lines {
        if is_correct::<false>(&line) {
            sum += line.0;
            sum2 += line.0;
        } else if is_correct::<true>(&line) {
            sum2 += line.0;
        }
    }
    println!("{sum:#?}");
    println!("{sum2:#?}");
}
