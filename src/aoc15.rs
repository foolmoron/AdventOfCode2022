use std::{
    collections::HashSet,
    ops::{RangeBounds, RangeInclusive},
};

use regex::Regex;

#[derive(Debug)]
struct Sensor {
    pos: (i32, i32),
    beacon: (i32, i32),
    range: i32,
}

fn parse() -> Vec<Sensor> {
    let re =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();
    return INPUT
        .lines()
        .map(|line| {
            let groups = re.captures(line).unwrap();
            let s_x = groups[1].parse::<i32>().unwrap();
            let s_y = groups[2].parse::<i32>().unwrap();
            let b_x = groups[3].parse::<i32>().unwrap();
            let b_y = groups[4].parse::<i32>().unwrap();
            return Sensor {
                pos: (s_x, s_y),
                beacon: (b_x, b_y),
                range: (b_x - s_x).abs() + (b_y - s_y).abs(),
            };
        })
        .collect::<Vec<_>>();
}

fn count_and_print<R: RangeBounds<i32> + IntoIterator<Item = i32>>(
    sensors: &Vec<Sensor>,
    range: R,
) {
    for y in range {
        let mut covered = covered_ranges(sensors, y)
            .iter()
            .flat_map(|r| r.clone())
            .collect::<HashSet<i32>>();
        let sensor_xs = sensors
            .iter()
            .filter(|s| s.pos.1 == y)
            .map(|s| s.pos.0)
            .collect::<HashSet<i32>>();
        let beacons_xs = sensors
            .iter()
            .filter(|s| s.beacon.1 == y)
            .map(|s| s.beacon.0)
            .collect::<HashSet<i32>>();
        for beacon_x in &beacons_xs {
            covered.remove(&beacon_x);
        }
        println!(
            "{: >20}: {}",
            format!("{} ({})", y, covered.len()),
            (-20..=40)
                .map(|i| if sensor_xs.contains(&i) {
                    'S'
                } else if covered.contains(&i) {
                    '#'
                } else if beacons_xs.contains(&i) {
                    'B'
                } else {
                    '.'
                })
                .collect::<String>()
        );
    }
}

fn covered_ranges(sensors: &Vec<Sensor>, y: i32) -> Vec<RangeInclusive<i32>> {
    let ranges = sensors
        .iter()
        .filter_map(|s| {
            let diff = (s.pos.1 - y).abs();
            let width = 2 * (s.range - diff) + 1;
            if width > 0 {
                return Some(s.pos.0 - width / 2..=s.pos.0 + width / 2);
            } else {
                return None;
            }
        })
        .collect::<Vec<_>>();
    return ranges;
}

fn calc1() -> i32 {
    let sensors = parse();
    // println!("{:?}", sensors);
    // count_and_print(&sensors, RANGE);
    return 0;
}

fn calc2() -> i64 {
    const MAX: i32 = 4_000_000;
    const MAX_BY_20: i32 = MAX / 20;
    let sensors = parse();
    for y in 0..=MAX {
        let ranges = covered_ranges(&sensors, y);
        if y % MAX_BY_20 == 0 {
            println!("y={}", y);
        }
        let mut x = 0;
        while x <= MAX {
            let mut found = false;
            for r in &ranges {
                if r.contains(&x) {
                    x = *r.end();
                    found = true;
                }
            }
            if !found {
                println!("Found! {} {}", x, y);
                return x as i64 * MAX as i64 + y as i64;
            }
            x += 1;
        }
    }
    return -1;
}

pub fn calc() {
    println!("{}\n{}", calc1(), calc2());
}

const RANGE: RangeInclusive<i32> = 2_000_000..=2_000_000;
const INPUT: &str = r#"Sensor at x=1326566, y=3575946: closest beacon is at x=1374835, y=2000000
Sensor at x=2681168, y=3951549: closest beacon is at x=3184941, y=3924923
Sensor at x=3959984, y=1095746: closest beacon is at x=3621412, y=2239432
Sensor at x=3150886, y=2479946: closest beacon is at x=3621412, y=2239432
Sensor at x=3983027, y=2972336: closest beacon is at x=4012908, y=3083616
Sensor at x=3371601, y=3853300: closest beacon is at x=3184941, y=3924923
Sensor at x=3174612, y=3992719: closest beacon is at x=3184941, y=3924923
Sensor at x=3316368, y=1503688: closest beacon is at x=3621412, y=2239432
Sensor at x=3818181, y=2331216: closest beacon is at x=3621412, y=2239432
Sensor at x=3960526, y=3229321: closest beacon is at x=4012908, y=3083616
Sensor at x=61030, y=3045273: closest beacon is at x=-467419, y=2369316
Sensor at x=3635583, y=3121524: closest beacon is at x=4012908, y=3083616
Sensor at x=2813357, y=5535: closest beacon is at x=3595763, y=-77322
Sensor at x=382745, y=1566522: closest beacon is at x=1374835, y=2000000
Sensor at x=3585664, y=538632: closest beacon is at x=3595763, y=-77322
Sensor at x=3979654, y=2158646: closest beacon is at x=3621412, y=2239432
Sensor at x=3996588, y=2833167: closest beacon is at x=4012908, y=3083616
Sensor at x=3249383, y=141800: closest beacon is at x=3595763, y=-77322
Sensor at x=3847114, y=225529: closest beacon is at x=3595763, y=-77322
Sensor at x=3668737, y=3720078: closest beacon is at x=3184941, y=3924923
Sensor at x=1761961, y=680560: closest beacon is at x=1374835, y=2000000
Sensor at x=2556636, y=2213691: closest beacon is at x=3621412, y=2239432
Sensor at x=65365, y=215977: closest beacon is at x=346716, y=-573228
Sensor at x=709928, y=2270200: closest beacon is at x=1374835, y=2000000
Sensor at x=3673956, y=2670437: closest beacon is at x=4029651, y=2547743
Sensor at x=3250958, y=3999227: closest beacon is at x=3184941, y=3924923
Sensor at x=3009537, y=3292368: closest beacon is at x=3184941, y=3924923"#;
