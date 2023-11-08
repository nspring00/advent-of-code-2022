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

fn part2_rec<'a>(name: &'a str, monkeys: &'a HashMap<&str, &str>, monkeys_eval: &mut HashMap<&'a str, u64>) -> Option<u64> {
    if name == "humn" {
        return None;
    }

    if monkeys_eval.contains_key(name) {
        return Some(monkeys_eval[name]);
    }

    let raw = monkeys[name];
    let parts = raw.split(" ").collect::<Vec<_>>();
    if parts.len() == 1 {
        let value = parts[0].parse::<u64>().unwrap();
        monkeys_eval.insert(name, value);
        return Some(value);
    }

    assert_eq!(parts.len(), 3);

    let v1 = part2_rec(parts[0], monkeys, monkeys_eval);
    let v2 = part2_rec(parts[2], monkeys, monkeys_eval);

    if name == "root" {
        println!("{}: {:?} {:?}", name, v1, v2);
        assert!(v1.is_some() || v2.is_some());
    }

    if v1.is_none() || v2.is_none() {
        return None;
    }

    let v1 = v1.unwrap();
    let v2 = v2.unwrap();

    let val = match parts[1] {
        "+" => v1 + v2,
        "-" => v1 - v2,
        "*" => v1 * v2,
        "/" => v1 / v2,
        _ => panic!("Unknown operator: {}", parts[2]),
    };

    monkeys_eval.insert(name, val);
    Some(val)
}

fn part2_target_rec<'a>(name: &'a str, target: u64, monkeys: &'a HashMap<&str, &str>, monkeys_eval: &mut HashMap<&'a str, u64>) -> u64 {
    if name == "humn" {
        return target;
    }

    let parts = monkeys[name].split(" ").collect::<Vec<_>>();
    assert_eq!(parts.len(), 3);

    let v1 = part2_rec(parts[0], monkeys, monkeys_eval);
    let v2 = part2_rec(parts[2], monkeys, monkeys_eval);
    assert!((v1.is_some() && v2.is_none()) || (v1.is_none() && v2.is_some()));

    if name == "root" {
        println!("{}: {:?} {:?}", name, v1, v2);

        return if v1.is_some() {
            part2_target_rec(parts[2], v1.unwrap(), monkeys, monkeys_eval)
        } else {
            part2_target_rec(parts[0], v2.unwrap(), monkeys, monkeys_eval)
        };
    }

    if let Some(v1) = v1 {
        let new_target = match parts[1] {
            "+" => target - v1,
            "-" => v1 - target,
            "*" => target / v1,
            "/" => v1 / target,
            _ => panic!("Unknown operator: {}", parts[2]),
        };
        return part2_target_rec(parts[2], new_target, monkeys, monkeys_eval);
    } else if let Some(v2) = v2 {
        let new_target = match parts[1] {
            "+" => target - v2,
            "-" => target + v2,
            "*" => target / v2,
            "/" => v2 * target,
            _ => panic!("Unknown operator: {}", parts[2]),
        };
        return part2_target_rec(parts[0], new_target, monkeys, monkeys_eval);
    } else {
        panic!("No value found for {}", name);
    }
}

fn part2(input: &str) -> u64 {
    let monkeys = input.lines()
        .map(|s| s.split(": "))
        .map(|mut s| (s.next().unwrap(), s.next().unwrap()))
        .collect::<HashMap<_, _>>();

    part2_target_rec("root", 0, &monkeys, &mut HashMap::new())
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
        assert_eq!(part2(TEST_INPUT_1), 301);
        assert_eq!(part2(INPUT), 3247317268284);
    }
}
