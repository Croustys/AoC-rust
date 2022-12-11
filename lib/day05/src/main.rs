fn part_one(input: &str) -> &str {
    for line in input.lines() {
        println!("{}", line);
    }
    "A"
}

fn main() {
    let input = include_str!("test.txt");
    part_one(input);
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_one() {
        let input = include_str!("test.txt");
        assert_eq!("CMZ", part_one(input));
    }
}