pub fn solve_part_one(input: String) -> i32 {
    let mut max = 0;
    let mut lines = input.lines();
    let mut subtotal = 0;
    for line in lines {
        println!("{}", line);
        match line.parse::<i32>() {
            Ok(n) => subtotal += n,
            Err(_) => {
                if subtotal > max {
                    max = subtotal;
                }
                subtotal = 0;
            }
        }
    }

    println!("result part one: {}", max);
    max
}

#[test]
fn test_solve_part_one() {
    let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000
    
10000";
    let actual = solve_part_one(input.to_string());
    let expected = 24000;
    assert_eq!(actual, expected);
}

pub fn solve_part_two(input: String) -> i32 {
    let mut max = 0;
    let mut lines = input.lines();
    let mut subtotal = 0;
    let mut top_three = (0, 0, 0);

    for line in lines {
        println!("LINE: {}", line);
        let (first, second, third) = top_three;
        match line.parse::<i32>() {
            Ok(n) => subtotal += n,
            Err(_) => {
                if subtotal > first {
                    top_three.0 = subtotal;
                    subtotal = first;
                }

                if subtotal > second {
                    top_three.1 = subtotal;
                    subtotal = second;
                }

                if subtotal > third {
                    top_three.2 = subtotal;
                    subtotal = third;
                }
                println!("{} wtf: {:?}", subtotal, top_three);

                subtotal = 0;
            }
        }

        println!("SUBTOTAL: {}", subtotal);
    }


    let result = top_three.0 + top_three.1 + top_three.2;
    println!("result part two: {}", result);
    result
}


#[test]
fn test_solve_part_two() {
    let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000
    
10000

";
    let actual = solve_part_two(input.to_string());
    let expected = 45000;
    assert_eq!(actual, expected);
}