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
    prev: String,
    opens: Vec<String>,
    time: i32,
    pressure: i32,
    potential: i32,
}

impl Node {
    fn heuristic(&self) -> i32 {
        self.pressure + self.potential
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (self.heuristic()).partial_cmp(&(other.heuristic()))
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.heuristic()).cmp(&(other.heuristic()))
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

fn potential(all: &HashMap<String, Valve>, opens: &Vec<String>, time: i32) -> i32 {
    let mut potential = 0;
    for (name, valve) in all {
        if valve.rate > 0 && !opens.contains(name) {
            potential += valve.rate * time / 2;
        }
    }
    return potential;
}

fn calc1() -> i32 {
    let valves = parse(INPUT);
    let mut heap = BinaryHeap::new();
    heap.push(Node {
        valve: "AA".to_string(),
        prev: "AA".to_string(),
        opens: vec![],
        time: 30,
        pressure: 0,
        potential: potential(&valves, &Vec::new(), 30),
    });

    let mut steps = 0;
    let mut best = 0;
    while let Some(curr) = heap.pop() {
        steps += 1;
        if steps % 100000 == 0 {
            println!("{}: {} ({}) - {:?}", steps, curr.valve, curr.heuristic(), curr.opens);
        }
        if steps > 500_000 {
            break;
        }
        let curr_name = curr.valve;
        let curr_valve = valves.get(&curr_name).unwrap();
        let new_time = curr.time - 1;
        // finish when time is over
        if new_time < 0 || curr.opens.len() == valves.len() {
            if curr.pressure > best {
                best = curr.pressure;
                println!("FOUND BETTER: {} - {:?}", best, curr.opens);
            }
            continue;
        }
        // open if not opened
        if !curr.opens.contains(&curr_name) {
            let next_valve = valves.get(&curr_name).unwrap();
            if next_valve.rate > 0 {
                let mut new_opens = curr.opens.clone();
                new_opens.push(curr_name.clone());
                heap.push(Node {
                    valve: curr_name.clone(),
                    prev: curr_name.clone(),
                    opens: new_opens.clone(),
                    time: new_time,
                    pressure: curr.pressure + next_valve.rate * new_time,
                    potential: potential(&valves, &new_opens, new_time),
                });
            }
        }
        // don't open, move
        for name in &curr_valve.adjacent {
            if curr.prev == *name {
                continue;
            }
            heap.push(Node {
                valve: name.clone(),
                prev: curr_name.clone(),
                opens: curr.opens.clone(),
                time: new_time,
                pressure: curr.pressure,
                potential: potential(&valves, &curr.opens, new_time),
            });
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
