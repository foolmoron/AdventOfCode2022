use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

struct Node {
    point: (i32, i32),
    steps: i32,
    dist: f32,
}

fn heuristic(
    grid: &Vec<Vec<i32>>,
    steps: i32,
    (prev_x, prev_y): (i32, i32),
    (next_x, next_y): (i32, i32),
    (_e_x, _e_y): (i32, i32),
) -> f32 {
    // way too complicated because I had a bug and I thought a better heuristic would solve it
    let dist = (((next_x - _e_x).pow(2) + (next_y - _e_y).pow(2)) as f32).sqrt();
    let diff = grid[next_y as usize][next_x as usize] - grid[prev_y as usize][prev_x as usize];
    let climbable = diff == 1;
    let down = diff < 0;
    return steps as f32 + dist
        - if climbable {
            500.0
        } else if down {
            -100.0
        } else {
            0.0
        };
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (other.dist).partial_cmp(&(self.dist))
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.dist).total_cmp(&(self.dist))
    }
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}
impl Eq for Node {}

fn parse(input: &str) -> Vec<Vec<i32>> {
    return input
        .split("\n")
        .map(|line| line.chars().map(|c| c as i32).collect::<Vec<_>>())
        .collect::<Vec<_>>();
}

fn debug(num: i32, s: &(i32, i32), steps: i32, grid: &Vec<Vec<i32>>, (x, y): (i32, i32)) {
    println!("Iteration {} starting at {:?}, {} steps:", num, s, steps);
    for (j, row) in grid.iter().enumerate() {
        for (i, &cell) in row.iter().enumerate() {
            if i == x as usize && j == y as usize {
                print!("█");
            } else if i == s.0 as usize && j == s.1 as usize {
                print!("▄");
            } else {
                print!("{}", char::from_u32(cell as u32).unwrap());
            }
        }
        println!();
    }
    println!();
}

fn calc1() -> i32 {
    let mut grid = parse(INPUT);

    let mut s = (0, 0);
    let mut e = (0, 0);
    for (y, row) in grid.clone().iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == 'S' as i32 {
                s = (x as i32, y as i32);
                grid[y][x] = 'a' as i32;
            } else if cell == 'E' as i32 {
                e = (x as i32, y as i32);
                grid[y][x] = 'z' as i32;
            }
        }
    }

    let mut heap = BinaryHeap::<Node>::new();
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    heap.push(Node {
        point: s,
        steps: 0,
        dist: 0.0,
    });

    let mut num = 0;
    while let Some(curr) = heap.pop() {
        if visited[curr.point.1 as usize][curr.point.0 as usize] {
            continue;
        }
        num += 1;
        visited[curr.point.1 as usize][curr.point.0 as usize] = true;

        if curr.point == e {
            debug(num, &s, curr.steps, &grid, curr.point);
            return curr.steps;
        }

        let curr_val = grid[curr.point.1 as usize][curr.point.0 as usize];
        for i in -1..=1 {
            for j in -1..=1 {
                if (i == 0) == (j == 0) {
                    // use only cardinal directions
                    continue;
                }
                let _x = curr.point.0 + i;
                let _y = curr.point.1 + j;
                if _x < 0 || _y < 0 || _x >= grid[0].len() as i32 || _y >= grid.len() as i32 {
                    continue;
                }
                let x = _x as usize;
                let y = _y as usize;
                if visited[y][x] {
                    continue;
                }

                let next_val = grid[y][x];
                // println!("{} vs {}: {}", curr_val, next_val, curr_val >= (next_val - 1));
                if curr_val >= (next_val - 1) {
                    heap.push(Node {
                        point: (_x, _y),
                        steps: curr.steps + 1,
                        dist: heuristic(&grid, curr.steps + 1, curr.point, (_x, _y), e),
                    });
                }
            }
        }
    }
    return -1;
}

