use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[derive(Debug)]
struct Rock {
    tiles: Vec<(i32, i32)>,
    width: i32,
    height: i32,
}

impl Rock {
    fn new(tiles: Vec<(i32, i32)>) -> Self {
        Rock {
            width: tiles.iter().map(|(x, _)| x).max().unwrap() + 1,
            height: tiles.iter().map(|(_, y)| y).max().unwrap() + 1,
            tiles,
        }
    }

    fn dummy() -> Self {
        Rock {
            width: 0,
            height: 0,
            tiles: vec![],
        }
    }
}

fn part1(input: &str) -> u32 {
    const N_ROCKS: u32 = 2022;
    // const N_ROCKS: u32 = 20;
    const N_COLS: usize = 7;

    let directions = input.trim().chars().collect::<Vec<_>>();

    let mut heights: [HashSet<i32>; N_COLS] = [
        HashSet::from([-1]),
        HashSet::from([-1]),
        HashSet::from([-1]),
        HashSet::from([-1]),
        HashSet::from([-1]),
        HashSet::from([-1]),
        HashSet::from([-1])
    ];

    // let dummy_rock = Rock::dummy();

    let rocks = vec![
        Rock::new(vec![(0, 0), (1, 0), (2, 0), (3, 0)]),
        Rock::new(vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)]),
        Rock::new(vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]),
        Rock::new(vec![(0, 0), (0, 1), (0, 2), (0, 3)]),
        Rock::new(vec![(0, 0), (0, 1), (1, 0), (1, 1)]),
    ];

    let mut step = 0u32;
    let mut max_y = -1i32;

    // print_field::<N_COLS>(&heights, 2, 3, &rocks[4]);
    // return 0;

    for i in 0..N_ROCKS {
        let rock = &rocks[i as usize % rocks.len()];

        // println!("Rock #{}:", i + 1);

        let mut x = 2usize;
        let mut y = max_y + 4;

        // print_field::<N_COLS>(&heights, max_y, x, y, rock);

        loop {
            let direction = directions[(step % directions.len() as u32) as usize];
            step += 1;

            let old_x = x;
            if direction == '<' {
                if x == 0 {
                    // println!("Jet of gas pushes rock left, but nothing happens:");
                } else {
                    x -= 1;
                    // println!("Jet of gas pushes rock left:");
                }
            } else if direction == '>' {
                if x + rock.width as usize == N_COLS {
                    // println!("Jet of gas pushes rock right, but nothing happens:");
                } else {
                    x += 1;
                    // println!("Jet of gas pushes rock right:");
                }
            } else {
                panic!("Invalid direction: {}", direction);
            }

            // Check if any rock tile collides with the new position
            let mut collision = false;
            for (rock_x, rock_y) in &rock.tiles {
                if heights[x + *rock_x as usize].contains(&(y + *rock_y)) {
                    collision = true;
                    break;
                }
            }
            if collision {
                // println!("Collision detected, rock stays in place:");
                x = old_x;
            }

            // print_field::<N_COLS>(&heights, max_y, x, y, rock);


            y -= 1;
            let mut collision = false;
            for (rock_x, rock_y) in &rock.tiles {
                if heights[x + *rock_x as usize].contains(&(y + *rock_y)) {
                    collision = true;
                    break;
                }
            }
            if collision {
                y += 1;
                for (rock_x, rock_y) in &rock.tiles {
                    heights[x + *rock_x as usize].insert(y + *rock_y);
                    max_y = max_y.max(y + *rock_y);
                }

                // println!("Rock falls 1 unit, causing it to come to rest:");
                // println!("{:?}", heights);
                // print_field::<N_COLS>(&heights, max_y, x, y, &dummy_rock);
                break;
            }

            // println!("Rock falls 1 unit:");
            // print_field::<N_COLS>(&heights, max_y, x, y, rock);
        }
    }


    max_y as u32 + 1
}

fn print_field<const N_COLS: usize>(tiles: &[HashSet<i32>; N_COLS], max_height: i32, rock_x: usize, rock_y: i32, rock: &Rock) {
    let max_y = (rock_y + rock.height).max(max_height + 1);

    for y in (0..max_y).rev() {
        print!("|");
        for x in 0..N_COLS {
            if tiles[x].contains(&y) {
                print!("#");
            } else if rock.tiles.contains(&(x as i32 - rock_x as i32, y - rock_y)) {
                print!("@");
            } else {
                print!(".");
            }
        }
        println!("|");
    }
    println!("+-------+\n");
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
        assert_eq!(part1(TEST_INPUT_1), 3068);
        assert_eq!(part1(INPUT), 3067);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_1), 0);
        assert_eq!(part2(INPUT), 0);
    }
}
