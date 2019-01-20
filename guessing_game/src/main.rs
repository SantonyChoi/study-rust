use std::io;

static UNITS: [&str; 10] = [
    "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

static TENS: [&str; 10] = [
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

static MORETENS: [&str; 10] = [
    "", "", "twenty", "thirty", "fourty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

static LARGEUNITS: [&str; 11] = [
    "",
    "",
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
    "sextillion",
    "septillion",
    "octillion",
];

fn main() {
    let mut number = String::new();
    let mut result = String::new();

    println!("Please input number.");

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let mut number: u64 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let mut times: u64 = 0;

    if number == 0 {
        println!("zero");
    } else {
        loop {
            times = times + 1;

            if number == 0 {
                break;
            }

            let this_turn: u64 = get_this_turn(&mut number);
            if this_turn == 0 {
                continue;
            }

            if result != String::new() {
                result = format!(", {}", result);
            }

            let this_chunk = one_step(this_turn);
            let this_last_unit = get_last_unit(times as usize);

            result = format!("{} {}{}", this_chunk, this_last_unit, result);
        }
    }
    println!("{}", result);
}

fn get_this_turn(number: &mut u64) -> u64 {
    let this_turn = *number % 1000;
    *number = *number / 1000;
    this_turn
}

fn one_step(number: u64) -> String {
    let hundred: String = get_hundred((number / 100) as usize);
    let ten: String = get_ten((number % 100) as usize);

    if number / 100 == 0 {
        format!("{}", ten)
    } else if number % 100 == 0 {
        format!("{}", hundred)
        format!("{} {}", hundred, ten)
    }
}

fn get_hundred(number: usize) -> String {
    if number == 0 {
    } else {
        format!("{} hundred", UNITS[number])
    }
}

fn get_ten(number: usize) -> String {
    if number == 0 {
        String::new()
    } else if number < 10 {
        format!("{}", UNITS[number])
    } else if number < 20 {
        format!("{}", TENS[number - 10])
    } else {
        let upper = number / 10;
        let lower = number % 10;
        format!("{} {}", MORETENS[upper], UNITS[lower])
    }
}

fn get_last_unit(number: usize) -> &'static str {
    LARGEUNITS[number]
}
