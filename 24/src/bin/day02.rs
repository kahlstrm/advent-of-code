use std::cmp::Ordering;

static TEST_INPUT: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;
static INPUT: &str = include_str!("../inputs/day02");

fn is_safe(line: &[i32]) -> bool {
    let mut ord: Option<Ordering> = None;
    for nums in line.windows(2) {
        let [a, b] = nums else {
            unreachable!();
        };
        if !(1..=3).contains(&a.abs_diff(*b)) {
            return false;
        }
        // first pair
        let Some(order) = ord else {
            ord = Some(a.cmp(b));
            continue;
        };
        if a.cmp(b) != order {
            return false;
        }
    }
    return true;
}
fn main() {
    let lines: Vec<_> = if INPUT.len() == 0 { TEST_INPUT } else { INPUT }
        .lines()
        .filter_map(|l| match l.trim() {
            "" => None,
            other => Some(
                other
                    .split_whitespace()
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect::<Vec<_>>(),
            ),
        })
        .collect();
    let res1 = lines.iter().filter(|line| is_safe(line)).count();
    let res2 = lines
        .into_iter()
        .filter(|line| {
            if is_safe(line) {
                return true;
            };
            // Not efficient but much prettier than alternatives
            for i in 0..line.len() {
                let mut line2 = line.clone();
                line2.remove(i);
                if is_safe(&line2) {
                    return true;
                }
            }

            false
        })
        .count();
    println!("{res1:#?}");
    println!("{res2:#?}");
}
