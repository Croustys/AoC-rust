use std::collections::VecDeque;

#[derive(Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl Move {
    fn from_line(line: &str) -> Self {
        let mut split = line.split(|c| c == ' ');
        let count = split.nth(1).unwrap().parse::<usize>().unwrap();
        let from = split.nth(1).unwrap().parse::<usize>().unwrap();
        let to = split.nth(1).unwrap().parse::<usize>().unwrap();
        Self { count, from, to }
    }
}

fn part_one(input: &str) -> String {
    let mut stacks: Vec<VecDeque<u8>> = vec![VecDeque::new(); 9];
    input
        .lines()
        .take(8)
        .for_each(|l| {
            let mut elements = [None; 9];
            for (i, p) in (1..l.len()).step_by(4).enumerate() {
                let c = l.as_bytes()[p];
                elements[i] = if c == b' ' { None } else { Some(c) };
            }
            for (i, e) in elements
                .into_iter()
                .enumerate()
                .filter(|(_, e)| e.is_some()) {
                stacks[i].push_back(e.unwrap());
            }
        });

    input
        .lines()
        .skip(10)
        .map(Move::from_line)
        .for_each(|m| {
            for _ in 0..m.count {
                let elem = stacks[m.from - 1].pop_front().unwrap();
                stacks[m.to - 1].push_front(elem);
            }
        });

    let mut result = String::from("");
    for i in stacks.iter() {
        if i.len() < 1 {
            continue;
        }
        let ch = i[0] as char;
        result.push(ch);
    }
    result
}
fn part_two(input: &str) -> String {
    let mut stacks: Vec<VecDeque<u8>> = vec![VecDeque::new(); 9];
    input
        .lines()
        .take(8)
        .for_each(|l| {
            let mut elements = [None; 9];
            for (i, p) in (1..l.len()).step_by(4).enumerate() {
                let c = l.as_bytes()[p];
                elements[i] = if c == b' ' { None } else { Some(c) };
            }
            for (i, e) in elements
                .into_iter()
                .enumerate()
                .filter(|(_, e)| e.is_some()) {
                stacks[i].push_back(e.unwrap());
            }
        });

    input
        .lines()
        .skip(10)
        .map(Move::from_line)
        .for_each(|m| {
            let mut temp_stacks = VecDeque::with_capacity(m.count);
            for _ in 0..m.count {
                temp_stacks.push_front(stacks[m.from - 1].pop_front().unwrap());
            }
            temp_stacks.iter().for_each(|e| {
                stacks[m.to - 1].push_front(*e);
            });
        });

    let mut result = String::from("");
    for i in stacks.iter() {
        if i.len() < 1 {
            continue;
        }
        let ch = i[0] as char;
        result.push(ch);
    }
    result
}

fn main() {
    let input = include_str!("test.txt");
    //println!("{}", part_one(input));
    println!("{}", part_two(input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_one() {
        let input = include_str!("test.txt");
        assert_eq!("CMZ", part_one(input));
    }
    #[test]
    fn test_two() {
        let input = include_str!("test.txt");
        assert_eq!("MCD", part_two(input));
    }
}