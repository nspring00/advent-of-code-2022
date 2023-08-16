use std::fs;

#[derive(PartialEq, Debug)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut score = 0;

    for line in lines {
        let other_hand = parse_hand(&line.chars().next().unwrap());
        let hand = parse_hand(&line.chars().nth(2).unwrap());

        if (hand == Hand::Rock && other_hand == Hand::Scissors) ||
            (hand == Hand::Paper && other_hand == Hand::Rock) ||
            (hand == Hand::Scissors && other_hand == Hand::Paper) {
            score += 6;
        }

        if hand == other_hand {
            score += 3;
        }

        score += match hand {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    score
}

fn part2(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut score = 0;

    for line in lines {
        let other_hand = parse_hand(&line.chars().next().unwrap());
        let result = line.chars().nth(2).unwrap();

        score += match result {
            'X' => 0,
            'Y' => 3,
            'Z' => 6,
            _ => panic!("Invalid result '{}'", result)
        };

        let hand = match result {
            'X' => match other_hand {
                Hand::Rock => Hand::Scissors,
                Hand::Paper => Hand::Rock,
                Hand::Scissors => Hand::Paper,
            },
            'Y' => other_hand,
            'Z' => match other_hand {
                Hand::Rock => Hand::Paper,
                Hand::Paper => Hand::Scissors,
                Hand::Scissors => Hand::Rock,
            },
            _ => panic!("Invalid result '{}'", result)
        };

        score += match hand {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    score
}

fn parse_hand(hand: &char) -> Hand {
    match hand {
        'A' => Hand::Rock,
        'X' => Hand::Rock,
        'B' => Hand::Paper,
        'Y' => Hand::Paper,
        'C' => Hand::Scissors,
        'Z' => Hand::Scissors,
        c => panic!("Invalid move '{}'", c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("A Y\nB X\nC Z"), 15);
        assert_eq!(part1(&fs::read_to_string("src/bin/day-02/input.txt").unwrap()), 11841);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("A Y\nB X\nC Z"), 12);
        assert_eq!(part2(&fs::read_to_string("src/bin/day-02/input.txt").unwrap()), 13022);
    }
}
