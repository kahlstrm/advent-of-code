use std::{
    collections::{HashMap, HashSet},
    fmt::write,
    hash::Hash,
    usize,
};

static TEST_INPUT: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#;
static INPUT: &str = include_str!("input");
mod matrixwalkerer;
fn is_correct(update: &[usize], rules: &HashMap<usize, HashSet<usize>>) -> bool {
    let mut stuff = HashSet::new();
    for num in update {
        if rules
            .get(num)
            .map(|h| h.intersection(&stuff).count())
            .unwrap_or_default()
            != 0
        {
            return false;
        }
        stuff.insert(*num);
    }
    return true;
}
fn fix_order(update: &[usize], inverted_rules: &HashMap<usize, HashSet<usize>>) -> Vec<usize> {
    let mut res: Vec<usize> = Vec::new();
    let u_hset = HashSet::from_iter(update.iter().copied());
    while res.len() != update.len() {
        let hset = HashSet::from_iter(res.iter().copied());
        for trial in update.iter().filter(|n| !res.contains(n)) {
            if inverted_rules
                .get(trial)
                .map(|s| {
                    s.intersection(&u_hset)
                        .copied()
                        .collect::<HashSet<_>>()
                        .difference(&hset)
                        .count()
                })
                .unwrap_or_default()
                == 0
            {
                res.push(*trial);
                break;
            };
        }
        println!("res len{}", res.len());
    }
    res
}

fn main() {
    let (rules, updates) = if INPUT.len() == 0 { TEST_INPUT } else { INPUT }
        .split_once("\n\n")
        .map(|(r, u)| {
            (
                r.lines().filter(|l| !l.is_empty()),
                u.lines().filter(|l| !l.is_empty()),
            )
        })
        .unwrap();
    let mut rules_map = HashMap::new();
    let mut rules_map2 = HashMap::new();
    for rule in rules {
        let (must_come_before, must_come_after) = rule
            .split_once('|')
            .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
            .unwrap();
        rules_map
            .entry(must_come_before)
            .or_insert_with(|| HashSet::new())
            .insert(must_come_after);
        rules_map2
            .entry(must_come_after)
            .or_insert_with(|| HashSet::new())
            .insert(must_come_before);
    }
    let updates: Vec<_> = updates
        .map(|update| {
            update
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    let mut sum = 0;
    let mut sum2 = 0;
    for update in updates {
        if is_correct(&update, &rules_map) {
            sum += update[update.len() / 2];
            println!("{:?} is correct", update);
        } else {
            println!("{:?} is incorrect", update);
            let fixed = fix_order(&update, &rules_map2);

            sum2 += fixed[fixed.len() / 2];
        }
    }
    println!("{}", sum);
    println!("{}", sum2);
}
