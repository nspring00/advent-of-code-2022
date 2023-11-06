use rayon::prelude::*;
use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[derive(Debug)]
struct Blueprint {
    id: u32,
    recipes: [[u32; 4]; 4],
}

fn compute_bp_score(bp: &Blueprint, time: u32) -> u32 {
    let max_robots = max_useful_robots(bp);
    compute_bp_score_rec(bp, time, &[0; 4], &[1, 0, 0, 0], &max_robots, None, 0)
}

fn compute_bp_score_rec(bp: &Blueprint, time: u32, materials: &[u32; 4], robots: &[u32; 4],
                        max_robots: &[u32; 4], skipped_robots: Option<&Vec<usize>>, best: u32) -> u32 {

    // Inspired by https://github.com/agubelu/Advent-of-Code-2022/blob/master/src/days/day19.rs

    if time == 1 {
        return materials[3] + robots[3];
    }

    // Calculate most optimistic upper bound (1 Geode robot every round) and prune if possible
    if get_upper_bound(time, materials, robots, 3) <= best {
        return 0;
    }

    // If we can't get enough Obsidian anymore, we're done
    if get_upper_bound(time, materials, robots, 2) <= bp.recipes[3][2] {
        return materials[3] + robots[3] * time;
    }

    let mut new_materials = *materials;
    for i in 0..4 {
        new_materials[i] += robots[i];
    }

    // Force build of Geode robot if possible
    if array_ge(&materials, &bp.recipes[3]) {
        new_materials[0] -= bp.recipes[3][0];
        new_materials[2] -= bp.recipes[3][2];
        let mut robots = robots.clone();
        robots[3] += 1;
        return compute_bp_score_rec(bp, time - 1, &new_materials, &robots, max_robots, None, best);
    }

    // Important! Base this calculation off the old materials, not the new ones
    let available_robots = (0..3).filter(|i| array_ge(&materials, &bp.recipes[*i])).collect::<Vec<_>>();
    let mut new_best = best;

    for &r_id in &available_robots {
        // Skip if we already have enough robots for max consumption per round
        if robots[r_id] >= max_robots[r_id] {
            continue;
        }
        // If robot was possible but skipped last round, don't build again
        if let Some(skipped_robots) = skipped_robots {
            if skipped_robots.contains(&r_id) {
                continue;
            }
        }

        let mut new_robots = robots.clone();
        new_robots[r_id] += 1;
        let mut local_materials = new_materials.clone();
        for i in 0..3 {
            local_materials[i] -= bp.recipes[r_id][i];
        }

        let score = compute_bp_score_rec(bp, time - 1, &local_materials, &new_robots, max_robots, None, new_best);
        if score > new_best {
            new_best = score;
        }
    }

    let score = compute_bp_score_rec(bp, time - 1, &new_materials, robots, max_robots, Some(&available_robots), new_best);
    if score > new_best {
        new_best = score;
    }

    new_best
}

fn get_upper_bound(time: u32, materials: &[u32; 4], robots: &[u32; 4], material_id: usize) -> u32 {
    materials[material_id] + robots[material_id] * time + time * (time - 1) / 2
}

fn max_useful_robots(bp: &Blueprint) -> [u32; 4] {
    let mut result = [0, 0, 0, u32::MAX];
    for i in 0..3 {
        result[i] = bp.recipes.iter().map(|x| x[i]).max().unwrap();
    }
    result
}

fn array_ge(a: &[u32; 4], b: &[u32; 4]) -> bool {
    for i in 0..4 {
        if a[i] < b[i] {
            return false;
        }
    }
    true
}

fn part1(input: &str) -> u32 {
    let blueprints = parse_input(input);

    blueprints.par_iter()
        .map(|bp| bp.id * compute_bp_score(bp, 24))
        .sum()
}

fn part2(input: &str) -> u32 {
    let blueprints = parse_input(input);

    blueprints.par_iter()
        .take(3)
        .map(|bp| compute_bp_score(bp, 32))
        .product()
}

fn parse_input(input: &str) -> Vec<Blueprint> {
    let re = Regex::new(r"\d+").unwrap();
    input.lines()
        .map(|line| {
            let ints = re.captures_iter(line)
                .map(|cap| cap[0].parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            let mut bp = Blueprint {
                id: 0,
                recipes: [[0; 4]; 4],
            };
            bp.id = ints[0];
            bp.recipes[0][0] = ints[1];
            bp.recipes[1][0] = ints[2];
            bp.recipes[2][0] = ints[3];
            bp.recipes[2][1] = ints[4];
            bp.recipes[3][0] = ints[5];
            bp.recipes[3][2] = ints[6];
            bp
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = include_str!("test_input.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_parse_input() {
        let result = parse_input(TEST_INPUT_1);
        assert_eq!(result.len(), 2);
        let bp1 = &result[0];
        assert_eq!(bp1.id, 1);
        assert_eq!(bp1.recipes, [[4, 0, 0, 0], [2, 0, 0, 0], [3, 14, 0, 0], [2, 0, 7, 0]]);
        let bp2 = &result[1];
        assert_eq!(bp2.id, 2);
        assert_eq!(bp2.recipes, [[2, 0, 0, 0], [3, 0, 0, 0], [3, 8, 0, 0], [3, 0, 12, 0]]);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT_1), 33);
        assert_eq!(part1(INPUT), 1681);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_1), 56 * 62);
        assert_eq!(part2(INPUT), 5394);
    }
}
