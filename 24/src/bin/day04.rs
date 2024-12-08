use aoc_2024::matrixwalkerer::{Direction, MatrixWalker, ALL_DIRECTIONS};

static TEST_INPUT: &[u8] = br#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
static INPUT: &[u8] = include_bytes!("../inputs/day04");

fn find_word(matrix: &[&[u8]], word: &[u8], (i, j): (usize, usize)) -> usize {
    let mut sum = 0;
    sum += ALL_DIRECTIONS
        .iter()
        .filter(|dir| {
            let mut walker = MatrixWalker::new(matrix, (i, j));
            let res = word
                .iter()
                .take_while(|c| walker.get_and_step(dir).is_some_and(|v| v == *c))
                .count()
                == word.len();
            res
        })
        .count();
    return sum;
}

fn find_x(matrix: &[&[u8]], cross_word: &[u8], (i, j): (usize, usize)) -> bool {
    if cross_word.len() % 2 != 1 {
        panic!("cross word should be uneven");
    }
    let middle_i = cross_word.len() / 2;
    let middle = cross_word[middle_i];
    if matrix[j][i] != middle {
        return false;
    }
    //println!("found middle {} at {},{}", middle as char, i, j);
    let mut walker1 = MatrixWalker::new(matrix, (i, j));
    let mut walker2 = MatrixWalker::new(matrix, (i, j));
    //println!("traversing {} steps", middle_i);
    if let Err(_n) = walker1.traverse(&Direction::NORTHWEST, middle_i) {
        //println!(
        //    "traversal from {},{} towards {:#?} failed after {} steps",
        //    i,
        //    j,
        //    Direction::NORTHWEST,
        //    _n
        //);
        return false;
    }
    if let Err(_n) = walker2.traverse(&Direction::SOUTHWEST, middle_i) {
        //println!(
        //    "traversal from {},{} towards {:#?} failed after {} steps",
        //    i,
        //    j,
        //    Direction::SOUTHWEST,
        //    _n
        //);
        return false;
    }
    let mut word_matching1 = true;
    let mut rev_matching1 = true;
    let mut word_matching2 = true;
    let mut rev_matching2 = true;
    for (c, c_rev) in cross_word.iter().zip(cross_word.iter().rev()) {
        let Some(v1) = walker1.get_and_step(&Direction::SOUTHEAST) else {
            //println!("walker1 stopped");
            return false;
        };
        let Some(v2) = walker2.get_and_step(&Direction::NORTHEAST) else {
            //println!("walker2 stopped");
            return false;
        };
        //println!(
        //    "matching {} and {} with values {} and {}",
        //    *c as char, *c_rev as char, *v1 as char, *v2 as char
        //);
        word_matching1 = word_matching1 && c == v1;
        rev_matching1 = rev_matching1 && c_rev == v1;
        word_matching2 = word_matching2 && c == v2;
        rev_matching2 = rev_matching2 && c_rev == v2;
        if !word_matching1 && !rev_matching1 || !word_matching2 && !rev_matching2 {
            //println!(
            //    "stopping matching {},{},{},{}",
            //    word_matching1, rev_matching1, word_matching2, rev_matching2
            //);
            return false;
        }
    }
    //println!(
    //    "found cross at {},{} with {},{},{}‚{}",
    //    i, j, word_matching1, rev_matching1, word_matching2, rev_matching2
    //);
    assert_eq!(word_matching1 || rev_matching1, true);
    assert_eq!(word_matching2 || rev_matching2, true);
    true
}
fn main() {
    let matrix: Vec<_> = if INPUT.len() == 0 { TEST_INPUT } else { INPUT }
        .split(|c| *c == b'\n')
        .filter(|l| l.len() != 0)
        .collect();
    //print!(" ");
    //for i in 0..matrix.len() {
    //    print!("{i}");
    //}
    //println!();
    //for (j, line) in matrix.iter().enumerate() {
    //    print!("{j}");
    //    io::stdout().write(line).unwrap();
    //    io::stdout().write(b"\n").unwrap();
    //}
    let mut res1 = 0;
    let mut res2 = 0;
    for j in 0..matrix.len() {
        for i in 0..matrix[j].len() {
            res1 += find_word(&matrix, b"XMAS", (i, j));
            res2 += find_x(&matrix, b"MAS", (i, j)) as usize;
        }
    }
    println!("{res1}");
    println!("{res2}");
}
