use std::cmp::{max, min};
use std::io::Write;

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let mut positions = Vec::new();
    for line in input.lines() {
        let mut last_pos = (-1, -1);
        for coord in line.split(" -> ") {
            let (x_str, y_str) = coord.split_once(",").unwrap();
            let pos = (x_str.parse::<i32>().unwrap(), y_str.parse::<i32>().unwrap());
            if last_pos != (-1, -1) {
                assert_eq!(!(pos.0 != last_pos.0 && pos.1 != last_pos.1), true);
                if pos.0 != last_pos.0 {
                    for x in min(last_pos.0, pos.0)..(max(last_pos.0, pos.0) + 1) {
                        positions.push((x, pos.1));
                    }
                } else {
                    for y in min(last_pos.1, pos.1)..(max(last_pos.1, pos.1) + 1) {
                        positions.push((pos.0, y));
                    }
                }
            }
            last_pos = pos;
        }
    }

    // println!("{:?}", positions);

    let min_x = min(500, positions.iter().map(|x| x.0).min().unwrap()) - 1;
    let max_x = max(500, positions.iter().map(|x| x.0).max().unwrap()) + 1;
    let min_y = min(0, positions.iter().map(|x| x.1).min().unwrap());
    let max_y = max(0 , positions.iter().map(|x| x.1).max().unwrap()) + 1;

    // println!("{} {} {} {}", min_x, max_x, min_y, max_y);

    let mut grid = vec![vec![false; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];
    for pos in positions {
        grid[(pos.1 - min_y) as usize][(pos.0 - min_x) as usize] = true;
    }

    let spawn = (500 - min_x as usize, 0 - min_y as usize);
    let mut count = 0;
    loop {

        let mut pos = spawn;
        loop {

            // Print grid, use '.' for false and '#' for true
            /*for y in 0..grid.len() {
                for x in 0..grid[y].len() {
                    if pos == (x, y) {
                        print!("o");
                        continue;
                    }
                    print!("{}", if grid[y][x] { '#' } else { '.' });
                }
                println!();
            }
            println!("\n###########################");*/

            if pos.1 >= grid.len() - 1 {
                return count;
            }
            if !grid[pos.1 + 1][pos.0] {
                pos.1 += 1;
            } else if !grid[pos.1 + 1][pos.0 - 1] {
                pos.0 -= 1;
                pos.1 += 1;
            } else if !grid[pos.1 + 1][pos.0 + 1] {
                pos.0 += 1;
                pos.1 += 1;
            } else {
                grid[pos.1][pos.0] = true;
                break;
            }
        }

        count += 1;
    }
}

fn part2(input: &str) -> u32 {
    let mut positions = Vec::new();
    for line in input.lines() {
        let mut last_pos = (-1, -1);
        for coord in line.split(" -> ") {
            let (x_str, y_str) = coord.split_once(",").unwrap();
            let pos = (x_str.parse::<i32>().unwrap(), y_str.parse::<i32>().unwrap());
            if last_pos != (-1, -1) {
                assert_eq!(!(pos.0 != last_pos.0 && pos.1 != last_pos.1), true);
                if pos.0 != last_pos.0 {
                    for x in min(last_pos.0, pos.0)..(max(last_pos.0, pos.0) + 1) {
                        positions.push((x, pos.1));
                    }
                } else {
                    for y in min(last_pos.1, pos.1)..(max(last_pos.1, pos.1) + 1) {
                        positions.push((pos.0, y));
                    }
                }
            }
            last_pos = pos;
        }
    }

    // println!("{:?}", positions);

    let mut min_x = min(500, positions.iter().map(|x| x.0).min().unwrap());
    let mut max_x = max(500, positions.iter().map(|x| x.0).max().unwrap());
    let min_y = min(0, positions.iter().map(|x| x.1).min().unwrap());
    let max_y = max(0 , positions.iter().map(|x| x.1).max().unwrap()) + 2;

    let addition = max_y - min_y;
    min_x -= addition;
    max_x += addition;

    println!("{} {} {} {}", min_x, max_x, min_y, max_y);

    let mut grid = vec![vec![false; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];
    for pos in positions {
        grid[(pos.1 - min_y) as usize][(pos.0 - min_x) as usize] = true;
    }

    let last_i = grid.len() - 1;
    for x in 0..grid[0].len() {
        grid[last_i][x] = true;
    }

    /*for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            print!("{}", if grid[y][x] { '#' } else { '.' });
        }
        println!();
    }
    println!("\n###########################");*/

    let spawn = (500 - min_x as usize, 0 - min_y as usize);
    let mut count = 0;
    loop {

        let mut pos = spawn;
        loop {

            // Print grid, use '.' for false and '#' for true
            /*for y in 0..grid.len() {
                for x in 0..grid[y].len() {
                    if pos == (x, y) {
                        print!("o");
                        continue;
                    }
                    print!("{}", if grid[y][x] { '#' } else { '.' });
                }
                println!();
            }
            println!("\n###########################");*/

            if pos.1 >= grid.len() - 1 {
                panic!("Reached bottom of grid");
            }
            if !grid[pos.1 + 1][pos.0] {
                pos.1 += 1;
            } else if !grid[pos.1 + 1][pos.0 - 1] {
                pos.0 -= 1;
                pos.1 += 1;
            } else if !grid[pos.1 + 1][pos.0 + 1] {
                pos.0 += 1;
                pos.1 += 1;
            } else {
                if pos == spawn {
                    return count + 1;
                }
                grid[pos.1][pos.0] = true;
                break;
            }
        }

        count += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9";
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT_1), 24);
        assert_eq!(part1(INPUT), 1199);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_1), 93);
        assert_eq!(part2(INPUT), 23925);
    }
}
