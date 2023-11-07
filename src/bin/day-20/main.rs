fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let mut nums = input.lines().map(|x| x.parse::<i32>().unwrap()).enumerate().collect::<Vec<_>>();

    let modulo = nums.len() as i32 - 1;

    for idx in 0..nums.len() {

        let (old_i, (_, value)) = nums.iter().enumerate().find(|(_, i)| i.0 == idx).unwrap();
        let new_i = (old_i as i32 + value).rem_euclid(modulo) as usize;

        // Learned about slice rotates from https://github.com/agubelu/Advent-of-Code-2022/blob/master/src/days/day20.rs
        if new_i > old_i {
            // println!("{} -> {} left", old_i, new_i);
            // println!("Before: {:?} {:?}", nums, indices);
            nums[old_i..=new_i].rotate_left(1);
            // println!("After:  {:?} {:?}", nums, indices);
        } else if new_i < old_i {
            // println!("{} -> {} right", old_i, new_i);
            // println!("Before: {:?} {:?}", nums, indices);
            nums[new_i..=old_i].rotate_right(1);
            // println!("After:  {:?} {:?}", nums, indices);
        }
    }

    let zero_pos = nums.iter().position(|(_, i)| *i == 0).unwrap();
    let result: i32 = [1000, 2000, 3000].into_iter()
        .map(|x| nums[(zero_pos + x) % nums.len()].1)
        .sum();

    result as u32
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
        assert_eq!(part1(TEST_INPUT_1), 3);
        assert_eq!(part1(INPUT), 3346);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_1), 0);
        assert_eq!(part2(INPUT), 0);
    }
}
