use std::{collections::VecDeque};

fn to_snafu(x: i64) -> String {
    let mut x = x;
    let mut parts = VecDeque::<i64>::new();
    while x > 0 {
        parts.push_front(x % 5);
        x /= 5;
    }
    // shift
    parts.push_front(0); // space for the initial digit to carry over
    for i in 0..parts.len() {
        let i_back = parts.len() - i - 1;
        if parts[i_back] >= 3 {
            parts[i_back - 1] += 1;
            parts[i_back] = parts[i_back] - 5;
        }
    }
    if parts[0] == 0 && parts.len() > 1 {
        parts.remove(0);
    }
    return parts.iter().map(|p| match p {
        -2 => "=",
        -1 => "-",
        0 => "0",
        1 => "1",
        2 => "2",
        _ => panic!("invalid part: {}", p),
    }).collect::<String>();
}

fn from_snafu(s: String) -> i64 {
    let mut values = s.chars().map(|c| match c {
        '=' => -2,
        '-' => -1,
        '0' => 0,
        '1' => 1,
        '2' => 2,
        _ => panic!("invalid char: {}", c),
    }).collect::<Vec<i64>>();
    values.reverse();
    let mut sum = 0;
    let mut digit = 1;
    for v in values {
        sum += v * digit;
        digit *= 5;
    }
    return sum;
}

fn test() {
    println!("{} => {} => {}", 0, to_snafu(0), from_snafu(to_snafu(0)));
    println!("{} => {} => {}", 1, to_snafu(1), from_snafu(to_snafu(1)));
    println!("{} => {} => {}", 2, to_snafu(2), from_snafu(to_snafu(2)));
    println!("{} => {} => {}", 3, to_snafu(3), from_snafu(to_snafu(3)));
    println!("{} => {} => {}", 4, to_snafu(4), from_snafu(to_snafu(4)));
    println!("{} => {} => {}", 5, to_snafu(5), from_snafu(to_snafu(5)));
    println!("{} => {} => {}", 6, to_snafu(6), from_snafu(to_snafu(6)));
    println!("{} => {} => {}", 7, to_snafu(7), from_snafu(to_snafu(7)));
    println!("{} => {} => {}", 8, to_snafu(8), from_snafu(to_snafu(8)));
    println!("{} => {} => {}", 9, to_snafu(9), from_snafu(to_snafu(9)));
    println!("{} => {} => {}", 10, to_snafu(10), from_snafu(to_snafu(10)));
    println!("{} => {} => {}", 11, to_snafu(11), from_snafu(to_snafu(11)));
    println!("{} => {} => {}", 15, to_snafu(15), from_snafu(to_snafu(15)));
    println!("{} => {} => {}", 20, to_snafu(20), from_snafu(to_snafu(20)));
    println!("{} => {} => {}", 2022, to_snafu(2022), from_snafu(to_snafu(2022)));
    println!("{} => {} => {}", 12345, to_snafu(12345), from_snafu(to_snafu(12345)));
    println!("{} => {} => {}", 314159265, to_snafu(314159265), from_snafu(to_snafu(314159265)));
}

fn calc1() -> String {
    let sum = INPUT.lines().map(|s| from_snafu(s.to_string())).sum();
    return to_snafu(sum);
}

fn calc2() -> i64 {
    return 0;
}

pub fn calc() {
    println!("{}\n{}", calc1(), calc2());
}

const INPUT: &str = r#"1022-=-111=02=
1-=02
1=-=002=20-21----01
12--=1121
1==1=2==220=
1-102-=2111
1--=0=2=20
10--0
2--02-2=-110=-==2=1
2=11222
1022==
1=22=2121
110200=1-20111--22
20-21000-2
10=2--0-=1-1
11=2-2
1=0-00=-21
20-=122
1-=0=-0-
1===12022=
1010==1-
1=====12=2010
1=-01==0=
1102--=0=2222
22=1=
2220210--=
1-=2021=-=022-
12=
102-22
22022-1=1
1===1--0-2-1
1221-=2==2=
11=
2=012=22
211
2
12100011012
211110==1-1-=0=1=
2=-
1=-02-2--
22-
1=02
1=0--
120-
22-21=20100222=
210-002-101221=
1=-21=--10=212=22
100-==
2=0=00220-=00=--=2
1=00-1122-1-2-1-10
10-
1111=11-021-22=0
1-==-1=-2120020
12--011=-0-1-=0
210-11
1=111
1=--
112=02101
212-=11=-1--=1==-
10-==-1
2==00-110
1=0-1==22=0-0===20-0
10-=1===01=22
2102=
2-=221=2
2-101=02210-20=
10-10201=021=0==1
12=0-11121-012
1=2=
102-=1--=00=11=-
1-0==-102=00
121=
11-2-21-00==0-
112-22000=0=
1=10-2001010--
1-12=02=020
11001-=-1-0212--0
1112=--0=1=-102
1-0010-=-==0
10210=-=0-=1=22-=1
210-0-01-=1==-
12002-1-2
1-=2-10=-102=001=
2==20
1-=0021=1=-
1=0-=0
2--
111--==2220
11-2211=-=2-=0
1-0-=---
1-010=0=-2-10=
10--0012-10=00=
1=2=221210
1-121011=20001==
2==0=2-22021=0=--
21102-
122=-=1-==
10=1=201111
2000=2--1=0-
1=-
2=
2-1=01=-121=-
1-120=101202-01
1==
22=2=01-1-02=0-
1101=-==0==-
1=1-=-=2=-
2021-2=
1==0--=--===220-=--
202=0112=2
12=01-2-=-
1=020-=
1=02101==111=-0
212=2-02
11-0-12-2-2=022
1==21
1=-1121-2220-=22=11
1=0012==
1=-0-2-1-==100=-100
2=-=2=2-0=
1=00
11012=0-
1-02-0-0-12==1
2110
1=2-1=22=1=
211202-
20000-
1-20
1=1001110-22=0=
10-0=00121
12=2==021-==2=201=
1-0--2
22=12111=101=11
11-02-=
1=110-2-==12=1=1020
1=2"#;
