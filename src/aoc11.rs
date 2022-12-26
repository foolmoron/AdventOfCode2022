#[derive(Debug, Clone)]
enum Val {
    Num(i64),
    Old,
}

#[derive(Debug, Clone)]
enum Op {
    Add(Val),
    Mul(Val),
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i64>,
    op: Op,
    divisible: i64,
    true_next: usize,
    false_next: usize,
}

fn calc1() -> i64 {
    let mut monkeys = INPUT
        .split("\n\n")
        .map(|block| {
            let mut split = block.split("\n");
            split.next();
            let items = split
                .next()
                .unwrap()
                .split(": ")
                .nth(1)
                .unwrap()
                .split(", ")
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            let op = split
                .next()
                .unwrap()
                .split("old ")
                .nth(1)
                .map(|s| {
                    let val = s
                        .split(" ")
                        .nth(1)
                        .unwrap()
                        .parse::<i64>()
                        .map(|n| Val::Num(n))
                        .unwrap_or(Val::Old);
                    if s.split(" ").nth(0).unwrap() == "*" {
                        Op::Mul(val)
                    } else {
                        Op::Add(val)
                    }
                })
                .unwrap();
            let divisible = split
                .next()
                .unwrap()
                .split("by ")
                .nth(1)
                .unwrap()
                .parse::<i64>()
                .unwrap();
            let true_next = split
                .next()
                .unwrap()
                .split("monkey ")
                .nth(1)
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let false_next = split
                .next()
                .unwrap()
                .split("monkey ")
                .nth(1)
                .unwrap()
                .parse::<usize>()
                .unwrap();
            return Monkey {
                items,
                op,
                divisible,
                true_next,
                false_next,
            };
        })
        .collect::<Vec<_>>();
    let count = monkeys.len();

    let mut touches = vec![0; count];

    for _r in 0..20 {
        let monkeys_ref = &mut monkeys;
        let mut moves = vec![Vec::<i64>::new(); count];
        for m in 0..count {
            let monkey = &mut monkeys_ref[m];

            // Calc moves
            for &item in &monkey.items {
                touches[m] += 1;
                let mut worry = item;
                match monkey.op {
                    Op::Add(Val::Num(n)) => worry += n,
                    Op::Add(Val::Old) => worry += item,
                    Op::Mul(Val::Num(n)) => worry *= n,
                    Op::Mul(Val::Old) => worry *= item,
                }
                worry = worry / 3;
                let next = if worry % monkey.divisible == 0 {
                    monkey.true_next
                } else {
                    monkey.false_next
                };
                moves[next].push(worry);
                // println!("Monkey {} threw item {} to monkey {}", m, worry, next);
            }

            // Clear current items
            monkey.items.clear();

            // Move items and clear
            for (m2, monkey) in monkeys_ref.iter_mut().enumerate() {
                monkey.items.append(&mut moves[m2]);
                moves[m2].clear();
            }
        }
        // println!("Round {}", r + 1);
        // for monkey in monkeys_ref.iter() {
        //     println!("{:?}", monkey.items);
        // }
        // println!()
    }

    touches.sort();
    touches.reverse();
    println!("{:#?}", touches);

    return touches[0] * touches[1];
}

