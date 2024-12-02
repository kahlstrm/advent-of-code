static TEST_INPUT: &str = r#"
3   4
4   3
2   5
1   3
3   9
3   3
"#;
static INPUT: &str = include_str!("input");

fn main() {
    let (mut left, mut right): (Vec<_>, Vec<_>) = if INPUT.len() == 0 { TEST_INPUT } else { INPUT }
        .lines()
        .filter_map(|l| match l.trim() {
            "" => None,
            other => {
                let mut l = other.split_whitespace();
                let first = l.next().unwrap().parse::<i32>().unwrap();
                let second = l.next().unwrap().parse::<i32>().unwrap();

                Some((first, second))
            }
        })
        .unzip();
    left.sort();
    right.sort();
    let res1: i32 = left
        .iter()
        .zip(right.clone())
        .map(|(a, b)| (a - b).abs())
        .sum();
    let res2: i32 = left
        .iter()
        .map(|v| v * (right.iter().filter(|b| **b == *v).count() as i32))
        .sum();
    println!("{res1:#?}");
    println!("{res2:#?}");
}
