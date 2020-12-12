use std::io::{self, BufRead};

fn interpretation1(command: &str, value: i32, orientation: &mut f32, position: &mut (i32, i32)) {
    match command {
        "N" => position.1 += value,
        "S" => position.1 -= value,
        "E" => position.0 += value,
        "W" => position.0 -= value,
        "L" => *orientation += value as f32,
        "R" => *orientation -= value as f32,
        "F" => {
            let (sin, cos) = orientation.to_radians().sin_cos();
            position.0 += value * cos as i32;
            position.1 += value * sin as i32;
        }
        _ => unreachable!(),
    }
}

fn rotate_point(rotation: i32, point: &(i32, i32)) -> (i32, i32) {
    let (sin, cos) = (rotation as f32).to_radians().sin_cos();
    let (sin, cos) = (sin as i32, cos as i32);
    (cos * point.0 - sin * point.1, sin * point.0 + cos * point.1)
}

fn interpretation2(
    command: &str,
    value: i32,
    ship_position: &mut (i32, i32),
    waypoint_position: &mut (i32, i32),
) {
    match command {
        "N" => waypoint_position.1 += value,
        "S" => waypoint_position.1 -= value,
        "E" => waypoint_position.0 += value,
        "W" => waypoint_position.0 -= value,
        "L" => *waypoint_position = rotate_point(value, waypoint_position),
        "R" => *waypoint_position = rotate_point(-value, waypoint_position),
        "F" => {
            ship_position.0 += waypoint_position.0 * value;
            ship_position.1 += waypoint_position.1 * value;
        }
        _ => unreachable!(),
    }
}

fn main() -> io::Result<()> {
    let mut ship_orientation: f32 = 0.0;
    let mut ship_position: (i32, i32) = (0, 0);
    let mut waypoint_position: (i32, i32) = (10, 1);

    for line in io::stdin().lock().lines() {
        let line = line?;
        let (command, value) = line.split_at(1);
        // interpretation1(command, value.parse.unwrap(), &mut ship_orientation, &mut ship_position);
        interpretation2(
            command,
            value.parse().unwrap(),
            &mut ship_position,
            &mut waypoint_position,
        );
    }

    println!(
        "Manhatttan distance = {}",
        ship_position.0.abs() + ship_position.1.abs()
    );

    Ok(())
}
