fn part_one(input: String) -> i32 {
    let mut max = 0;
    let mut current_sum = 0;
    for each in input.lines() {
        if !each.trim().is_empty() {
            current_sum += each.trim().parse::<i32>().unwrap();
        }else {
            if max < current_sum {
                max = current_sum;
            }
            current_sum = 0;
        }
    }
    max
}
fn part_two(input: String) -> i32 {
    let mut totals_carried = Vec::new();
    let mut max = 0;
    let mut current_sum = 0;
    for each in input.lines() {
        if !each.trim().is_empty() {
            current_sum += each.trim().parse::<i32>().unwrap();
        }else {
            totals_carried.push(current_sum);
            if max < current_sum {
                max = current_sum;
            }
            current_sum = 0;
        }
    }
    totals_carried.push(current_sum);

    totals_carried.sort();
    totals_carried.reverse();
    
    let result = totals_carried.iter().take(3).sum();
    result
}

fn main() {
    let input_part_one = 
    "1000
    2000
    3000
    
    4000
    
    5000
    6000
    
    7000
    8000
    9000
    
    10000";
    println!("Part one: {}", part_one(input_part_one.to_string()));
    println!("Part two: {}", part_two(input_part_one.to_string()));
}

#[cfg(test)]
mod tests {
    use crate::part_one;
    use crate::part_two;
    const INPUT: &str = "1000
    2000
    3000
    
    4000
    
    5000
    6000
    
    7000
    8000
    9000
    
    10000";

    #[test]
    fn part_one_test() {
        let expected_result = 24000;
        assert_eq!(expected_result, part_one(INPUT.to_string()));
    }
    #[test]
    fn part_two_test() {
        let expected_result = 45000;
        assert_eq!(expected_result, part_two(INPUT.to_string()));
    }
}