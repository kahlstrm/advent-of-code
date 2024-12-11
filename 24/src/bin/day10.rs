use std::collections::HashSet;

use aoc_2024::matrixwalkerer::{Direction, MatrixWalker};

const USE_TEST_INPUT: bool = false;
static TEST_INPUT: &[u8] = br#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;
static INPUT: &[u8] = include_bytes!("../inputs/day10");
fn debug_print(matrix: &[&[u8]]) {
    for line in matrix {
        for c in *line {
            print!("{}", *c as char)
        }
        println!();
    }
}
// https://adventofcode.com/2024/day/10
fn main() {
    let debug = USE_TEST_INPUT || INPUT.len() == 0;
    let matrix: Vec<_> = if debug { TEST_INPUT } else { INPUT }
        .trim_ascii()
        .split(|c| *c == b'\n')
        .filter_map(|l| match l.trim_ascii() {
            &[] => None,
            other => other.into(),
        })
        .collect();
    if debug {
        debug_print(&matrix);
    }
    let positions = matrix.iter().enumerate().flat_map(|(j, line)| {
        line.iter()
            .enumerate()
            .filter_map(move |(i, c)| if *c == b'0' { Some((i, j)) } else { None })
    });
    let (res1, res2) = positions.fold((0, 0), |acc, pos| {
        let (unique_tops, unique_trails) = find_trailheads(&matrix, pos);
        (acc.0 + unique_tops, acc.1 + unique_trails)
    });
    println!("{res1}");
    println!("{res2}");
}
const DIRECTIONS: &[Direction] = &[
    Direction::NORTH,
    Direction::WEST,
    Direction::SOUTH,
    Direction::EAST,
];
fn find_trailheads(matrix: &[&[u8]], (i, j): (usize, usize)) -> (usize, usize) {
    assert_eq!(matrix[j][i], b'0');
    fn _find_heads(
        matrix: &[&[u8]],
        coordinates: (usize, usize),
        prev: Option<Direction>,
        // runtime is already ~3ms so no significant difference between Vec<Vec<bool>> and HashSet
        mut visited: HashSet<(usize, usize)>,
    ) -> (usize, usize, HashSet<(usize, usize)>) {
        let cur = matrix[coordinates.1][coordinates.0];
        if cur == b'9' {
            let is_unique = visited.contains(&coordinates);
            visited.insert(coordinates);
            return (is_unique as usize, 1, visited);
        }
        visited.insert(coordinates);
        let walker = MatrixWalker::new(matrix, coordinates);
        DIRECTIONS
            .iter()
            .filter(|dir| !prev.is_some_and(|prev_dir| prev_dir == **dir))
            .fold((0, 0, visited), |acc, dir| {
                if walker.peek_forward(dir).is_some_and(|v| cur + 1 == *v) {
                    let (found_uniq, found, visited) =
                        _find_heads(matrix, walker.pos_toward(dir).unwrap(), prev, acc.2);
                    (acc.0 + found_uniq, acc.1 + found, visited)
                } else {
                    acc
                }
            })
    }
    let (unique_trailheads, all_paths, _) = _find_heads(matrix, (i, j), None, HashSet::new());
    (unique_trailheads, all_paths)
}