fn calc2() -> i32 {
    let mut grid = parse(INPUT);

    let mut s_options = Vec::<(i32, i32)>::new();
    let mut e = (0, 0);
    for (y, row) in grid.clone().iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == 'S' as i32 || cell == 'a' as i32 {
                s_options.push((x as i32, y as i32));
                grid[y][x] = 'a' as i32;
            } else if cell == 'E' as i32 {
                e = (x as i32, y as i32);
                grid[y][x] = 'z' as i32;
            }
        }
    }

    let mut best = i32::MAX;
    for s in s_options {
        let mut heap = BinaryHeap::<Node>::new();
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        heap.push(Node {
            point: s,
            steps: 0,
            dist: 0.0,
        });

        let mut num = 0;
        while let Some(curr) = heap.pop() {
            if visited[curr.point.1 as usize][curr.point.0 as usize] {
                continue;
            }
            num += 1;
            visited[curr.point.1 as usize][curr.point.0 as usize] = true;

            if curr.point == e {
                debug(num, &s, curr.steps, &grid, curr.point);
                best = std::cmp::min(best, curr.steps);
            }

            let curr_val = grid[curr.point.1 as usize][curr.point.0 as usize];
            for i in -1..=1 {
                for j in -1..=1 {
                    if (i == 0) == (j == 0) {
                        // use only cardinal directions
                        continue;
                    }
                    let _x = curr.point.0 + i;
                    let _y = curr.point.1 + j;
                    if _x < 0 || _y < 0 || _x >= grid[0].len() as i32 || _y >= grid.len() as i32 {
                        continue;
                    }
                    let x = _x as usize;
                    let y = _y as usize;
                    if visited[y][x] {
                        continue;
                    }

                    let next_val = grid[y][x];
                    // println!("{} vs {}: {}", curr_val, next_val, curr_val >= (next_val - 1));
                    if curr_val >= (next_val - 1) {
                        heap.push(Node {
                            point: (_x, _y),
                            steps: curr.steps + 1,
                            dist: heuristic(&grid, curr.steps + 1, curr.point, (_x, _y), e),
                        });
                    }
                }
            }
        }
    }

    return best;
}

pub fn calc() {
    println!("{}\n{}", calc1(), calc2());
}

