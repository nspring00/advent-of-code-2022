use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1::<2000000>(input));
    println!("Part 2: {}", part2::<4000000>(input));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Sensor {
    pos: (i32, i32),
    range: i32,
}

struct Beacon {
    pos: (i32, i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Interval {
    start: i32,
    end: i32,
}

trait IntervalVec {
    fn distinct_values(&self) -> u32;
    fn distinct_values_without(&self, excludes: &[i32]) -> u32;
}

impl IntervalVec for Vec<Interval> {
    fn distinct_values(&self) -> u32 {
        let mut values = Vec::new();
        for interval in self {
            for i in interval.start..(interval.end + 1) {
                values.push(i);
            }
        }
        values.sort();
        values.dedup();
        values.len() as u32
    }

    fn distinct_values_without(&self, excludes: &[i32]) -> u32 {
        let mut values = Vec::new();
        for interval in self {
            for i in interval.start..(interval.end + 1) {
                values.push(i);
            }
        }
        values.sort();
        values.dedup();
        values.retain(|x| !excludes.contains(x));
        values.len() as u32
    }
}

fn part1<const ROW: i32>(input: &str) -> u32 {
    let (sensors, beacons) = &parse(input);
    let mut intervals = Vec::new();
    for sensor in sensors {
        let distance = sensor.range - (sensor.pos.1 - ROW).abs();
        if distance >= 0 {
            intervals.push(Interval {
                start: sensor.pos.0 - distance,
                end: sensor.pos.0 + distance,
            });
        }
    }
    println!("{:?}", intervals);
    intervals.distinct_values_without(&beacons.iter().filter_map(|x| if x.pos.1 == ROW { Some(x.pos.0) } else { None }).collect::<Vec<_>>())
}

fn manhattan_dist(a: &(i32, i32), b: &(i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn intersect(a: &Sensor, b: &Sensor) -> Vec<(i32, i32)> {
    [intersection_points(a, b), intersection_points(b, a)].concat()
}

fn intersection_points(a: &Sensor, b: &Sensor) -> [(i32, i32); 4] {
    let d1 = a.range + 1;
    let d2 = b.range + 1;
    let dxs = [
        (a.pos.0 - d1, b.pos.0 - d2),
        (a.pos.0 - d1, b.pos.0 + d2),
        (a.pos.0 + d1, b.pos.0 - d2),
        (a.pos.0 + d1, b.pos.0 + d2),
    ];

    dxs.map(|(x1, x2)| ((x2 + b.pos.1 + x1 - a.pos.1) / 2, (x2 + b.pos.1 - x1 + a.pos.1) / 2))
}

fn part2<const BOUND: i32>(input: &str) -> u64 {
    // Borrowed some math from https://jactl.io/blog/2023/04/20/advent-of-code-2022-day15.html
    let (sensors, _beacons) = &parse(input);
    let results = sensors.iter()
        .flat_map(|s|
        sensors.iter()
            .flat_map(|s2|
                intersect(s, s2)
            )
            .filter(|(x, y)| 0 <= *x && *x <= BOUND && 0 <= *y && *y <= BOUND)
            .filter(|p1| sensors.iter().all(|b| manhattan_dist(p1, &b.pos) > b.range))
        )
        .collect::<Vec<_>>();
    // results.sort_unstable();
    // results.dedup();

    results[0].0 as u64 * 4000000 + results[0].1 as u64
}

fn parse(input: &str) -> (Vec<Sensor>, Vec<Beacon>) {
    let re = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
    input.lines()
        .map(|x| {
            let caps = re.captures(x).unwrap();
            let pos = (caps[1].parse::<i32>().unwrap(), caps[2].parse::<i32>().unwrap());
            let beacon_pos = (caps[3].parse::<i32>().unwrap(), caps[4].parse::<i32>().unwrap());
            let range = (pos.0 - beacon_pos.0).abs() + (pos.1 - beacon_pos.1).abs();
            (Sensor {
                pos,
                range
            },
                Beacon {
                pos: beacon_pos,
            })
        })
        .unzip()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = include_str!("test_input.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1::<10>(TEST_INPUT_1), 26);
        assert_eq!(part1::<2000000>(INPUT), 5688618);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2::<20>(TEST_INPUT_1), 56000011);
        assert_eq!(part2::<4000000>(INPUT), 12625383204261);
    }
}
