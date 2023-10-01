use std::cmp::min;
use std::collections::HashMap;
use std::ops::Index;
use itertools::{enumerate, Itertools};
use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));

    return;
    const TEST_INPUT_1: &str = include_str!("test_input.txt");
    let (valves, distances) = parse_input(TEST_INPUT_1);
    let result = evaluate_path(&vec![&"DD", &"BB", &"JJ", &"HH", &"EE", &"CC"], &valves, &distances);
    println!("Result: {}", result);
}

#[derive(Debug, Eq, PartialEq)]
struct Valve {
    label: String,
    flow_rate: u32,
    next_valves: Vec<String>,
}

#[derive(Debug, Clone)]
struct ValveSystem {
    valves: HashMap<String, bool>,
    open_flow: u32,
    total_flow: u32,
    time: u32,
    position: String,
}

impl ValveSystem {
    fn new(valves: &HashMap<String, Valve>) -> ValveSystem {
        ValveSystem {
            valves: valves.keys().map(|x| (x.to_string(), false)).collect(),
            open_flow: 0,
            total_flow: 0,
            time: 0,
            position: "AA".to_string(),
        }
    }

    fn tick(&mut self) {
        self.total_flow += self.open_flow;
        self.time += 1;
    }

    fn open_valve(&mut self, valve: &Valve) {
        self.open_flow += valve.flow_rate;
        self.valves.entry(valve.label.clone()).and_modify(|x| *x = true);
    }

    fn move_to(&mut self, valve: String) {
        self.position = valve;
    }
}

fn part1(input: &str) -> u32 {
    let (valves, distances) = parse_input(input);

    let usable_valves = valves.values().filter(|x| x.flow_rate > 0).map(|x| x.label.as_str()).collect::<Vec<_>>();
    println!("Usable valves: {:?}", usable_valves);
    // Print number of permutations
    println!("Number of permutations: {}", (1..=(usable_valves.len() as u64)).product::<u64>());

    let mut best = 0;
    for (i, perm) in enumerate(usable_valves.iter().permutations(usable_valves.len())) {
        if i % 100000 == 0 {
            println!("{}", i);
        }
        let result = evaluate_path(&perm, &valves, &distances);
        if result > best {
            best = result;
        }
    }

    best
}

fn evaluate_path(path: &Vec<&&str>, valves: &HashMap<String, Valve>, distances: &Vec<Vec<u32>>) -> u32 {
    let mut total_flow = 0;
    let mut current_flow = 0;
    let mut time = 1;
    let mut position = "AA";

    for pos in path {
        let v1 = valves.get(position).unwrap();
        let v2 = valves.get(**pos).unwrap();
        position = *pos;

        let id_v1 = valves.iter().position(|(_, v)| *v == *v1).unwrap();
        let id_v2 = valves.iter().position(|(_, v)| *v == *v2).unwrap();

        let distance = distances[id_v1][id_v2];

        let time_to_move = min(distance, 30 - time);
        total_flow += current_flow * time_to_move;
        time += time_to_move;

        //eprintln!("{}: Moving from {} to {} in {}, release {}", time, v1.label, v2.label, distance, current_flow * time_to_move);

        if time == 30 {
            break;
        }

        // Open valve if useful
        if v2.flow_rate > 0 {
            //eprintln!("{}: Opening valve {}, release {}", time, v2.label, current_flow);
            total_flow += current_flow;
            time += 1;
            current_flow += v2.flow_rate;
        }

        if time == 30 {
            break;
        }
    }

    if time <= 30 {
        total_flow += current_flow * (30 - time + 1);
        //println!("{}: Releasing {} at the end", time, current_flow * (30 - time + 1));
    }

    total_flow
}

fn part2(input: &str) -> u32 {
    0
}

fn parse_input(input: &str) -> (HashMap<String, Valve>, Vec<Vec<u32>>) {
    let re = Regex::new(r"^Valve (.+) has flow rate=(\d+);.*to valves? (.*)$").unwrap();
    let valves = input.lines()
        .map(|x| re.captures(x).map(|caps| {
            let (_, [label, flow_rate, next_valves]) = caps.extract();
            (label.to_string(), Valve {
                label: label.to_string(),
                flow_rate: flow_rate.parse().unwrap(),
                next_valves: next_valves.split(", ").map(|x| x.to_string()).collect(),
            })
        }).unwrap())
        .collect::<HashMap<_, _>>();

    let distances = valves.values().map(|x|
        valves.values().map(|y|
            if x.label == y.label {
                0
            } else if x.next_valves.contains(&y.label) {
                1
            } else {
                1000
            }
        ).collect::<Vec<_>>()
    ).collect::<Vec<_>>();

    // Use Floyd-Warshall to compute shortest paths between all pairs of valves.
    let mut distances = distances;
    for k in 0..distances.len() {
        for i in 0..distances.len() {
            for j in 0..distances.len() {
                distances[i][j] = distances[i][j].min(distances[i][k] + distances[k][j]);
            }
        }
    };

    (valves, distances)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = include_str!("test_input.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_path_eval() {
        let (valves, distances) = parse_input(TEST_INPUT_1);
        assert_eq!(evaluate_path(&vec![&"DD", &"BB", &"JJ", &"HH", &"EE", &"CC"], &valves, &distances), 1651);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT_1), 1651);
        assert_eq!(part1(INPUT), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_1), 0);
        assert_eq!(part2(INPUT), 0);
    }
}
