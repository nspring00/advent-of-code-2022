use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let lines: Vec<&[u8]> = input.lines().map(|x| x.as_bytes()).collect();
    let mut sum: i32 = 0;

    for line in lines {
        assert_eq!(line.len() % 2, 0);
        let (arr_1, arr_2) = line.split_at(line.len() / 2);
        let set_1: HashSet<u8> = HashSet::from_iter(arr_1.iter().cloned());
        let set_2: HashSet<u8> = HashSet::from_iter(arr_2.iter().cloned());
        let c = set_1.intersection(&set_2).next().expect("no common value");
        sum += char_value(*c) as i32;
    }

    sum
}

fn part2(input: &str) -> i32 {
    let lines: Vec<&[u8]> = input.lines().map(|x| x.as_bytes()).collect();
    let mut sum: i32 = 0;

    for i in (0..lines.len()).step_by(3) {
        let set_1: HashSet<u8> = HashSet::from_iter(lines[i].iter().cloned());
        let set_2: HashSet<u8> = HashSet::from_iter(lines[i + 1].iter().cloned());
        let set_3: HashSet<u8> = HashSet::from_iter(lines[i + 2].iter().cloned());

        let set_1_2: HashSet<u8> = set_1.intersection(&set_2).cloned().collect();
        let c = set_1_2.intersection(&set_3).next().expect("no common value");
        sum += char_value(*c) as i32;
    }

    sum
}

fn char_value(c: u8) -> u8 {
    if c >= b'a' && c <= b'z' {
        c - b'a' + 1
    } else if c >= b'A' && c <= b'Z' {
        c - b'A' + 27
    } else {
        panic!("Invalid character '{}'", c);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT_1), 157);
        assert_eq!(part1(&fs::read_to_string("src/bin/day-03/input.txt").unwrap()), 8233);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_1), 70);
        assert_eq!(part2(&fs::read_to_string("src/bin/day-03/input.txt").unwrap()), 2821);
    }

    #[test]
    fn test_char_value() {
        assert_eq!(char_value(b'a'), 1);
        assert_eq!(char_value(b'p'), 16);
        assert_eq!(char_value(b'L'), 38);
    }
}
