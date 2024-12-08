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
fn count_digits(num: usize) -> u32 {
    num.ilog10() + 1
}
#[cfg(test)]
mod tests {
    use crate::count_digits;

    #[test]
    fn count_digits_works() {
        for num in 1..10000 {
            assert_eq!(count_digits(num), num.to_string().len() as u32);
        }
    }
}
fn is_correct(line: &(usize, Vec<usize>), with_concat: bool) -> bool {
    //println!("{total}: for vec {:?},i is {}, cur is {}", numbers, acc, i);
    fn _is_correct(line: &(usize, Vec<usize>), with_concat: bool, acc: usize, i: usize) -> bool {
        let (total, numbers) = line;
        if i == numbers.len() {
            return acc == *total;
        }
        let cur = numbers[i];
        let first_res = _is_correct(line, with_concat, acc + cur, i + 1)
            || _is_correct(line, with_concat, acc * cur, i + 1);
        if !with_concat {
            return first_res;
        }
        let concatted = acc * 10_usize.pow(count_digits(cur)) + cur;
        return first_res || _is_correct(line, with_concat, concatted, i + 1);
    }
    return _is_correct(line, with_concat, 0, 0);
}
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
        if is_correct(&line, false) {
            sum += line.0;
            sum2 += line.0;
        } else if is_correct(&line, true) {
            sum2 += line.0;
        }
    }
    println!("{sum:#?}");
    println!("{sum2:#?}");
}
