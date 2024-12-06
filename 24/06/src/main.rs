use matrixwalkerer::Direction;

static TEST_INPUT: &[u8] = br#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
static INPUT: &[u8] = include_bytes!("input");
mod matrixwalkerer;
fn traverse_matrix(matrix: &[&[u8]], start_pos: (usize, usize)) -> (usize, bool) {
    let mut visited: Vec<Vec<Option<Direction>>> = (0..matrix.len())
        .map(|j| {
            let mut v = Vec::<_>::with_capacity(matrix[j].len());
            for _ in 0..matrix[j].len() {
                v.push(None);
            }
            v
        })
        .collect();
    visited[start_pos.1][start_pos.0] = Some(Direction::NORTH);
    let mut walker = matrixwalkerer::MatrixWalker::new(&matrix, start_pos);
    let mut dirs = [
        Direction::NORTH,
        Direction::WEST,
        Direction::SOUTH,
        Direction::EAST,
    ]
    .iter()
    .cycle();
    let mut dir = dirs.next().unwrap();
    let is_looped = loop {
        match walker.peek_forward(dir) {
            Some(&b'.') | Some(&b'^') => {
                walker.get_and_step(&dir);
                let (i, j) = walker.pos();
                match visited[j][i] {
                    Some(prev_dir) => {
                        if *dir == prev_dir {
                            break true;
                        }
                    }
                    None => (),
                }
                visited[j][i] = Some(dir.to_owned());
            }
            Some(&b'#') | Some(&b'O') => dir = dirs.next().unwrap(),
            Some(c) => unreachable!("somehow got here what is this {}", *c as char),
            None => break false,
        }
    };
    let res: usize = visited
        .iter()
        .map(|arr| arr.iter().filter(|x| x.is_some()).count())
        .sum();
    return (res, is_looped);
}
fn nested_vec_as_slice<'a>(vec: &'a Vec<Vec<u8>>) -> Vec<&'a [u8]> {
    let test: Vec<&[u8]> = vec.iter().map(|v| v.as_slice()).collect();
    return test;
}

fn main() {
    let mut matrix: Vec<Vec<u8>> = if INPUT.len() == 0 { TEST_INPUT } else { INPUT }
        .split(|c| *c == b'\n')
        .filter(|l| l.len() != 0)
        .map(|a| a.to_vec())
        .collect();

    let start_pos = matrix
        .iter()
        .position(|arr| arr.iter().find(|c| **c == b'^').is_some())
        .map(|j| (matrix[j].iter().position(|c| *c == b'^').unwrap(), j))
        .unwrap();
    let (res, _) = traverse_matrix(&nested_vec_as_slice(&matrix), start_pos);
    println!("{res}");
    let mut res2 = 0;
    for j in 0..matrix.len() {
        for i in 0..matrix[j].len() {
            let cur = matrix[j][i];
            if cur != b'.' {
                continue;
            }
            matrix[j][i] = b'#';
            let (_, looped) = traverse_matrix(&nested_vec_as_slice(&matrix), start_pos);
            if looped {
                res2 += 1;
            }
            matrix[j][i] = cur;
        }
    }
    println!("{res2}");
}
