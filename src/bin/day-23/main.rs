use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let mut elves = input.lines()
        .enumerate()
        .flat_map(|(r, l)| l
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == '#')
            .map(move |(c, _)| (r as i32, c as i32)))
        .collect::<HashSet<_>>();

    let mut dir_order = [0, 2, 3, 1];

    // print_map(&elves);

    for _ in 0..10 {
        elves = simulate_round(&elves, &dir_order);
        dir_order.rotate_left(1);
        // print_map(&elves);
    }

    // Get min/max x and y
    let min_x = *elves.iter().map(|(_, x)| x).min().unwrap();
    let max_x = *elves.iter().map(|(_, x)| x).max().unwrap();
    let min_y = *elves.iter().map(|(y, _)| y).min().unwrap();
    let max_y = *elves.iter().map(|(y, _)| y).max().unwrap();

    (max_y - min_y + 1) as u32 * (max_x - min_x + 1) as u32 - elves.len() as u32
}

fn simulate_round(elves: &HashSet<(i32, i32)>, dir_order: &[usize; 4]) -> HashSet<(i32, i32)> {
    let elves_moves = elves.iter().map(|&pos| (pos, compute_move(pos, &elves, &dir_order))).collect::<HashMap<_, _>>();
    elves_moves.iter().map(|(&pos, &new_pos)| {
        if pos == new_pos {
            return pos;
        }

        // Compute diff in x and y
        let dx = new_pos.0 - pos.0;
        let dy = new_pos.1 - pos.1;

        let other = (pos.0 + 2 * dx, pos.1 + 2 * dy);
        if let Some(&other) = elves_moves.get(&other) {
            if other == new_pos {
                return pos;
            }
        }

        new_pos
    }).collect::<HashSet<_>>()
}

fn compute_move(pos: (i32, i32), elves: &HashSet<(i32, i32)>, dir_order: &[usize; 4]) -> (i32, i32) {
    // Compute 8-neighborhood
    let neighbors = [
        (pos.0 - 1, pos.1 - 1),
        (pos.0 - 1, pos.1),
        (pos.0 - 1, pos.1 + 1),
        (pos.0, pos.1 + 1),
        (pos.0 + 1, pos.1 + 1),
        (pos.0 + 1, pos.1),
        (pos.0 + 1, pos.1 - 1),
        (pos.0, pos.1 - 1),
        (pos.0 - 1, pos.1 - 1),  // Top left again
    ].map(|x| elves.get(&x));

    if neighbors.iter().all(|x| x.is_none()) {
        return pos;
    }

    for &dir in dir_order {
        match dir {
            0 => {
                if neighbors[0..=2].iter().all(|x| x.is_none()) {
                    return (pos.0 - 1, pos.1);
                }
            }
            1 => {
                if neighbors[2..=4].iter().all(|x| x.is_none()) {
                    return (pos.0, pos.1 + 1);
                }
            }
            2 => {
                if neighbors[4..=6].iter().all(|x| x.is_none()) {
                    return (pos.0 + 1, pos.1);
                }
            }
            3 => {
                if neighbors[6..=8].iter().all(|x| x.is_none()) {
                    return (pos.0, pos.1 - 1);
                }
            }
            _ => unreachable!(),
        }
    }


    pos
}

fn print_map(elves: &HashSet<(i32, i32)>) {
    let min_x = (*elves.iter().map(|(_, x)| x).min().unwrap()).min(0);
    let max_x = (*elves.iter().map(|(_, x)| x).max().unwrap()).max(4);
    let min_y = (*elves.iter().map(|(y, _)| y).min().unwrap()).min(0);
    let max_y = (*elves.iter().map(|(y, _)| y).max().unwrap()).max(5);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if elves.contains(&(y, x)) {
                print!("#");
            } else {
                print!(".");
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
    fn test_compute_move() {
        assert_eq!(simulate_round(&HashSet::from([
            (1, 2),
            (1, 3),
            (2, 2),
            (4, 2),
            (4, 3)
        ]), &[0, 2, 3, 1]), HashSet::from([
            (0, 2),
            (0, 3),
            (2, 2),
            (4, 2),
            (3, 3)
        ]));

        assert_eq!(simulate_round(&HashSet::from([
            (0, 2),
            (0, 3),
            (2, 2),
            (4, 2),
            (3, 3)
        ]), &[2, 3, 1, 0]), HashSet::from([
            (1, 2),
            (1, 3),
            (2, 1),
            (3, 4),
            (5, 2)
        ]));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT_1), 110);
        assert_eq!(part1(INPUT), 4075);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_1), 0);
        assert_eq!(part2(INPUT), 0);
    }
}
