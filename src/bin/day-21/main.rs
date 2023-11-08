use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}


fn part1_rec<'a>(name: &'a str, monkeys: &'a HashMap<&str, &str>, monkeys_eval: &mut HashMap<&'a str, u64>) -> u64 {
    if monkeys_eval.contains_key(name) {
        return monkeys_eval[name];
    }

    let raw = monkeys[name];
    let parts = raw.split(" ").collect::<Vec<_>>();
    if parts.len() == 1 {
        let value = parts[0].parse::<u64>().unwrap();
        monkeys_eval.insert(name, value);
        return value;
    }

    assert_eq!(parts.len(), 3);

    let v1 = part1_rec(parts[0], monkeys, monkeys_eval);
    let v2 = part1_rec(parts[2], monkeys, monkeys_eval);
    let val = match parts[1] {
        "+" => v1 + v2,
        "-" => v1 - v2,
        "*" => v1 * v2,
        "/" => v1 / v2,
        _ => panic!("Unknown operator: {}", parts[2]),
    };

    monkeys_eval.insert(name, val);
    val
}

fn part1(input: &str) -> u64 {
    let monkeys = input.lines()
        .map(|s| s.split(": "))
        .map(|mut s| (s.next().unwrap(), s.next().unwrap()))
        .collect::<HashMap<_, _>>();

    part1_rec("root", &monkeys, &mut HashMap::new())
}

fn part2(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = include_str!("test_input.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT_1), 152);
        assert_eq!(part1(INPUT), 169525884255464);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_1), 0);
        assert_eq!(part2(INPUT), 0);
    }
}
