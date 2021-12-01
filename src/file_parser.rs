use std::fs::File;
use std::io::prelude::*;

pub fn read_lines<T: Clone>(filename: &str, map: fn(String) -> Option<T>) -> Option<Vec<T>> {
    let mut file = File::open(filename).ok()?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).ok()?;

    let mapped_contents = contents
        .split("\n")
        .collect::<Vec<_>>()
        .iter()
        .filter(|x| !x.is_empty())
        .map(|x| map(x.to_string()))
        .collect::<Vec<_>>();

    if mapped_contents.iter().any(|x| x.is_none()) {
        None
    } else {
        Some(mapped_contents.iter().map(|x| x.clone().unwrap()).collect())
    }
}