const USE_TEST_INPUT: bool = false;
static TEST_INPUT: &str = r#"125 17"#;
static INPUT: &str = include_str!("../inputs/day11");

fn count_digits(num: usize) -> u32 {
    num.ilog10() + 1
}
fn split_number(num: usize) -> (usize, usize) {
    let digits = count_digits(num);
    assert!(digits % 2 == 0);
    let mul = 10_usize.pow(digits / 2);
    let upper = num / 10_usize.pow(digits / 2);
    let lower = num % (upper * mul);
    (upper, lower)
}
#[cfg(test)]
mod tests {
    use crate::split_number;

    #[test]
    fn split_number_works() {
        let stone = 1234;
        assert_eq!(split_number(stone), (12, 34))
    }
}
fn blink(line: &[usize], count: usize) -> usize {
    let mut sum = 0;
    for stone in line {
        println!("doing stone {}", stone);

        let mut res = vec![*stone];
        for i in 0..count {
            let mut res2 = vec![];
            println!("{i}");
            for stone in &res {
                if *stone == 0 {
                    res2.push(1);
                } else if count_digits(*stone) % 2 == 0 {
                    let (left, right) = split_number(*stone);
                    res2.push(left);
                    res2.push(right);
                } else {
                    res2.push(*stone * 2024);
                }
            }
            res = res2;
        }
        sum += res.len();
    }
    sum
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
    let res = blink(&line, 75);
    println!("{}", res)
}