const INPUT: &str = r#"abccccccaaccaaccccaaaaacccccaaaaccccccccccccccccccccccccccccccccaaaaaaaaaaaaaaaaaaaccccccccccccccccaaaccccccccccccaacccccccccccccccccccccccccccccccccccccccccaaaa
abaaaaccaaaaaccccaaaaaccccccaaaaccccccccccccccccccccaaacccccccccccaaaaaaaaaaaaaaaaaaccccccccccccccccaaaaccccccaaacaaccccccccccccccccccccccccccccccccccccccccaaaaa
abaaacccaaaaaaaacaaaaaacccccaaaaccccccccccccccccccccaaaaacccccccccaaaaaaaaaaaaaaaaacccccccccaaccccaaaaaacccccccaaaaaccccaaccccccccccccccacccccccccccccccccccaaaaa
abaaacccccaaaaaccccaaaaccccccaaacccccccccccccccccccaaaaaccccccccccaaaaaacacaaaaaacccccccccccaaccccaaaaacccccccccaaaaaaccaaaaaaccccccccccaaaccccacccccccccccaaaaaa
abaacccccaaaaaccccaacccccccccccccccaaaaacccccccccccaaaaacccccccccaaaaaaaaccaaaaaaacccccccaaaaaaaaccaaaaacccccccaaaaaaaccaaaaacccccccccccaaacccaaaccccccccccccccaa
abaaacccaaacaacccccccccccccccccccccaaaaaccccccccccccaaaaacccccccaaaaaaaaaccaaccaaacccccccaaaaaaaaccaacccccccccaaaaaaccaaaaaaccccccccccccaaaacaaaaccccccccccccccaa
abaaacccccccaaccccccccccccccccccccaaaaaaccccccccccccaaccccccaacccaaaccaaaaccccccaacccccccccaaaacccccccccccccccaacaaaccaaaaaaaccccccccccccajjjjjjjcccccccccccccccc
abcaacccccccccccccccccccccccccccccaaaaaaccccccccccccccccccccaaaaccccccaaaaccccccccccccccaacaaaaaccccccccccccccccccaaccccaaaaaacccccccccccjjjjjjjjjcccccaaaccccccc
abccccccccccccccccccccccccccccccccaaaaaaccaaccccccccccccccaaaaaacccccccaaacccccccccccaacaaaaaaaaccccccccccccccccccccccccaaccaaccccccccaiijjjjojjjjcccccaaacaccccc
abcccccccccccccccccccccccaaacccccccaaacacaaacccccccccccccccaaaaccccaaccccccccccccccccaaaaaaacccaccccccccccccccccccccccccaacccccccccccaiiijjooooojjkccaaaaaaaacccc
abccccccccccccccccccccccaaaaccccccccccaaaaaccccccccccccccccaaaaacccaaaaaccccccccccccccaaaaaacccccccccccccccccccccccccccccccccccccciiiiiiiioooooookkkcaaaaaaaacccc
abccccccccccccccccccccccaaaaccccccccccaaaaaaaacccccccccccccaacaaccaaaaacccccccaaacccaaaaaaaaccccccccccccccccccccccccccccccccccchiiiiiiiiooooouoookkkccaaaaaaccccc
abcccccccccaaccccccccccccaaaccccccccccccaaaaacccccccccccccccccccccaaaaaccccccaaaacccaaaaacaacccccccccccccaacaacccccccccccccccchhhiiiinnnooouuuuoookkkccaaaaaccccc
abcccccccccaaacccccccccccccccccccccccccaaaaacccccccccccccccccccccccaaaaacccccaaaaccccccaaccccccccccccccccaaaaacccccccccccccccchhhnnnnnnnnouuuuuuppkkkkaaaaaaccccc
abccccccaaaaaaaacccaaccccccccccccccccccaacaaccaacaaccccccccccccccccaacccccccccaaaccccccaacccccccccccccccaaaaacccccccccccccccchhhnnnnnnnnntuuxuuupppkkkkkacccccccc
abccccccaaaaaaaacacaaaacccccccccccccccccccaaccaaaaacccccccccccccccccccccccccccccccccccccccccccccccccccccaaaaaacccccccaacccccchhhnnnnttttttuxxxuuppppkkkkkcccccccc
abcccccccaaaaaaccaaaaaaccccccccccaaccccccccccaaaaaccccccccccccccccccccccccaaacccccccccccccccccccccccccccccaaaaccaaccaaacccaaahhhnnntttttttuxxxxuupppppllllccccccc
abcccccccaaaaaacccaaaacccccccccaaaaaaccccccccaaaaaacccccccccccccccccccccccaaacccccccccccccccccccccccccccccacccccaaaaaaacaaaaahhhppntttxxxxxxxxuuuuvpppplllccccccc
abcccccccaaaaaacccaaaacccccccccaaaaaacccccaaaaaaaaaccccccccccccccccccccaaaaaaaacccccccccccccccccccccaaaccccccaacaaaaaaccaaaaahhhpppttxxxxxxxxyuuvvvvvppplllcccccc
abcccccccaaccaacccaacaccaaaaccccaaaaacccccaaaaaaaaaccccccccccccccccccccaaaaaaaacccccccccccccccccccccaaacaaaaaaaccaaaaaaaaaaaaahhppptttxxxxxxyyyyyyvvvppplllcccccc
SbccccccccccccccccccccccaaaacccaaaaacccccccaaaaaaaaacaaaccccccccaacccccccaaaaaccccccccaaaaacccccccccaaaaaaaaaaaaaaaaaaaaaaaaacgggpppttxxxxEzzyyyyyvvvqqqlllcccccc
abccccccccccccccccccccccaaaacccaaaaacccccccaaaaaaaaccaaaccccccccaaacaaccaaaaaaccccccccaaaaacccccccaaaaaaaaaaaaaaaaaaaaaaaaaaacgggpppsssxxxyyyyyyvvvvvqqlllccccccc
abcccaaaccccccccccccccccaaaccccccccccccccccaaaaaaaaaaaaaccccccccaaaaaaccaaaaaacccccccaaaaaacccaaaccaaaaaccaaaaaaaaaaaacccccccccgggppssswwyyyyyyvvvvqqqqlllccccccc
abcaaaaaccccccccccccccccccccccccccccccccccaaaaaaaaaaaaacccccccaaaaaaacccaccaaacccccccaaaaaacccaaacccaaaaaaaaaaaccccaaacccaaaaacgggppsswwwyyyyyyvvqqqqqlllcccccccc
abcaaaaaaccccccccccccccccccccccccccccccccccaaccaaaaaaaaaaaccccaaaaaaacccccccccccccccccaaaaacccaaacaaaacaaaaaaaaccccaaacccaaaaacggpppsswwwywwyyyvvqqqmmmlccccccccc
abcaaaaaacccccccaacaaccccccccccccccccccccccccccaaaaaaaaaaaccccccaaaaacccccccccccccccccaaaccaaaaaaaaaaacccccccaacccccccccaaaaaacggpppsswwwwwwwwyvvqqqmmmcccccccccc
abcaaaaaccccccccaaaaaccccccccccccccccccccccccccccaaaaaaaacccccccaacaaacccccccccccccccccccccaaaaaaaaaccccccccccccccccccccaaaaaagggoossswwwwrrwwwvvqqmmmccccccccccc
abcaaaaacccccccaaaaaccccccccccccccccccccccccccccaaaaaaacccccccccaaccccccccccccccccccccccccccaaaaaaacccccccccccaaaccccccccaaaaagggooosssssrrrrwwwvqqmmmcccaacccccc
abcccccccccccccaaaaaaccccccccccccccccccccaacccccccccaaaccccccccccccccccccccccccccccccccccccccaaaaaaccccccccccccaaaaccccccaaaccgggooosssssrrrrrwwrrqmmmcccaacccccc
abcccccccccccccccaaaacccccccccccccccccccaaaacccccccacaaacccccccccccccccccccccccccccccccccccccaaaaaaacccccccccaaaaaacccccccccccgffoooooosoonrrrrrrrrmmmccaaaaacccc
abcccccccccccccccaccccccccccccccccccccccaaaacccccccaaaaacccccccccccccccccccccccccccccccccccccaaacaaacccccccccaaaaacccccccccccccfffoooooooonnnrrrrrmmmddcaaaaacccc
abccccccccccccccccccccccccccccccccccccccaaaaccccccccaaaaacccccccccccccccccccccccccaaaccccccccaacccccccccccccccaaaaaccccccccccccffffoooooonnnnnnrnnmmmdddaaaaacccc
abcccccccccccccccccccccccccccccccccccccccccccccccccaaaaaacccccccccccccccccaaaaaccaaaacccccccccccccccccccccccccaacccccccccccccccfffffffffeeeennnnnnmmdddaaaacccccc
abcccccccaaaccccccccaccccccccccccccccccccccccccccccaaaaccccccccccccaaaccccaaaaaccaaaaccccccccccccccccccccccccccccccccccccccccccccfffffffeeeeennnnnmddddaaaaaccccc
abcccaaccaaacccccaaaacccccaacccccccccccccccccccccccccaaacccccccccccaaacccaaaaaacccaaaccccccccccccccccccccccccccccccccccccccccccccccffffeeeeeeeedddddddcccaacccccc
abcccaaaaaaacccccaaaaaaccaaacccccccccccccccccccccccccccacccccccccccaaaaccaaaaaaccccccccccccccccccccccccccaacccccccccaaaccccccccccccccaaaaaaeeeeedddddcccccccccccc
abcccaaaaaacccccccaaaacccaaacaaaccccaaaacccccccccaaacaaaccccccaacccaaaacaaaaaaacccccccccccccccccccccccccaaaccccccccaaaacccccccccccccccccccaaaaeeddddccccccccccccc
abccccaaaaaaaacccaaaaaaaaaaaaaaaccccaaaacccccccccaaaaaaacccccaaacccaaaaaaaaaacccccccccccccccccccccccaaacaaaccccccccaaaaccccccccccccccccccccaaaccccccccccccccaaaca
abcccaaaaaaaaacccaacaaaaaaaaaaacccccaaaaccccccccccaaaaaacaaacaaacaaaaaaaaaacccccccccccccccaaacccccccaaaaaaaaaaccccccaaaccccccccccccccccccccaacccccccccccccccaaaaa
abcccaaaaaaaacccccccccccaaaaaacccccccaacccccccccccaaaaaaaaaaaaaaaaaaaaaaaaccccccccccccccccaaaacccccccaaaaaaaaacccccccccccccccccccccccccccccaaacccccccccccccccaaaa
abccaaaaaaacccccccccccccaaaaaaacccccccccccccccccaaaaaaaaaaaaaaaaaaaaaaaaaaacccccccccccccccaaaacccccccaaaaaaaacccccccccccccccccccccccccccccccccccccccccccccccaaaaa"#;
