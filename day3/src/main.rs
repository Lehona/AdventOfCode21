const BITLEN: usize = 12;

macro_rules! BITFORMAT {
    () => {
        "{:012b}"
    };
}
macro_rules! INPUT {
    () => {
        "input.txt"
    };
}

fn calc_gamma(lines: &[String]) -> usize {
    let nr_of_lines = lines.len();
    let mut occurences_of_1 = [0; BITLEN];

    for line in lines {
        for (idx, c) in line.chars().enumerate() {
            if c == '1' {
                occurences_of_1[idx] += 1;
            }
        }
    }

    occurences_of_1
        .iter()
        .map(|n| (*n * 2 >= nr_of_lines) as usize)
        .fold(0, |mut acc, n| {
            acc <<= 1;
            acc | n
        })
}

fn calc_epsilon(lines: &[String]) -> usize {
    let nr_of_lines = lines.len();
    let mut occurences_of_1 = [0; BITLEN];

    for line in lines {
        for (idx, c) in line.chars().enumerate() {
            if c == '0' {
                occurences_of_1[idx] += 1;
            }
        }
    }

    occurences_of_1
        .iter()
        .map(|n| (*n * 2 > nr_of_lines) as usize)
        .fold(0, |mut acc, n| {
            acc <<= 1;
            acc | n
        })
}

fn main() {
    let input = include_str!(INPUT!());
    let lines: Vec<String> = input.lines().map(|l| l.to_owned()).collect();

    let gamma = calc_gamma(&lines);
    let epsilon = calc_epsilon(&lines);

    println!(
        "gamma: {:012b} epsilon: {:012b} e*g: {}",
        gamma,
        epsilon,
        gamma * epsilon
    );

    let mut oxygen_rating = lines.clone();
    for idx in 0..BITLEN {
        let oxygen_gamma = format!(BITFORMAT!(), calc_gamma(&oxygen_rating));
        let bit = oxygen_gamma.bytes().nth(idx);

        oxygen_rating.retain(|e| e.bytes().nth(idx) == bit);

        if oxygen_rating.len() == 1 {
            break;
        }
    }

    let mut co2_scrubber = lines.clone();
    for idx in 0..BITLEN {
        let co2_epsilon = format!(BITFORMAT!(), calc_epsilon(&co2_scrubber));
        let bit = co2_epsilon.bytes().nth(idx);

        co2_scrubber.retain(|e| e.bytes().nth(idx) == bit);

        if co2_scrubber.len() == 1 {
            break;
        }
    }

    let oxy = usize::from_str_radix(&oxygen_rating[0], 2).unwrap();
    let co2 = usize::from_str_radix(&co2_scrubber[0], 2).unwrap();
    println!("oxy: {}, co2: {}, life support: {}", oxy, co2, oxy * co2);
}
