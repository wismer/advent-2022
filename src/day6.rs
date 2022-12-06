use std::collections::HashSet;

struct DataParser {
    chars: Vec<char>
}

impl DataParser {
    fn check(&self, packet_length: usize) -> Result<usize, usize> {
        let mut seen: HashSet<char> = HashSet::new();
        let mut c_idx = 0;

        loop {
            let c = &self.chars[c_idx];
            if seen.contains(c) {
                c_idx -= seen.len() - 1;
                seen.clear();
            } else {
                seen.insert(*c);
                c_idx += 1;
            }

            if seen.len() == packet_length {
                break;
            }
        }

        Ok(c_idx)
    }
}

pub fn solve_part_one(input: String) -> usize {
    let parser = DataParser { chars: input.chars().collect() };

    match parser.check(4) {
        Ok(idx) => idx,
        Err(_) => panic!("not right")
    }
}

pub fn solve_part_two(input: String) -> usize {
    let parser = DataParser { chars: input.chars().collect() };

    match parser.check(14) {
        Ok(n) => n,
        Err(_) => panic!("not right")
    }
}

#[test]
fn test_part_one() {
    let test_input_sample_one = "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string();
    let test_input_sample_two = "nppdvjthqldpwncqszvftbrmjlhg".to_string();
    let test_input_sample_three = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string();
    let test_input_sample_four = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string();

    let (first, second, third, fourth) = (5, 6, 10, 11);
    let (actual_first, actual_second, actual_third, actual_fourth) = (
        solve_part_one(test_input_sample_one),
        solve_part_one(test_input_sample_two),
        solve_part_one(test_input_sample_three),
        solve_part_one(test_input_sample_four)
    );

    assert_eq!(first, actual_first);
    assert_eq!(second, actual_second);
    assert_eq!(third, actual_third);
    assert_eq!(fourth, actual_fourth);
}

#[test]
fn test_solve_part_two() {
    let test_input_sample_one = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string();
    let test_input_sample_two = "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string();
    let test_input_sample_three = "nppdvjthqldpwncqszvftbrmjlhg".to_string();
    let test_input_sample_four = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string();
    let test_input_sample_five = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string();

    let (first, second, third, fourth, fifth) = (19, 23, 23, 29, 26);
    let (actual_first, actual_second, actual_third, actual_fourth, actual_fifth) = (
        solve_part_two(test_input_sample_one),
        solve_part_two(test_input_sample_two),
        solve_part_two(test_input_sample_three),
        solve_part_two(test_input_sample_four),
        solve_part_two(test_input_sample_five)
    );

    assert_eq!(first, actual_first);
    assert_eq!(second, actual_second);
    assert_eq!(third, actual_third);
    assert_eq!(fourth, actual_fourth);
    assert_eq!(fifth, actual_fifth);
}