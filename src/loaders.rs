use std::{
    fmt::Debug,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    str::FromStr,
};

fn buf_open(filename: impl AsRef<Path>) -> BufReader<File> {
    let file = File::open(filename).expect("No such file");
    BufReader::new(file)
}

pub fn file_to<T>(filename: impl AsRef<Path>) -> impl Iterator<Item = T>
where
    T::Err: Debug,
    T: FromStr,
{
    buf_open(filename).lines().map(|l| {
        l.expect("Could not parse line")
            .parse::<T>()
            .expect("Failed to parse")
    })
}

pub fn parse_password_req(
    filename: impl AsRef<Path>,
) -> impl Iterator<Item = (crate::aoc2::PasswordReq, String)> {
    buf_open(filename).lines().map(|line| {
        let line = line.expect("Could not parse line");
        let (req, password) = line.split_once(':').unwrap();
        let (range, character) = req.split_once(' ').unwrap();
        let (start, stop) = range.split_once('-').unwrap();
        let password_req = crate::aoc2::PasswordReq {
            range: start.parse().unwrap()..=stop.parse().unwrap(),
            character: character.bytes().next().unwrap(),
        };
        (password_req, password.to_owned())
    })
}

pub fn parse_vecvec_of_trees(filename: impl AsRef<Path>) -> Vec<Vec<bool>> {
    buf_open(filename)
        .lines()
        .map(|line| {
            line.expect("Could not parse line")
                .chars()
                .map(|c| c == '#')
                .collect()
        })
        .collect()
}

pub fn file_to_boarding_card_id(filename: impl AsRef<Path>) -> impl Iterator<Item = u16> {
    /*
    The boarding cards are essentially binary number with other characters.
    Multiplying a number by 8 and adding a an 7-bit number is the same as just concatenating the
    numbers
    */
    buf_open(filename).lines().map(|line| {
        line.expect("Couldn't read line")
            .chars()
            .fold(0, |val, pos| match pos {
                'B' | 'R' => (val << 1) + 1,
                'F' | 'L' => val << 1,
                _ => panic!("Unexpected char"),
            })
    })
}
