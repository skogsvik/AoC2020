use itertools::Itertools;
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

pub fn delimited_file_to<T>(filename: impl AsRef<Path>, delim_byte: u8) -> impl Iterator<Item = T>
where
    T::Err: Debug,
    T: FromStr,
{
    buf_open(filename).split(delim_byte).map(|bytes| {
        std::str::from_utf8(&bytes.expect("Unexpected IO interruption"))
            .expect("Failed to read str from utf8")
            .trim()
            .parse::<T>()
            .expect("Failed to parse")
    })
}

pub fn file_to_lines(filename: impl AsRef<Path>) -> impl Iterator<Item = String> {
    buf_open(filename)
        .lines()
        .map(|line| line.expect("Couldn't read line"))
}

pub fn file_to_paragraphs(filename: impl AsRef<Path>) -> impl Iterator<Item = Vec<String>> {
    // I wish I could figure out how to make this an iterator of iterators, but I'm having trouble
    // with the lifetimes.
    // Perhaps the issue lies in https://github.com/rust-lang/rust/issues/61756 ?
    file_to_lines(filename).peekable().batching(|lines| {
        lines.peek()?; // Don't keep iterating if the iterator is empty
        Some(lines.take_while(|line| !line.is_empty()).collect())
    })
}
