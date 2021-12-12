use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let mut total_fuel = 0;
    let mut additional_fuel = 0;

    for line in io::stdin().lock().lines().flatten() {
        let mut fuel_required = (line.parse::<i32>().unwrap() / 3) - 2;
        total_fuel += fuel_required;

        fuel_required = (fuel_required / 3) - 2;
        while fuel_required > 0 {
            additional_fuel += fuel_required;
            fuel_required = (fuel_required / 3) - 2;
        }
    }

    println!("(1) Fuel required is {}", total_fuel);
    println!(
        "(2) Revised fuel required is {}",
        total_fuel + additional_fuel
    );

    Ok(())
}
