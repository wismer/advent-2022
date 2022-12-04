fn parse_line(line: &str) -> ((i32, i32), (i32, i32)) {
    let raw: Vec<(i32, i32)> = line.split(",").map(|section_assignments| {
        let parsed: Vec<i32> = section_assignments.split("-").map(|sections| {
            sections.parse().unwrap()
        }).collect();

        (parsed[0], parsed[1])
    }).collect();
    (raw[0], raw[1])
}

pub fn solve_part_one(input: String) -> i32 {
    let mut fully_contained_count = 0;
    for line in input.lines() {
        let (first, second) = parse_line(line);

        fully_contained_count = if first.0 <= second.0 && first.1 >= second.1 || second.0 <= first.0 && second.1 >= first.1 {
            fully_contained_count + 1
        } else {
            fully_contained_count
        };
    }

    println!("result part one: {}", fully_contained_count);
    fully_contained_count
}

#[test]
fn test_solve_part_one() {
    let test_input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8".to_string();

    let expected = 2;
    let actual = solve_part_one(test_input);

    assert_eq!(expected, actual);
}

pub fn solve_part_two(input: String) -> i32 {
    let mut overlaps = 0;
    for line in input.lines() {
        let (first, second) = parse_line(line);

        let first_range = first.0..=first.1;
        let second_range = second.0..=second.1;
        overlaps = if first_range.contains(&second.0) || first_range.contains(&second.1) {
            overlaps + 1
        } else if second_range.contains(&first.0) || second_range.contains(&first.1) {
            overlaps + 1
        } else {
            overlaps
        }

    }
    println!("result part two: {}", overlaps);
    overlaps
}

#[test]
fn test_solve_part_two() {
    let test_input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8".to_string();

    let expected = 4;
    let actual = solve_part_two(test_input);

    assert_eq!(expected, actual);
}