use std::{
    collections::{HashMap, HashSet},
    isize, iter,
};

static TEST_INPUT: &[u8] = br#"T....#....
...T......
.T....#...
.........#
..#.......
..........
...#......
..........
....#.....
.........."#;
static INPUT: &[u8] = include_bytes!("../inputs/day08");
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Pos(isize, isize);
fn create_anti_node_iters<'a>(
    (first, second): (&'a Pos, &'a Pos),
    (width, height): (usize, usize),
) -> (
    impl Iterator<Item = Pos> + use<'a>,
    impl Iterator<Item = Pos> + use<'a>,
) {
    let diff_pos = Pos(first.0 - second.0, first.1 - second.1);
    let increasing_diff_iter = iter::repeat(diff_pos)
        .enumerate()
        .map(|(n, diff_pos)| Pos(diff_pos.0 * (n + 1) as isize, diff_pos.1 * (n + 1) as isize));
    let first_iter = increasing_diff_iter
        .clone()
        .map(|pos| Pos(first.0 + pos.0, first.1 + pos.1))
        .take_while(move |anti_node| {
            anti_node.0 >= 0
                && anti_node.0 < width as isize
                && anti_node.1 >= 0
                && anti_node.1 < height as isize
        });
    let second_iter = increasing_diff_iter
        .map(|pos| Pos(second.0 - pos.0, second.1 - pos.1))
        .take_while(move |anti_node| {
            anti_node.0 >= 0
                && anti_node.0 < width as isize
                && anti_node.1 >= 0
                && anti_node.1 < height as isize
        });
    (first_iter, second_iter)
}
// https://adventofcode.com/2023/day/8
fn main() {
    let lines: Vec<_> = if INPUT.len() == 0 { TEST_INPUT } else { INPUT }
        .split(|c| *c == b'\n')
        .filter(|l| l.len() != 0)
        .collect();
    let width = lines[0].len();
    let height = lines.len();
    let pairings: HashMap<u8, Vec<Pos>> =
        lines
            .iter()
            .enumerate()
            .fold(HashMap::new(), |mut map, (j, arr)| {
                for (i, char) in arr.iter().enumerate() {
                    match *char {
                        b'.' => (),
                        b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' => {
                            map.entry(*char)
                                .or_default()
                                .push(Pos(i as isize, j as isize));
                        }
                        _ => (),
                    }
                }
                map
            });
    let mut anti_nodes1: HashSet<Pos> = HashSet::new();
    let mut anti_nodes2: HashSet<Pos> = HashSet::new();
    for pair_arr in pairings.values() {
        for (first_idx, second_idx) in (0..pair_arr.len() - 1)
            .flat_map(|first| (first + 1..pair_arr.len()).map(move |second| (first, second)))
        {
            let first = &pair_arr[first_idx];
            let second = &pair_arr[second_idx];
            anti_nodes2.insert(*first);
            anti_nodes2.insert(*second);
            let (mut first_iter, mut second_iter) =
                create_anti_node_iters((first, second), (width, height));
            if let Some(anti_node) = first_iter.next() {
                anti_nodes1.insert(anti_node);
                anti_nodes2.insert(anti_node);
            }
            if let Some(anti_node) = second_iter.next() {
                anti_nodes1.insert(anti_node);
                anti_nodes2.insert(anti_node);
            }
            anti_nodes2.extend(first_iter);
            anti_nodes2.extend(second_iter);
        }
    }
    println!("{}", anti_nodes1.len());
    println!("{}", anti_nodes2.len());
}