fn calc2() -> i64 {
    let mut monkeys = INPUT
        .split("\n\n")
        .map(|block| {
            let mut split = block.split("\n");
            split.next();
            let items = split
                .next()
                .unwrap()
                .split(": ")
                .nth(1)
                .unwrap()
                .split(", ")
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            let op = split
                .next()
                .unwrap()
                .split("old ")
                .nth(1)
                .map(|s| {
                    let val = s
                        .split(" ")
                        .nth(1)
                        .unwrap()
                        .parse::<i64>()
                        .map(|n| Val::Num(n))
                        .unwrap_or(Val::Old);
                    if s.split(" ").nth(0).unwrap() == "*" {
                        Op::Mul(val)
                    } else {
                        Op::Add(val)
                    }
                })
                .unwrap();
            let divisible = split
                .next()
                .unwrap()
                .split("by ")
                .nth(1)
                .unwrap()
                .parse::<i64>()
                .unwrap();
            let true_next = split
                .next()
                .unwrap()
                .split("monkey ")
                .nth(1)
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let false_next = split
                .next()
                .unwrap()
                .split("monkey ")
                .nth(1)
                .unwrap()
                .parse::<usize>()
                .unwrap();
            return Monkey {
                items,
                op,
                divisible,
                true_next,
                false_next,
            };
        })
        .collect::<Vec<_>>();
    let count = monkeys.len();

    let max_mod = monkeys.iter().map(|m| m.divisible).reduce(|acc, i| acc * i).unwrap();

    let mut touches = vec![0; count];

    for _r in 0..10_000 {
        let monkeys_ref = &mut monkeys;
        let mut moves = vec![Vec::<i64>::new(); count];
        for m in 0..count {
            let monkey = &mut monkeys_ref[m];

            // Calc moves
            for &item in &monkey.items {
                touches[m] += 1;
                let mut worry = item;
                match monkey.op {
                    Op::Add(Val::Num(n)) => worry += n,
                    Op::Add(Val::Old) => worry += item,
                    Op::Mul(Val::Num(n)) => worry *= n,
                    Op::Mul(Val::Old) => worry *= item,
                }
                worry = worry % max_mod;
                let next = if worry % monkey.divisible == 0 {
                    monkey.true_next
                } else {
                    monkey.false_next
                };
                moves[next].push(worry);
                // println!("Monkey {} threw item {} to monkey {}", m, worry, next);
            }

            // Clear current items
            monkey.items.clear();

            // Move items and clear
            for (m2, monkey) in monkeys_ref.iter_mut().enumerate() {
                monkey.items.append(&mut moves[m2]);
                moves[m2].clear();
            }
        }
        // println!("Round {}", r + 1);
        // for monkey in monkeys_ref.iter() {
        //     println!("{:?}", monkey.items);
        // }
        // println!()
    }

    touches.sort();
    touches.reverse();
    println!("{:#?}", touches);

    return touches[0] * touches[1];
}

pub fn calc() {
    println!("{}\n{}", calc1(), calc2());
}
// const INPUT: &str = r#"Monkey 0:
// Starting items: 79, 98
// Operation: new = old * 19
// Test: divisible by 23
//   If true: throw to monkey 2
//   If false: throw to monkey 3

// Monkey 1:
// Starting items: 54, 65, 75, 74
// Operation: new = old + 6
// Test: divisible by 19
//   If true: throw to monkey 2
//   If false: throw to monkey 0

// Monkey 2:
// Starting items: 79, 60, 97
// Operation: new = old * old
// Test: divisible by 13
//   If true: throw to monkey 1
//   If false: throw to monkey 3

// Monkey 3:
// Starting items: 74
// Operation: new = old + 3
// Test: divisible by 17
//   If true: throw to monkey 0
//   If false: throw to monkey 1"#;

const INPUT: &str = r#"Monkey 0:
Starting items: 57, 58
Operation: new = old * 19
Test: divisible by 7
  If true: throw to monkey 2
  If false: throw to monkey 3

Monkey 1:
Starting items: 66, 52, 59, 79, 94, 73
Operation: new = old + 1
Test: divisible by 19
  If true: throw to monkey 4
  If false: throw to monkey 6

Monkey 2:
Starting items: 80
Operation: new = old + 6
Test: divisible by 5
  If true: throw to monkey 7
  If false: throw to monkey 5

Monkey 3:
Starting items: 82, 81, 68, 66, 71, 83, 75, 97
Operation: new = old + 5
Test: divisible by 11
  If true: throw to monkey 5
  If false: throw to monkey 2

Monkey 4:
Starting items: 55, 52, 67, 70, 69, 94, 90
Operation: new = old * old
Test: divisible by 17
  If true: throw to monkey 0
  If false: throw to monkey 3

Monkey 5:
Starting items: 69, 85, 89, 91
Operation: new = old + 7
Test: divisible by 13
  If true: throw to monkey 1
  If false: throw to monkey 7

Monkey 6:
Starting items: 75, 53, 73, 52, 75
Operation: new = old * 7
Test: divisible by 2
  If true: throw to monkey 0
  If false: throw to monkey 4

Monkey 7:
Starting items: 94, 60, 79
Operation: new = old + 2
Test: divisible by 3
  If true: throw to monkey 1
  If false: throw to monkey 6"#;
