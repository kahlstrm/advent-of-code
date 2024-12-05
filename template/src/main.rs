static TEST_INPUT: &str = r#""#;
static INPUT: &str = include_str!("input");
mod matrixwalkerer;

fn main() {
    let lines = if INPUT.len() == 0 { TEST_INPUT } else { INPUT }
        .lines()
        .filter_map(|l| match l.trim() {
            "" => None,
            other => Some(other),
        });
    println!("{lines:#?}")
}
