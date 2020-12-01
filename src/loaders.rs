use std::{
    fmt::Debug,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    str::FromStr,
};

pub fn file_to<T>(filename: impl AsRef<Path>) -> impl Iterator<Item = T>
where
    T::Err: Debug,
    T: FromStr,
{
    let file = File::open(filename).expect("No such file");
    let buf = BufReader::new(file);
    buf.lines().map(|l| {
        l.expect("Could not parse line")
            .parse::<T>()
            .expect("Failed to parse")
    })
}
