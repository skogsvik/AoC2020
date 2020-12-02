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
