#[derive(Debug)]
struct Section {
    start: i32,
    end: i32,
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let pairs = input.lines().map(parse_sections).collect::<Vec<_>>();
    let mut result: i32 = 0;

    for pair in pairs {
        if pair.0.start <= pair.1.start && pair.0.end >= pair.1.end
            || pair.1.start <= pair.0.start && pair.1.end >= pair.0.end {
            result += 1;
        }
    }

    result
}

fn part2(input: &str) -> i32 {
    let pairs = input.lines().map(parse_sections).collect::<Vec<_>>();
    let mut result: i32 = 0;

    for pair in pairs {
        if pair.0.start <= pair.1.start && pair.0.end >= pair.1.start
            || pair.1.start <= pair.0.start && pair.1.end >= pair.0.start {
            result += 1;
        }
    }

    result
}

fn parse_sections(line: &str) -> (Section, Section) {
    let (s1_str, s2_str) = line.split_once(',').unwrap();
    let (s1_start, s1_end) = s1_str.split_once('-').unwrap();
    let (s2_start, s2_end) = s2_str.split_once('-').unwrap();
    let s1 = Section {
        start: s1_start.parse::<i32>().unwrap(),
        end: s1_end.parse::<i32>().unwrap(),
    };
    let s2 = Section {
        start: s2_start.parse::<i32>().unwrap(),
        end: s2_end.parse::<i32>().unwrap(),
    };
    (s1, s2)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8"), 2);
        assert_eq!(part1(include_str!("input.txt")), 534);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8"), 4);
        assert_eq!(part2(include_str!("input.txt")), 841);
    }
}
