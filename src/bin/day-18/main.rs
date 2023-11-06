use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let cubes = input.lines()
        .map(|line| line.split(",")
            .map(|s| s.parse::<i32>().unwrap()))
        .map(|mut coords| (coords.next().unwrap(), coords.next().unwrap(), coords.next().unwrap()))
        .collect::<HashSet<_>>();

    let mut surfaces = 0;
    for (x, y, z) in cubes.iter() {
        for (dx, dy, dz) in &[(0, 0, 1), (0, 0, -1), (0, 1, 0),
                              (0, -1, 0), (1, 0, 0), (-1, 0, 0)] {
            if !cubes.contains(&(x + dx, y + dy, z + dz)) {
                surfaces += 1;
            }
        }
    }

    surfaces
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
        assert_eq!(part1(TEST_INPUT_1), 64);
        assert_eq!(part1(INPUT), 3650);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_1), 0);
        assert_eq!(part2(INPUT), 0);
    }
}
