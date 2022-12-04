use std::collections::{HashMap, HashSet};

pub fn solve_part_one(input: String) -> i32 {
    let mut dict: HashMap<char, usize> = HashMap::new();
    for (idx, c) in ('a'..='z').enumerate() {
        dict.insert(c, idx + 1);
    }
    for (idx, c) in ('A'..='Z').enumerate() {
        dict.insert(c, idx + 27);
    }

    let mut result = 0;
    for line in input.lines() {
        let (first, last) = line.split_at(line.len() / 2);
        let mut seen: HashSet<char> = HashSet::new();
        for c in first.chars() {
            if !seen.contains(&c) && last.find(c).is_some() {
                result += *dict.get(&c).unwrap() as i32;
            }
            seen.insert(c);
        }
    }

    println!("result: {}", result);
    result
}



#[test]
fn test_solve_part_one() {
    let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw".to_string();

    let expected = 157;
    let actual = solve_part_one(input);
    assert_eq!(expected, actual);
}

pub fn solve_part_two(input: String) -> i32 {
    let mut dict: HashMap<char, usize> = HashMap::new();
    for (idx, c) in ('a'..='z').enumerate() {
        dict.insert(c, idx + 1);
    }
    for (idx, c) in ('A'..='Z').enumerate() {
        dict.insert(c, idx + 27);
    }

    let mut result = 0;
    let lines: Vec<&str> = input.lines().into_iter().collect();
    let mut idx = 0;
    loop {
        if idx >= lines.len() {
            break;
        }

        let first = lines[idx];
        let second = lines[idx + 1];
        let third = lines[idx + 2];
        let mut seen: HashSet<char> = HashSet::new();
        for c in first.chars() {
            seen.insert(c);
        }

        for c in seen {
            if second.contains(c) && third.contains(c) {
                result += *dict.get(&c).unwrap() as i32;
            }
        }

        idx += 3;
    }

    println!("part two result: {}", result);
    result
}



#[test]
fn test_solve_part_two() {
    let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw".to_string();

    let expected = 70;
    let actual = solve_part_two(input);
    assert_eq!(expected, actual);
}
