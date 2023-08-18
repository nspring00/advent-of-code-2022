fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let sums = parse_sums(input);
    sums.iter().max().unwrap().clone()
}

fn part2(input: &str) -> i32 {
    let mut sums = parse_sums(input);
    sums.sort();
    sums[sums.len() - 1] + sums[sums.len() - 2] + sums[sums.len() - 3]
}

fn parse_sums(input: &str) -> Vec<i32> {
    let lines: Vec<&str> = input.lines().collect();
    let mut sums: Vec<i32> = Vec::new();

    let mut sum = 0;
    for line in lines {
        if line.is_empty() {
            sums.push(sum);
            sum = 0;
        } else {
            let num: i32 = line.parse().unwrap();
            sum += num;
        }
    }

    if sum != 0 {
        sums.push(sum);
    }

    sums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000"), 24000);
        assert_eq!(part1(include_str!("input.txt")), 70296);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000"), 45000);
        assert_eq!(part2(include_str!("input.txt")), 205381);
    }
}
