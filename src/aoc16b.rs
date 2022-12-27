// FAIL: Instead need to create all permutations of valve orders and calculate which would result in the highest pressure

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use regex::Regex;

#[derive(Debug, Clone)]
struct Valve {
    name: String,
    rate: i32,
    adjacent: Vec<String>,
}

impl PartialEq for Valve {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[derive(Debug, Clone)]
struct Node {
    valve: String,
    // prev: String,
    valve_el: String,
    // prev_el: String,
    opens: Vec<String>,
    time: i32,
    pressure: i32,
    potential: f32,
}

impl Node {
    fn heuristic(&self) -> f32 {
        self.pressure as f32 + self.potential
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (self.heuristic()).partial_cmp(&(other.heuristic()))
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.heuristic()).total_cmp(&(other.heuristic()))
    }
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.valve == other.valve
    }
}
impl Eq for Node {}

fn parse(input: &str) -> HashMap<String, Valve> {
    let re =
        Regex::new(r"Valve (.+) has flow rate=(\d+); tunnels? leads? to valves? (.+)").unwrap();
    return input
        .split("\n")
        .map(|line| {
            let groups = re.captures(line).unwrap();
            let name = groups[1].to_string();
            let rate = groups[2].parse::<i32>().unwrap();
            let adjacent = groups[3]
                .split(", ")
                .map(|s| s.to_string())
                .collect::<Vec<_>>();
            return (
                name.clone(),
                Valve {
                    name,
                    rate,
                    adjacent,
                },
            );
        })
        .collect::<HashMap<String, Valve>>();
}

fn potential(all: &HashMap<String, Valve>, opens: &Vec<String>, time: i32) -> f32 {
    let mut potential = 0.0;
    for (name, valve) in all {
        if valve.rate > 0 && !opens.contains(name) {
            potential += valve.rate as f32 * time as f32 / 2.1;
        }
    }
    return potential;
}

fn calc1() -> i32 {
    let valves = parse(INPUT);
    let mut heap = BinaryHeap::new();
    heap.push(Node {
        valve: "AA".to_string(),
        valve_el: "AA".to_string(),
        opens: vec![],
        time: 26,
        pressure: 0,
        potential: potential(&valves, &Vec::new(), 26),
    });

    let mut steps = 0;
    let mut best = 0;
    while let Some(curr) = heap.pop() {
        steps += 1;
        if steps % 100000 == 0 {
            println!("{}: {} ({}) - {:?}", steps, curr.valve, curr.heuristic(), curr.opens);
        }
        if steps > 5_000_000 {
            break;
        }
        let curr_name = curr.valve;
        let curr_name_el = curr.valve_el;
        let curr_valve = valves.get(&curr_name).unwrap();
        let curr_valve_el = valves.get(&curr_name_el).unwrap();
        let new_time = curr.time - 1;
        // finish when time is over
        if new_time < 0 || curr.opens.len() == valves.len() {
            if curr.pressure > best {
                best = curr.pressure;
                println!("FOUND BETTER: {} - {:?}", best, curr.opens);
            }
            continue;
        }
        // both open
        if curr_name != curr_name_el && !curr.opens.contains(&curr_name) && !curr.opens.contains(&curr_name_el) {
            if curr_valve.rate > 0 && curr_valve_el.rate > 0 {
                let mut new_opens = curr.opens.clone();
                new_opens.push(curr_name.clone());
                new_opens.push(curr_name_el.clone());
                heap.push(Node {
                    valve: curr_name.clone(),
                    valve_el: curr_name_el.clone(),
                    opens: new_opens.clone(),
                    time: new_time,
                    pressure: curr.pressure + curr_valve.rate * new_time + curr_valve_el.rate * new_time,
                    potential: potential(&valves, &new_opens, new_time),
                });
            }
        }
        // I open, el moves
        if !curr.opens.contains(&curr_name) {
            if curr_valve.rate > 0 {
                let mut new_opens = curr.opens.clone();
                new_opens.push(curr_name.clone());
                for name_el in &curr_valve_el.adjacent {
                    heap.push(Node {
                        valve: curr_name.clone(),
                        valve_el: name_el.clone(),
                        opens: new_opens.clone(),
                        time: new_time,
                        pressure: curr.pressure + curr_valve.rate * new_time,
                        potential: potential(&valves, &new_opens, new_time),
                    });
                }
            }
        }
        // el opens, I move
        if !curr.opens.contains(&curr_name_el) {
            if curr_valve_el.rate > 0 {
                let mut new_opens = curr.opens.clone();
                new_opens.push(curr_name_el.clone());
                for name in &curr_valve.adjacent {
                    heap.push(Node {
                        valve: name.clone(),
                        valve_el: curr_name_el.clone(),
                        opens: new_opens.clone(),
                        time: new_time,
                        pressure: curr.pressure + curr_valve_el.rate * new_time,
                        potential: potential(&valves, &new_opens, new_time),
                    });
                }
            }
        }
        // both move
        for name in &curr_valve.adjacent {
            for name_el in &curr_valve.adjacent {
                heap.push(Node {
                    valve: name.clone(),
                    valve_el: name_el.clone(),
                    opens: curr.opens.clone(),
                    time: new_time,
                    pressure: curr.pressure,
                    potential: potential(&valves, &curr.opens, new_time),
                });
            }
        }
    }

    return best;
}

