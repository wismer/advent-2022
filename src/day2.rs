use std::{cmp::Ordering};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum Pick {
    Rock,
    Paper,
    Scissors
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct PlayerPick {
    pub pick: Pick,
    pub val: i32
}

impl PlayerPick {
    fn new(pick: char) -> Self {
        let (pick, val) = match pick {
            'A' | 'X' => (Pick::Rock, 1),
            'B' | 'Y' => (Pick::Paper, 2),
            'C' | 'Z' => (Pick::Scissors, 3),
            _ => panic!("whoops")
        };
        PlayerPick {
            pick,
            val
        }
    }

    pub fn lose_against(self, pick: Pick) -> Self {
        match pick {
            Pick::Paper => Self::new('X'),
            Pick::Scissors => Self::new('Y'),
            Pick::Rock => Self::new('Z')
        }
    }
    pub fn win_against(self, pick: Pick) -> Self {
        match pick {
            Pick::Paper => Self::new('Z'),
            Pick::Scissors => Self::new('X'),
            Pick::Rock => Self::new('Y')
        }
    }
}

impl Ord for PlayerPick {
    fn cmp(&self, other: &Self) -> Ordering {
        self.pick.cmp(&other.pick)
    }
}

impl PartialOrd for PlayerPick {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.pick.cmp(&other.pick))
    }

    fn gt(&self, other: &Self) -> bool {
        use Pick::*;
        match self.pick {
            Paper => other.pick == Rock,
            Scissors => other.pick == Paper,
            Rock => other.pick == Scissors
        }
    }

    fn lt(&self, other: &Self) -> bool {
        use Pick::*;

        match self.pick {
            Paper => other.pick == Scissors,
            Scissors => other.pick == Rock,
            Rock => other.pick == Paper
        }
    }
}

pub fn solve_part_one(input: String) -> i32 {
    let mut total_score = 0;
    for line in input.lines() {
        let mut entries = line.chars();
        let opponent_pick = entries.nth(0).unwrap();
        let player_pick = entries.nth(1).unwrap();
        let opponent = PlayerPick::new(opponent_pick);
        let player = PlayerPick::new(player_pick);

        let subtotal = if opponent > player {
            player.val
        } else if opponent < player {
            player.val + 6
        } else {
            player.val + 3
        };

        total_score += subtotal;
    }
    println!("result: {}", total_score);
    total_score
}

#[test]
fn test_part_one() {
    let test_input = "A Y
B X
C Z
".to_string();
    let actual = solve_part_one(test_input);
    let expected = 15;

    assert_eq!(actual, expected);
}

pub fn solve_part_two(input: String) -> i32 {
    let mut total_score = 0;
    for line in input.lines() {
        let mut entries = line.chars();
        let opponent_pick = entries.nth(0).unwrap();
        let player_pick = entries.nth(1).unwrap();
        let opponent = PlayerPick::new(opponent_pick);
        let mut player = PlayerPick::new(player_pick);

        player = match player_pick {
            'X' => player.lose_against(opponent.pick),
            'Z' => player.win_against(opponent.pick),
            'Y' => opponent,
            _ => panic!("why do I do this to myself")
        };

        let subtotal = if opponent > player {
            player.val
        } else if opponent < player {
            player.val + 6
        } else {
            player.val + 3
        };

        total_score += subtotal;
    }
    println!("result: {}", total_score);
    total_score
}

#[test]
fn test_part_two() {
    let test_input = "A Y
B X
C Z
".to_string();
    let actual = solve_part_two(test_input);
    let expected = 12;

    assert_eq!(actual, expected);
}