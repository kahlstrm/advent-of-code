static TEST_INPUT: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;
static INPUT: &str = include_str!("input");
fn is_correct(line: &(isize, Vec<isize>), with_concat: bool) -> bool {
    //println!("{total}: for vec {:?},i is {}, cur is {}", numbers, acc, i);
    fn _is_correct(line: &(isize, Vec<isize>), with_concat: bool, acc: isize, i: usize) -> bool {
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
        let concatted: isize = (acc.to_string() + &cur.to_string()).parse().unwrap();
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
                let result: isize = first.parse().unwrap();
                let numbers: Vec<_> = rest
                    .trim()
                    .split_whitespace()
                    .map(|n| n.parse::<isize>().unwrap())
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