fn calc2() -> i32 {
    return 0;
}

pub fn calc() {
    println!("{}\n{}", calc1(), calc2());
}

const INPUT: &str = r#"Valve DJ has flow rate=0; tunnels lead to valves ZH, AA
Valve LP has flow rate=0; tunnels lead to valves AA, EE
Valve GT has flow rate=0; tunnels lead to valves FJ, AW
Valve RO has flow rate=5; tunnels lead to valves NO, FD, QV, BV
Valve PS has flow rate=0; tunnels lead to valves FY, UV
Valve QV has flow rate=0; tunnels lead to valves EB, RO
Valve MV has flow rate=0; tunnels lead to valves FL, EB
Valve RN has flow rate=0; tunnels lead to valves AW, LQ
Valve HF has flow rate=0; tunnels lead to valves QN, HW
Valve PY has flow rate=19; tunnel leads to valve SN
Valve AT has flow rate=0; tunnels lead to valves YQ, UY
Valve UY has flow rate=3; tunnels lead to valves KV, ID, AT, PB, PG
Valve YI has flow rate=0; tunnels lead to valves FL, FD
Valve EB has flow rate=8; tunnels lead to valves MV, GQ, QV
Valve ID has flow rate=0; tunnels lead to valves NO, UY
Valve FY has flow rate=15; tunnels lead to valves LQ, PS
Valve GQ has flow rate=0; tunnels lead to valves EB, KM
Valve HW has flow rate=0; tunnels lead to valves FJ, HF
Valve CQ has flow rate=17; tunnels lead to valves KM, GO
Valve AW has flow rate=20; tunnels lead to valves RN, GT, WH, MX
Valve BV has flow rate=0; tunnels lead to valves RO, ZH
Valve PB has flow rate=0; tunnels lead to valves UY, AA
Valve MX has flow rate=0; tunnels lead to valves AW, YG
Valve DE has flow rate=4; tunnels lead to valves MM, PZ, PG, DS, EP
Valve AA has flow rate=0; tunnels lead to valves EP, PB, LP, JT, DJ
Valve QN has flow rate=23; tunnels lead to valves SN, HF
Valve GO has flow rate=0; tunnels lead to valves CQ, MK
Valve PZ has flow rate=0; tunnels lead to valves IJ, DE
Valve PG has flow rate=0; tunnels lead to valves UY, DE
Valve FL has flow rate=18; tunnels lead to valves MV, YI
Valve DS has flow rate=0; tunnels lead to valves DE, ZH
Valve ZH has flow rate=11; tunnels lead to valves YQ, BV, DJ, DS, SB
Valve KV has flow rate=0; tunnels lead to valves UY, IJ
Valve UV has flow rate=9; tunnels lead to valves MM, PS, YG
Valve WH has flow rate=0; tunnels lead to valves JT, AW
Valve FD has flow rate=0; tunnels lead to valves YI, RO
Valve FJ has flow rate=24; tunnels lead to valves HW, GT
Valve JT has flow rate=0; tunnels lead to valves AA, WH
Valve SN has flow rate=0; tunnels lead to valves PY, QN
Valve KM has flow rate=0; tunnels lead to valves GQ, CQ
Valve LQ has flow rate=0; tunnels lead to valves RN, FY
Valve NO has flow rate=0; tunnels lead to valves ID, RO
Valve SB has flow rate=0; tunnels lead to valves ZH, IJ
Valve MK has flow rate=25; tunnel leads to valve GO
Valve YG has flow rate=0; tunnels lead to valves MX, UV
Valve IJ has flow rate=16; tunnels lead to valves EE, KV, PZ, SB
Valve EP has flow rate=0; tunnels lead to valves AA, DE
Valve MM has flow rate=0; tunnels lead to valves UV, DE
Valve YQ has flow rate=0; tunnels lead to valves AT, ZH
Valve EE has flow rate=0; tunnels lead to valves LP, IJ"#;
