mod aoc1;
mod aoc2;
mod aoc3;
mod aoc4;
mod aoc5;
mod aoc6;
mod aoc7;
mod aoc8;
mod aoc9;
mod aoc10;
mod aoc11;
mod aoc12;
mod aoc13;
mod aoc14;
mod aoc15;
mod aoc16;
mod aoc16b;
// mod aoc17;
// mod aoc18;
// mod aoc19;
// mod aoc20;
// mod aoc21;
// mod aoc22;
// mod aoc23;
// mod aoc24;
// mod aoc25;


fn main() {
    let calc = match std::env::args().nth(1).unwrap().as_str() {
        "1" => aoc1::calc,
        "2" => aoc2::calc,
        "3" => aoc3::calc,
        "4" => aoc4::calc,
        "5" => aoc5::calc,
        "6" => aoc6::calc,
        "7" => aoc7::calc,
        "8" => aoc8::calc,
        "9" => aoc9::calc,
        "10" => aoc10::calc,
        "11" => aoc11::calc,
        "12" => aoc12::calc,
        "13" => aoc13::calc,
        "14" => aoc14::calc,
        "15" => aoc15::calc,
        "16" => aoc16::calc,
        "16b" => aoc16b::calc,
        // "17" => aoc17::calc,
        // "18" => aoc18::calc,
        // "19" => aoc19::calc,
        // "20" => aoc20::calc,
        // "21" => aoc21::calc,
        // "22" => aoc22::calc,
        // "23" => aoc23::calc,
        // "24" => aoc24::calc,
        // "25" => aoc25::calc,
        &_ => todo!()
    };
    calc();
}
