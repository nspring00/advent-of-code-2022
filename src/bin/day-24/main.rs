use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn part1(input: &str) -> u32 {
    let field = input.lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut storms = field.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &cell)| cell != '#' && cell != '.')
                .map(move |(x, &cell)| (y, x, match cell {
                    '^' => Direction::Up,
                    'v' => Direction::Down,
                    '<' => Direction::Left,
                    '>' => Direction::Right,
                    _ => unreachable!(),
                }))
        })
        .collect::<Vec<_>>();

    let mut obstacle_field = field.iter().map(|row| {
        row.into_iter().map(|&cell| cell == '#').collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    for &(y, x, _) in &storms {
        obstacle_field[y][x] = true;
    }

    let mut obstacle_fields = vec![obstacle_field];
    loop {
        let (storms_new, obstacle_field) = move_storms(&storms, &field);
        if obstacle_field == obstacle_fields[0] {
            break;
        }
        storms = storms_new;
        obstacle_fields.push(obstacle_field);
    }

    // for (i, obstacle_field) in obstacle_fields.iter().enumerate() {
    //     // Print field
    //     println!("After {} minutes:", i);
    //     for row in obstacle_field {
    //         for cell in row {
    //             print!("{}", if *cell { '#' } else { '.' });
    //         }
    //         println!();
    //     }
    //     println!();
    // }

    let start = (0, 1);
    let target = (field.len() - 1, field[0].len() - 2);

    let mut queue = vec![start];
    let mut round = 0;
    let mut visited = HashSet::new();

    loop {
        let obstacle_field = &obstacle_fields[round % obstacle_fields.len()];
        let mut next_queue = Vec::new();

        while let Some((y, x)) = queue.pop() {
            if (y, x) == target {
                return round as u32;
            }

            if obstacle_field[y][x] {
                continue;
            }

            if visited.contains(&(y, x)) {
                continue;
            }
            visited.insert((y, x));

            next_queue.push((y, x));
            if y > 0 {
                next_queue.push((y - 1, x));
            }
            if y < field.len() - 1 {
                next_queue.push((y + 1, x));
            }
            if x > 0 {
                next_queue.push((y, x - 1));
            }
            if x < field[0].len() - 1 {
                next_queue.push((y, x + 1));
            }
        }

        visited.clear();
        queue = next_queue;
        round += 1;
    }
}

fn move_storms(storms: &[(usize, usize, Direction)], field: &[Vec<char>]) -> (Vec<(usize, usize, Direction)>, Vec<Vec<bool>>) {
    let storms = storms.iter()
        .map(|&(y, x, direction)| {
            let (mut y, mut x) = match direction {
                Direction::Up => (y - 1, x),
                Direction::Down => (y + 1, x),
                Direction::Left => (y, x - 1),
                Direction::Right => (y, x + 1),
            };

            if field[y][x] == '#' {
                (y, x) = match direction {
                    Direction::Up => (field.len() - 2, x),
                    Direction::Down => (1, x),
                    Direction::Left => (y, field[0].len() - 2),
                    Direction::Right => (y, 1),
                }
            }

            (y, x, direction)
        })
        .collect::<Vec<_>>();

    let mut obstacle_field = field.into_iter().map(|row| {
        row.into_iter().map(|&cell| cell == '#').collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    for &(y, x, _) in &storms {
        obstacle_field[y][x] = true;
    }

    (storms, obstacle_field)
}

#[allow(dead_code)]
fn print_field(field: &Vec<Vec<char>>, storms: &[(usize, usize, Direction)]) {
    for (y, row) in field.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if storms.iter().any(|&(storm_y, storm_x, _)| storm_y == y && storm_x == x) {
                let storm = storms.iter().find(|&&(storm_y, storm_x, _)| storm_y == y && storm_x == x).unwrap();
                let c = match storm.2 {
                    Direction::Up => '^',
                    Direction::Down => 'v',
                    Direction::Left => '<',
                    Direction::Right => '>',
                };
                print!("{}", c);
            } else if cell == '#' {
                print!("{}", '#');
            } else {
                print!("{}", '.');
            }
        }
        println!();
    }
    println!();
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
        assert_eq!(part1(TEST_INPUT_1), 18);
        assert_eq!(part1(INPUT), 311);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_1), 0);
        assert_eq!(part2(INPUT), 0);
    }
}
