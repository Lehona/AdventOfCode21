use std::error::Error;

const INPUT: &str = include_str!("input.txt");

fn main() -> Result<(), Box<dyn Error>> {
    let meters: Vec<i32> = INPUT.lines().map(|l| l.trim().parse().unwrap()).collect();
    let increases = meters.windows(2).filter(|w| w[0] < w[1]).count();

    println!("There have been {} increases in depth.", increases);

    let measurement_windows: Vec<i32> = meters.windows(3).map(|w| w[0] + w[1] + w[2]).collect();
    let window_increases = measurement_windows
        .windows(2)
        .filter(|w| w[0] < w[1])
        .count();
    println!(
        "There have been {} windowed increases in depth.",
        window_increases
    );

    Ok(())
}
