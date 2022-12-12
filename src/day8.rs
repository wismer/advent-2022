use std::collections::HashSet;



struct Forest {
    trees: Vec<Vec<usize>>,
    visible_trees: HashSet<(usize, usize)>,
    size: isize
}

impl Forest {
    fn new(input: String) -> Self {
        let mut trees: Vec<Vec<usize>> = vec![];
        for line in input.lines() {
            let as_nums: Vec<usize> = line
                .chars()
                .map(|c| c.to_digit(10)
                .unwrap() as usize)
                .collect();
            trees.push(as_nums);
        }
        let size = trees.len() as isize;
        Forest {
            trees,
            visible_trees: HashSet::new(),
            size
        }
    }

    fn check_proximity(&self, origin_x: isize, origin_y: isize) -> usize {
        let height = *self.get(origin_x, origin_y).unwrap();
        let mut ranges: Vec<usize> = vec![];
        for (x_mod, y_mod) in [(-1isize, 0isize), (0, 1), (1, 0), (0, -1)] {
            let mut blocked = false;
            let mut viewing_range = 0;
            let (mut x, mut y) = (origin_x + x_mod, origin_y + y_mod);
            while !blocked {
                if x < 1 || y < 1 || x > self.size - 2 || y > self.size - 2 {
                    blocked = true;
                }

                viewing_range += 1;
                match self.get(x, y) {
                    Some(next) => {
                        if height <= *next {
                            blocked = true;
                        } else {
                            x += x_mod;
                            y += y_mod;
                        }
                    },
                    None => break
                }
            }
            ranges.push(viewing_range);
        }

        ranges.iter().fold(1, |acc, n| acc * *n)
    }

    fn check_column(&mut self, col: usize) {
        let mut x = 1;
        let mut max = *self.get(x - 1, col as isize).unwrap();
        while x < self.size - 1 {
            println!("x: {}, y: {}", x, col);
            let n = *self.get(x, col as isize).unwrap();
            if n > max {
                self.visible_trees.insert((x as usize, col));
                max = n;
            }
            x += 1;
        }

        max = *self.get(x, col as isize).unwrap();
        x -= 1;
        while x > 0 {
            println!("x: {}, y: {}", x, col);
            let n = *self.get(x, col as isize).unwrap();
            if n > max {
                self.visible_trees.insert((x as usize, col));
                max = n;
            }
            x -= 1;
            
        }
    }

    fn check_row(&mut self, row: usize) {
        let mut y = 1;
        let mut max = *self.get(row as isize, y - 1).unwrap();
        while y < self.size - 1 {
            let n = *self.get(row as isize, y).unwrap();
            if n > max {
                self.visible_trees.insert((row, y as usize));
                max = n;
            }
            y += 1;
        }

        max = *self.get(row as isize, y).unwrap();
        y -= 1;

        while y > 0 {
            let n = *self.get(row as isize, y).unwrap();
            if n > max {
                self.visible_trees.insert((row, y as usize));
                max = n;
            }
            y -= 1;
        }
    }

    fn get(&self, row: isize, col: isize) -> Option<&usize> {
        match self.trees.get(row as usize) {
            Some(r) => r.get(col as usize),
            None => None
        }
    }
}

pub fn solve_part_one(input: String) -> usize {
    let mut forest = Forest::new(input);
    for x in 1..forest.size - 1 {
        forest.check_column(x as usize);
        forest.check_row(x as usize);
    }

    forest.visible_trees.len() + ((forest.size * 4) - 4) as usize
}

pub fn solve_part_two(input: String) -> usize {
    let forest = Forest::new(input);
    let mut result = 0;
    for x in 1..forest.size - 1 {
        
        for y in 1..forest.size - 1 {
            let tree_score = forest.check_proximity(x, y);
            if tree_score > result {
                result = tree_score;
            }
        }
    }

    result
}

const TEST_INPUT: &'static str = "30373
25512
65332
33549
35390";

#[test]
fn test_part_one() {
    assert_eq!(21, solve_part_one(TEST_INPUT.to_owned()));
}


#[test]
fn test_part_two() {
    assert_eq!(8, solve_part_two(TEST_INPUT.to_owned()));
}