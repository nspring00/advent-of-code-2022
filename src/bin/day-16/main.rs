use std::cmp::min;
use std::collections::HashMap;
use std::fmt::Debug;
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

struct ValveSystem2<'a> {
    valves: Vec<String>,
    tunnels: HashMap<&'a str, Vec<&'a str>>,
    flow_rates: HashMap<&'a str, u32>,
    mask_values: HashMap<&'a str, u32>,
    distances: HashMap<&'a str, HashMap<&'a str, u32>>,
}

fn parse_input2(input: &str) -> ValveSystem2 {
    let re = Regex::new(r"^Valve (.+) has flow rate=(\d+);.*to valves? (.*)$").unwrap();
    let valves_raw = input.lines()
        .map(|x| re.captures(x).map(|caps| {
            let (_, [label, flow_rate, next_valves_str]) = caps.extract();
            let next_valves = next_valves_str.split(", ").collect::<Vec<_>>();
            (label, flow_rate.parse::<u32>().unwrap(), next_valves)
        }).unwrap())
        .collect::<Vec<_>>();

    let valves = valves_raw.iter().map(|(label, _, _)| label.to_string()).collect::<Vec<_>>();

    let tunnels = valves_raw.iter().map(|(label, _, next_valves)| (*label, next_valves.clone())).collect::<HashMap<_, _>>();

    let flow_rates = valves_raw.iter().filter(|(_, flow_rate, _)| *flow_rate != 0).map(|(label, flow_rate, _)| (*label, *flow_rate)).collect::<HashMap<_, _>>();

    let mask_values = flow_rates.iter().enumerate().map(|(i, x)| (*(x.0), 1 << i)).collect::<HashMap<_, _>>();

    // Compute a distance matrix between all valves using Floyd-Warshall
    let mut distances = tunnels.keys().map(|x| (*x, tunnels.keys().map(|y| (*y, 1000u32)).collect::<HashMap<_, _>>())).collect::<HashMap<_, _>>();
    for (x, ys) in tunnels.iter() {
        distances.get_mut(x).unwrap().insert(*x, 0);
        for y in ys {
            distances.get_mut(x).unwrap().insert(*y, 1);
        }
    }
    for k in tunnels.keys() {
        for i in tunnels.keys() {
            for j in tunnels.keys() {
                let new_dist = distances[i][j].min(distances[k][j] + distances[i][k]);
                distances.get_mut(i).unwrap().insert(*j, new_dist);
            }
        }
    }

    ValveSystem2 {
        valves,
        tunnels,
        flow_rates,
        mask_values,
        distances,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = include_str!("test_input.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_parse_input2() {
        let input = "Valve AA has flow rate=0; tunnels lead to valves BB\n\
                            Valve BB has flow rate=13; tunnels lead to valves AA, CC\n\
                            Valve CC has flow rate=2; tunnels lead to valves BB";
        let valve_system = parse_input2(input);
        assert_eq!(valve_system.valves, vec!["AA", "BB", "CC"]);
        assert_eq!(valve_system.tunnels, vec![("AA", vec!["BB"]), ("BB", vec!["AA", "CC"]), ("CC", vec!["BB"])].into_iter().collect::<HashMap<_, _>>());
        assert_eq!(valve_system.flow_rates, vec![("BB", 13), ("CC", 2)].into_iter().collect::<HashMap<_, _>>());
        assert_eq!(valve_system.distances, vec![("AA", vec![("AA", 0), ("BB", 1), ("CC", 2)].into_iter().collect::<HashMap<_, _>>()), ("BB", vec![("AA", 1), ("BB", 0), ("CC", 1)].into_iter().collect::<HashMap<_, _>>()), ("CC", vec![("AA", 2), ("BB", 1), ("CC", 0)].into_iter().collect::<HashMap<_, _>>())].into_iter().collect::<HashMap<_, _>>());

        // Assert that every key in the bitmask has a unique value and is a power of 2
        assert_eq!(valve_system.mask_values.values().unique().count(), valve_system.mask_values.len());
        assert!(valve_system.mask_values.values().all(|x| x.is_power_of_two()));
    }

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
