use std::{num::NonZero, sync::Arc, thread::JoinHandle};

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
fn traverse_matrix(
    matrix: &[&[u8]],
    start_pos: (usize, usize),
    visited: &mut Vec<Vec<Option<Direction>>>,
    blocked_pos: Option<(usize, usize)>,
) -> bool {
    for arr in visited.iter_mut() {
        arr.fill(None);
    }
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
        if let Some(blocked) = blocked_pos {
            if walker
                .pos_toward(dir)
                .map(|pos| pos == blocked)
                .unwrap_or(false)
            {
                dir = dirs.next().unwrap();
                continue;
            }
        }
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
            Some(&b'#') => dir = dirs.next().unwrap(),
            Some(c) => unreachable!("somehow got here what is this {}", *c as char),
            None => break false,
        }
    };
    return is_looped;
}
fn main() {
    let matrix: Arc<[&[u8]]> = if INPUT.len() == 0 { TEST_INPUT } else { INPUT }
        .split(|c| *c == b'\n')
        .filter(|l| l.len() != 0)
        .collect();

    let start_pos: Arc<(usize, usize)> = matrix
        .iter()
        .position(|arr| arr.iter().find(|c| **c == b'^').is_some())
        .map(|j| (matrix[j].iter().position(|c| *c == b'^').unwrap(), j))
        .unwrap()
        .into();
    let mut visited: Vec<Vec<Option<Direction>>> = (0..matrix.len())
        .map(|j| vec![None; matrix[j].len()])
        .collect();
    traverse_matrix(&matrix, *start_pos, &mut visited, None);
    let visited_positions: Vec<(usize, usize)> = visited
        .iter()
        .enumerate()
        .flat_map(|(j, arr)| {
            arr.iter()
                .enumerate()
                .filter_map(move |(i, visited)| match visited {
                    Some(_) => Some((i, j)),
                    None => None,
                })
        })
        .collect();
    let visited_count = visited_positions.len();
    println!("{}", visited_count);
    let cpu_count = std::thread::available_parallelism().unwrap_or(NonZero::new(1).unwrap());
    let looped_count = visited_positions
        .chunks(visited_count / cpu_count)
        .map(|c| c.to_owned())
        .into_iter()
        .map(|arr| {
            let matrix = matrix.clone();
            let start_pos = start_pos.clone();
            std::thread::spawn(move || {
                let mut visited: Vec<Vec<Option<Direction>>> = (0..matrix.len())
                    .map(|j| vec![None; matrix[j].len()])
                    .collect();
                arr.iter()
                    .filter(|(i, j)| {
                        let cur = matrix[*j][*i];
                        if cur != b'.' {
                            return false;
                        }
                        return traverse_matrix(&matrix, *start_pos, &mut visited, (*i, *j).into());
                    })
                    .count()
            })
        })
        .collect::<Vec<JoinHandle<usize>>>()
        .into_iter()
        .fold(0, |acc, handle| acc + handle.join().unwrap());
    println!("{looped_count}");
}
