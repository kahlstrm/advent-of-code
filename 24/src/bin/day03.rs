static TEST_INPUT: &str =
    r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
static INPUT: &str = include_str!("../inputs/day03");
const MUL_START: &str = "mul(";
const DO_INSTR: &str = "do()";
const DONT_INSTR: &str = "don't()";
enum Instruction {
    MUL,
    DO,
    DONT,
}
fn find_instr(line: &str) -> Option<(usize, Instruction)> {
    for i in 0..line.len() {
        if line[i..].starts_with(MUL_START) {
            return Some((i + MUL_START.len(), Instruction::MUL));
        }
        if line[i..].starts_with(DO_INSTR) {
            return Some((i + DO_INSTR.len(), Instruction::DO));
        }
        if line[i..].starts_with(DONT_INSTR) {
            return Some((i + DONT_INSTR.len(), Instruction::DONT));
        }
    }
    None
}
fn parse_mul_instruction(line: &str) -> Option<(i32, usize)> {
    let Some(end_idx) = line.find(')') else {
        return None;
    };
    //println!("{:#?}", &line[..end_idx]);
    let Some((first, second)) = line[..end_idx].split_once(',').and_then(|(a, b)| {
        let Ok(a) = a.parse::<i32>() else {
            return None;
        };
        let Ok(b) = b.parse::<i32>() else {
            return None;
        };
        Some((a, b))
    }) else {
        return None;
    };
    //println!(
    //    "found! {} * {} = {}, end idx {}",
    //    first,
    //    second,
    //    first * second,
    //    end_idx
    //);
    return Some((first * second, end_idx));
}
fn main() {
    let mut line = if INPUT.len() == 0 { TEST_INPUT } else { INPUT };
    let mut acc = 0;
    let mut acc2 = 0;
    let mut enabled = true;
    while !line.is_empty() {
        let Some((start_idx, instr)) = find_instr(line) else {
            break;
        };
        line = &line[start_idx..];
        match instr {
            Instruction::MUL => {
                let Some((mul_res, end)) = parse_mul_instruction(line) else {
                    continue;
                };
                acc += mul_res;
                if enabled {
                    acc2 += mul_res;
                }
                line = &line[end + 1..];
            }
            Instruction::DO => enabled = true,
            Instruction::DONT => enabled = false,
        }
    }
    println!("{acc:#?}");
    println!("{acc2:#?}");
}
