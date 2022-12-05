#[derive(Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize
}

impl Move {
    pub fn move_crates(self, crates: &mut Vec<Vec<char>>, reverse: bool) {
        let mut crates_to_move: Vec<char> = vec![];
        let from_crate = crates.get_mut(self.from - 1).unwrap();

        if reverse {
            let take_idx = from_crate.len() - self.count;
            crates_to_move = from_crate.split_off(take_idx);
        } else {
            for _ in 0..self.count {
                let elf_crate = from_crate.pop().unwrap();
                crates_to_move.push(elf_crate);
            }
        }
        let to_crate = crates.get_mut(self.to - 1).unwrap();
        to_crate.append(&mut crates_to_move);

    }
}

fn parse_crate_status(raw_status: String) -> Vec<Vec<char>> {
    let mut lines = raw_status.lines().rev();
    let mut cols: Vec<Vec<char>> = lines
        .next()
        .unwrap()
        .matches(char::is_alphanumeric)
        .map(|_| vec![])
        .collect();

    for line in lines {
        let (mut idx, mut col_idx) = (1, 0usize);
        while let Some(label) = line.get(idx..(idx + 1)) {
            let label_as_char = label.parse::<char>().unwrap();
            match cols.get_mut(col_idx) {
                Some(col) => if label_as_char != ' ' {
                    col.push(label_as_char)
                },
                None => {
                    println!("what, {}", col_idx);
                }
            }
            idx += 4;
            col_idx += 1;
        }
    }

    cols
}


fn parse_moves(raw_moves: String) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    for line in raw_moves.lines() {
        let ln: Vec<usize> = line.split(" ").enumerate().filter_map(|(idx, s)| {
            match idx {
                1 | 3 | 5 => s.parse::<usize>().ok(),
                _ => None
            }
        }).collect();
        moves.push(
            Move {
                count: ln[0],
                from: ln[1],
                to: ln[2] 
            }
        )

    }

    moves
}

fn solve(input: String, reverse: bool) -> String {
    let raw_sections: Vec<&str> = input.split("\n\n").collect();
    let mut crates = parse_crate_status(raw_sections[0].to_owned());
    let moves = parse_moves(raw_sections[1].to_owned());

    for crate_move in moves {
        crate_move.move_crates(&mut crates, reverse);
    }

    let top_crates = crates
        .iter()
        .map(|col| *col.last().unwrap())
        .fold(String::new(), |mut acc, c| {
            acc.push(c);
            acc
        });
        
    top_crates
}

pub fn solve_part_one(input: String) -> String {
    solve(input, false)
}

pub fn solve_part_two(input: String) -> String {
    solve(input, true)
}

#[test]
fn test_solve_part_one() {
    let sample_input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2".to_string();
    let expected = "CMZ";
    let actual = solve_part_one(sample_input);
    assert_eq!(expected, actual);
}

#[test]
fn test_solve_part_two() {
    let sample_input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2".to_string();
    let expected = "MCD";
    let actual = solve_part_two(sample_input);
    assert_eq!(expected, actual);
}