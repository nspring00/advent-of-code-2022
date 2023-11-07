fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn solve<const ROUNDS: u8>(nums: &[i64]) -> u64 {
    let mut nums: Vec<(usize, &i64)> = nums.iter().enumerate().collect::<Vec<_>>();

    let modulo = nums.len() as i64 - 1;

    for _ in 0..ROUNDS {
        for idx in 0..nums.len() {
            let (old_i, (_, &value)) = nums.iter().enumerate().find(|(_, i)| i.0 == idx).unwrap();
            let new_i = (old_i as i64 + value).rem_euclid(modulo) as usize;

            // Learned about slice rotates from https://github.com/agubelu/Advent-of-Code-2022/blob/master/src/days/day20.rs
            // println!("{} -> {} left", old_i, new_i);
            // println!("Before: {:?} {:?}", nums, indices);
            if new_i > old_i {
                nums[old_i..=new_i].rotate_left(1);
            } else if new_i < old_i {
                nums[new_i..=old_i].rotate_right(1);
            }
            // println!("After:  {:?} {:?}", nums, indices);

            // Alternative solution: remove and insert
            // let removed = nums.remove(old_i);
            // nums.insert(new_i, removed);
        }
    }

    let zero_pos = nums.iter().position(|(_, &i)| i == 0).unwrap();
    let result: i64 = [1000, 2000, 3000].into_iter()
        .map(|x| nums[(zero_pos + x) % nums.len()].1)
        .sum();

    result as u64
}

fn part1(input: &str) -> u64 {
    solve::<1>(&input.trim().lines().map(|l| l.parse().unwrap()).collect::<Vec<_>>())
}

fn part2(input: &str) -> u64 {
    let key = 811_589_153;
    solve::<10>(&input.trim().lines().map(|l| key * l.parse::<i64>().unwrap()).collect::<Vec<_>>())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = include_str!("test_input.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT_1), 3);
        assert_eq!(part1(INPUT), 3346);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_1), 1623178306);
        assert_eq!(part2(INPUT), 4265712588168);
    }
}
