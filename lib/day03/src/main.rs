use std::collections::HashMap;

fn part_one(input: String) -> u16 {
    let mut chars = Vec::new();
    for line in input.lines() {
        let (first, second) = line.split_at(line.len() / 2);
        chars.push(get_common_letter(&first, &second));
    }
    get_value_from_charvec(chars)
}
fn get_value_from_charvec(chars: Vec<char>) -> u16 {
    let mut sum = 0;
    for &ch in chars.iter() {
        let subtract_value = ch.is_lowercase().then_some(96).unwrap_or(38);
        sum += (ch as u16) - subtract_value;
    }
    sum
}
fn get_common_letter(first: &str, second: &str) -> char {
    for letter in first.chars() {
        if second.contains(letter) {
            return letter;
        }
    }
    '\0' // program shouldn't be able to reach this point
}
fn get_common_char_from_rucksacks(vec: Vec<&str>) -> char {
    let mut char_map = HashMap::new();
    for first_ch in vec[0].chars() {
        let mut first_occurence = false;
        if !char_map.contains_key(&first_ch) {
            char_map.insert(first_ch, 1);
            first_occurence = true;
        }

        if let Some(_) = vec[1].chars().position(|ch| ch == first_ch) {
            if first_occurence {
                *char_map.get_mut(&first_ch).unwrap() += 1;
            }
        }
        if let Some(_) = vec[2].chars().position(|ch| ch == first_ch) {
            if first_occurence {
                *char_map.get_mut(&first_ch).unwrap() += 1;
            }
        }
    }
    if
        let Some(result) = char_map
            .iter()
            .find_map(|(key, &val)| if val == 3 { Some(key) } else { None })
    {
        return *result;
    }
    '\0' // shouldn't be able to reach
}
fn part_two(input: String) -> u16 {
    let mut rucksack_container = Vec::new();
    let mut temp_container = Vec::new();

    let mut i = 0;

    for line in input.lines() {
        temp_container.push(line);
        i += 1;

        if i == 3 {
            i = 0;
            rucksack_container.push(temp_container.to_vec());
            temp_container.clear();
        }
    }

    let mut chars = Vec::new();
    for each in rucksack_container.iter() {
        chars.push(get_common_char_from_rucksacks(each.to_vec()));
    }

    get_value_from_charvec(chars)
}

const INPUT: &str =
    "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

fn main() {
    println!("part one: {}", part_one(INPUT.to_string()));
    println!("part two: {}", part_two(INPUT.to_string()));
}

#[cfg(test)]
mod tests {
    use crate::part_one;
    use crate::part_two;

    const INPUT: &str =
        "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part_one_test() {
        assert_eq!(157, part_one(INPUT.to_string()));
    }
    #[test]
    fn part_two_test() {
        assert_eq!(70, part_two(INPUT.to_string()));
    }
}