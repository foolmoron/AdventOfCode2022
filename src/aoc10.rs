fn calc1() -> i32 {
    let mut instructions = INPUT
        .lines()
        .map(|line| {
            let mut parts = line.split(" ");
            let ins = parts.next().unwrap();
            let val = parts.next().unwrap_or("0").parse::<i32>().unwrap();
            let cycles = match ins {
                "addx" => 2,
                "noop" => 1,
                _ => 0,
            };
            return (ins, val, cycles);
        })
        .collect::<Vec<_>>();

    let mut sum = 0;
    let mut cycle = 0;
    let mut reg_x = 1;
    while !instructions.is_empty() {
        if cycle > 220 {
            break;
        }

        let (ins, val, ref mut cycles_remaining) = instructions[0];

        // During cycle
        if *cycles_remaining > 0 {
            *cycles_remaining -= 1;
            cycle += 1;
        }
        if (cycle - 20) % 40 == 0 {
            println!("{}: {} {}", cycle, reg_x, reg_x * cycle);
            sum += reg_x * cycle;
        }

        // End of cycle
        if *cycles_remaining <= 0 {
            match ins {
                "addx" => reg_x += val,
                "noop" => (),
                _ => (),
            }
            instructions.remove(0);
        }
    }

    return sum;
}

fn calc2() -> i32 {
    let mut instructions = INPUT
        .lines()
        .map(|line| {
            let mut parts = line.split(" ");
            let ins = parts.next().unwrap();
            let val = parts.next().unwrap_or("0").parse::<i32>().unwrap();
            let cycles = match ins {
                "addx" => 2,
                "noop" => 1,
                _ => 0,
            };
            return (ins, val, cycles);
        })
        .collect::<Vec<_>>();

    let mut cycle = 0;
    let mut reg_x = 1;
    while !instructions.is_empty() {
        let (ins, val, ref mut cycles_remaining) = instructions[0];

        // During cycle
        let pos = cycle % 40;
        print!("{} ", if ((pos - reg_x) as i32).abs() <= 1 { '#' } else { '.' });
        if *cycles_remaining > 0 {
            *cycles_remaining -= 1;
            cycle += 1;
        }
        if cycle % 40 == 0 {
            println!();
        }

        // End of cycle
        if *cycles_remaining <= 0 {
            match ins {
                "addx" => reg_x += val,
                "noop" => (),
                _ => (),
            }
            instructions.remove(0);
        }
    }
    println!();

    return 0;
}

pub fn calc() {
    println!("{}\n{}", calc1(), calc2());
}

const INPUT: &str = r#"addx 2
addx 3
addx 3
addx -2
addx 4
noop
addx 1
addx 4
addx 1
noop
addx 4
addx 1
noop
addx 2
addx 5
addx -28
addx 30
noop
addx 5
addx 1
noop
addx -38
noop
noop
noop
noop
addx 5
addx 5
addx 3
addx 2
addx -2
addx 2
noop
noop
addx -2
addx 12
noop
addx 2
addx 3
noop
addx 2
addx -31
addx 32
addx 7
noop
addx -2
addx -37
addx 1
addx 5
addx 1
noop
addx 31
addx -25
addx -10
addx 13
noop
noop
addx 18
addx -11
addx 3
noop
noop
addx 1
addx 4
addx -32
addx 15
addx 24
addx -2
noop
addx -37
noop
noop
noop
addx 5
addx 5
addx 21
addx -20
noop
addx 6
addx 19
addx -5
addx -8
addx -22
addx 26
addx -22
addx 23
addx 2
noop
noop
noop
addx 8
addx -10
addx -27
addx 33
addx -27
noop
addx 34
addx -33
addx 2
addx 19
addx -12
addx 11
addx -20
addx 12
addx 18
addx -11
addx -14
addx 15
addx 2
noop
addx 3
addx 2
noop
noop
noop
addx -33
noop
addx 1
addx 2
noop
addx 3
addx 4
noop
addx 1
addx 2
noop
noop
addx 7
addx 1
noop
addx 4
addx -17
addx 18
addx 5
addx -1
addx 5
addx 1
noop
noop
noop
noop"#;
