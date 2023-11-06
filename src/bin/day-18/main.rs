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
    let cubes = input.lines()
        .map(|line| line.split(",")
            .map(|s| s.parse::<i32>().unwrap()))
        .map(|mut coords| (coords.next().unwrap(), coords.next().unwrap(), coords.next().unwrap()))
        .collect::<HashSet<_>>();

    let min_x = *cubes.iter().map(|(x, _, _)| x).min().unwrap();
    let max_x = *cubes.iter().map(|(x, _, _)| x).max().unwrap();
    let min_y = *cubes.iter().map(|(_, y, _)| y).min().unwrap();
    let max_y = *cubes.iter().map(|(_, y, _)| y).max().unwrap();
    let min_z = *cubes.iter().map(|(_, _, z)| z).min().unwrap();
    let max_z = *cubes.iter().map(|(_, _, z)| z).max().unwrap();

    // Count the number of surfaces of the cubes that are facing the outside
    let mut surfaces = 0;
    let mut visited = HashSet::new();
    let mut queue = Vec::new();
    queue.push((min_x - 1, min_y - 1, min_z - 1));
    while let Some((x, y, z)) = queue.pop() {
        if x < min_x - 1 || x > max_x + 1 || y < min_y - 1 || y > max_y + 1 || z < min_z - 1 || z > max_z + 1 {
            continue;
        }

        if visited.contains(&(x, y, z)) {
            continue;
        }

        visited.insert((x, y, z));

        assert!(!cubes.contains(&(x, y, z)));

        for (dx, dy, dz) in
        &[(0, 0, 1), (0, 0, -1), (0, 1, 0), (0, -1, 0), (1, 0, 0), (-1, 0, 0)] {
            if cubes.contains(&(x + dx, y + dy, z + dz)) {
                surfaces += 1;
            } else {
                queue.push((x + dx, y + dy, z + dz));
            }
        }
    }

    surfaces
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
        assert_eq!(part2(TEST_INPUT_1), 58);
        assert_eq!(part2(INPUT), 2118);
    }
}
