pub use crate::loaders::file_to_lines as load;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_answer1() {
        assert_eq!(answer1(load("input/aoc12")), 904)
    }

    #[test]
    fn test_answer2() {
        assert_eq!(answer2(load("input/aoc12")), 18747)
    }

    #[bench]
    fn bench_answer1(b: &mut Bencher) {
        let input: Vec<_> = load("input/aoc12").collect();
        b.iter(|| answer1(input.iter().cloned()));
    }

    #[bench]
    fn bench_answer2(b: &mut Bencher) {
        let input: Vec<_> = load("input/aoc12").collect();
        b.iter(|| answer2(input.iter().cloned()));
    }
}

fn parse_movements(
    instructions: impl Iterator<Item = String>,
) -> impl Iterator<Item = (char, i32)> {
    instructions.map(|line| (line.chars().next().unwrap(), line[1..].parse().unwrap()))
}

fn sin(angle: i32) -> i32 {
    match angle % 360 {
        0 => 0,
        90 | -270 => 1,
        180 | -180 => 0,
        270 | -90 => -1,
        n => panic!("Angle not along axis {}", n),
    }
}

fn cos(angle: i32) -> i32 {
    match angle.abs() % 360 {
        0 => 1,
        90 => 0,
        180 => -1,
        270 => 0,
        n => panic!("Angle not along axis {}", n),
    }
}

pub fn answer1(instructions: impl Iterator<Item = String>) -> i32 {
    let (x, y, _) = parse_movements(instructions).fold(
        (0, 0, 0),
        |(mut x, mut y, mut angle), (action, val)| {
            match action {
                'N' => x += val,
                'S' => x -= val,
                'E' => y += val,
                'W' => y -= val,
                'L' => angle += val,
                'R' => angle -= val,
                'F' => {
                    x += val * sin(angle);
                    y += val * cos(angle);
                }
                _ => panic!("Unexpected action"),
            }
            (x, y, angle)
        },
    );

    x.abs() + y.abs()
}

pub fn answer2(instructions: impl Iterator<Item = String>) -> i32 {
    let ((ship_x, ship_y), _) = parse_movements(instructions).fold(
        ((0, 0), (10, 1)),
        |(mut ship, mut wp), (action, val)| {
            match action {
                'N' => wp.1 += val,
                'S' => wp.1 -= val,
                'E' => wp.0 += val,
                'W' => wp.0 -= val,
                'L' => {
                    let s = sin(-val);
                    let c = cos(-val);
                    wp = (wp.0 * c + wp.1 * s, -wp.0 * s + wp.1 * c); // Rotational matrix
                }
                'R' => {
                    let s = sin(val);
                    let c = cos(val);
                    wp = (wp.0 * c + wp.1 * s, -wp.0 * s + wp.1 * c);
                }
                'F' => {
                    ship.0 += val * wp.0;
                    ship.1 += val * wp.1;
                }
                _ => panic!("Unexpected action"),
            }
            (ship, wp)
        },
    );

    ship_x.abs() + ship_y.abs()
}
