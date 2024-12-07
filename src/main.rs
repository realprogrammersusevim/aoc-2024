use std::env;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("We need a day number.");
    } else {
        match args[1].parse::<u32>() {
            Ok(day) => match day {
                1 => day1::run(),
                2 => day2::run(),
                3 => day3::run(),
                4 => day4::run(),
                5 => day5::run(),
                6 => day6::run(),
                7 => todo!(),
                8 => todo!(),
                9 => todo!(),
                10 => todo!(),
                11 => todo!(),
                12 => todo!(),
                13 => todo!(),
                14 => todo!(),
                15 => todo!(),
                16 => todo!(),
                17 => todo!(),
                18 => todo!(),
                19 => todo!(),
                20 => todo!(),
                21 => todo!(),
                22 => todo!(),
                23 => todo!(),
                24 => todo!(),
                25 => todo!(),
                _ => eprintln!("Invalid day number."),
            },
            Err(_) => eprintln!("Invalid day number."),
        }
    }
}
