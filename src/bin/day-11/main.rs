use std::fmt::Debug;
use std::fs;
use std::str::FromStr;
use regex::Regex;

struct Monkey {
    id: usize,
    items: Vec<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    divider: u64,
    true_monkey: usize,
    false_monkey: usize,
    inspect_count: u32,
}

impl Monkey {
    fn new(id: usize, items: Vec<u64>, operation: Box<dyn Fn(u64) -> u64>, divider: u64, true_monkey: usize, false_monkey: usize) -> Monkey {
        Monkey {
            id,
            items,
            operation,
            divider,
            true_monkey,
            false_monkey,
            inspect_count: 0,
        }
    }

    fn inspect(&mut self) -> (usize, u64) {
        self.inspect_count += 1;
        let item = self.items.remove(0);
        //println!("  Monkey inspects an item with a worry level of {}.", item);
        let mut worry_level = (self.operation)(item);
        //println!("    Worry level is {}.", worry_level);
        worry_level = worry_level / 3;
        //println!("    Monkey gets bored with item. Worry level is divided by 3 to {}.", worry_level);
        let receiver = if worry_level % self.divider == 0 {
            //println!("    Current worry level is divisible by {}.", self.divider);
            self.true_monkey
        } else {
            //println!("    Current worry level is divisible by {}.", self.divider);
            self.false_monkey
        };
        //println!("    Item with worry level {} is thrown to monkey {}.", worry_level, receiver);

        (receiver, worry_level)
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> u32 {
    part1_parsed(&mut parse_input(input))
}

fn part2(input: &str) -> u32 {
    0
}

fn part1_parsed(monkeys: &mut [Monkey]) -> u32 {
    let rounds = 20;

    for round in 0..rounds {
        println!("#### Round {}:", round + 1);

        for i in 0..monkeys.len() {
            // println!("Monkey {}:", i);
            let monkey = &mut monkeys[i];
            let mut moved_items = Vec::new();
            while monkey.items.len() > 0 {
                moved_items.push(monkey.inspect());
            }

            for (receiver, worry_level) in moved_items {
                monkeys[receiver].items.push(worry_level);
            }
        }

        for monkey in monkeys.iter() {
            println!("Monkey {}: {}", monkey.id, monkey.items.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "));
        }
    }

    let mut inspect_counts = monkeys.iter().map(|x| x.inspect_count).collect::<Vec<_>>();
    inspect_counts.sort();
    inspect_counts[inspect_counts.len() - 1] * inspect_counts[inspect_counts.len() - 2]
}

fn parse_input(input: &str) -> Vec<Monkey> {
    let lines = input.lines().collect::<Vec<_>>();
    let mut monkeys = Vec::new();

    let id_re = Regex::new(r"Monkey (\d+):").unwrap();


    for i in (0..lines.len()).step_by(7) {
        let id = re_capture_1::<usize>(&id_re, lines[i]);
        let items = &lines[i + 1][18..].split(", ")
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        let operation_str = &lines[i + 2][23..];
        let operation: Box<dyn Fn(u64) -> u64> = if operation_str.chars().next().unwrap() == '+' {
            let other = operation_str[2..].parse::<u64>().unwrap();
            Box::new(move |x| x + other)
        } else if operation_str == "* old" {
            Box::new(|x| x * x)
        } else {
            let other = operation_str[2..].parse::<u64>().unwrap();
            Box::new(move |x| x * other)
        };
        let divider = re_capture_1::<u64>(&Regex::new(r"Test: divisible by (\d+)").unwrap(), lines[i + 3]);
        let true_monkey = re_capture_1::<usize>(&Regex::new(r"If true: throw to monkey (\d+)").unwrap(), lines[i + 4]);
        let false_monkey = re_capture_1::<usize>(&Regex::new(r"If false: throw to monkey (\d+)").unwrap(), lines[i + 5]);

        monkeys.push(Monkey::new(id, items.to_vec(), operation, divider, true_monkey, false_monkey));
    }

    monkeys
}

fn re_capture_1<T: FromStr>(re: &Regex, line: &str) -> T where <T as FromStr>::Err: Debug {
    re.captures(line).unwrap().get(1).unwrap().as_str().parse::<T>().unwrap()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    fn parsed_test_input() -> [Monkey; 4] {
        [
            Monkey::new(0, vec![79, 98], Box::from(|x| x * 19), 23, 2, 3),
            Monkey::new(1, vec![54, 65, 75, 74], Box::from(|x| x + 6), 19, 2, 0),
            Monkey::new(2, vec![79, 60, 97], Box::from(|x| x * x), 13, 1, 3),
            Monkey::new(3, vec![74], Box::from(|x| x + 3), 17, 0, 1)
        ]
    }

    #[test]
    fn test_part1_no_parsing() {
        assert_eq!(part1_parsed(&mut parsed_test_input()), 10605);
    }

    #[test]
    fn test_part1() {
        let test_input = &fs::read_to_string("src/bin/day-11/test_input.txt").unwrap();
        assert_eq!(part1_parsed(&mut parse_input(test_input)), 10605);

        let input = &fs::read_to_string("src/bin/day-11/input.txt").unwrap();
        assert_eq!(part1_parsed(&mut parse_input(input)), 88208);
    }
}

