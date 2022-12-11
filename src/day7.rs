use std::{collections::{HashMap, HashSet}, cmp::min_by};
use std::cmp::min;

fn remove (path: &mut String) {
    path.pop();
    while !path.is_empty() && path.chars().last().unwrap() != '/' {
        path.pop();
    }
}

fn derivatives (s: String) -> Vec<String> {
    let mut gamma = s;
    let mut vec: Vec<String> = vec![];
    while gamma.len() != 1 {
        vec.push(gamma.clone());
        remove(&mut gamma);
    }
    return vec;
}

pub fn solve_part_one(input: String) -> usize {
    let mut path: String = "/".to_string();
    let mut get_files: HashMap<String, (usize, HashSet<String>)> = HashMap::new();
    get_files.insert("/".to_string(), (0, HashSet::new()));
    for line in input.lines() {
        let s = line.to_string();
        if s.is_empty() {
            break;
        }
        let vector: Vec<&str> = s.split(' ').collect();
        if s.starts_with("$ cd ..") {
            remove(&mut path);
        } else if s.starts_with("$ cd /") {
            path = "/".to_string();
        } else if s.starts_with("$ cd ") {
            path.push_str(vector[2]);
            path.push('/');
        } else if s.starts_with("$ ls") {
            continue
        } else if s.starts_with("dir ") {
            let mut x = path.clone();
            x.push_str(&vector[1]);
            x.push('/');
            get_files.insert(x, (0, HashSet::new()));
        } else {
            for x in derivatives(path.clone()) {
                let gamma = get_files.get_mut(&x).unwrap();
                let sz: usize = vector[0].parse().unwrap();
                gamma.1.insert(vector[1].to_string());
                gamma.0 += sz;
            }
        }
    }
    let mut sum = 0;
    for (key, value) in get_files.into_iter() {
        if value.0 <= 100000 {
            sum += value.0;
        }
    }

    sum
}


pub fn solve_part_two(input: String) -> usize {
    let mut path: String = "/".to_string();
    let mut get_files: HashMap<String, (usize, HashSet<String>)> = HashMap::new();
    get_files.insert("/".to_string(), (0, HashSet::new()));
    for line in input.lines() {
        let s = line.to_string();
        if s.is_empty() {
            break;
        }
        let vector: Vec<&str> = s.split(' ').collect();
        if s.starts_with("$ cd ..") {
            remove(&mut path);
        } else if s.starts_with("$ cd /") {
            path = "/".to_string();
        } else if s.starts_with("$ cd ") {
            path.push_str(vector[2]);
            path.push('/');
        } else if s.starts_with("$ ls") {
            continue
        } else if s.starts_with("dir ") {
            let mut x = path.clone();
            x.push_str(&vector[1]);
            x.push('/');
            get_files.insert(x, (0, HashSet::new()));
        } else {
            for x in derivatives(path.clone()) {
                let gamma = get_files.get_mut(&x).unwrap();
                let sz: usize = vector[0].parse().unwrap();
                gamma.1.insert(vector[1].to_string());
                gamma.0 += sz;
            }
        }
    }
    let mut sum = get_files.get("/").unwrap();
    let mut unused = 3000000;
    let mut ans: usize = 7000000;
    for (value, key) in get_files.into_iter() {
        println!("\ntest: {:?}", key.0);
        if key.0 >= unused {
            ans = min(ans, key.0);
        }
    }

    ans
}

const TEST_INPUT: &'static str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

#[test]
fn test_solve_part_one() {
    let actual = solve_part_one(TEST_INPUT.to_owned());
    assert_eq!(95437, actual);
}

#[test]
fn test_solve_part_two() {
    let actual = solve_part_two(TEST_INPUT.to_owned());
    assert_eq!(24933642, actual);
}

