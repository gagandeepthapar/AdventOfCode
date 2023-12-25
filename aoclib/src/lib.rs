use std::env;
use std::fs::read_to_string;
use std::io::Error;
use std::path::Path;
use std::time::Instant;

pub trait Runner {
    // name
    fn name(&self) -> (usize, usize);

    // parse
    fn parse(&mut self, filetype: Loader) -> Result<(), Error>;
    // part1
    fn part1(&mut self) -> Vec<String>;

    // part2
    fn part2(&mut self) -> Vec<String>;
}

pub enum Selector {
    All,
    One(usize),
}

#[derive(Clone, Copy)]
pub enum Loader {
    Sample,
    Actual,
}

pub fn read_to_lines<T: AsRef<Path>>(pathname: T) -> Vec<String> {
    // read specified file at pathname to vector of strings separated by newlines
    let cd = env::current_dir().expect("Failed");
    let relpath = cd.join("../").join(pathname);
    read_to_string(relpath)
        .expect("Missing File")
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

pub fn run_solution<T: Runner + ?Sized>(runnable: &mut T, filetype: Loader) -> Result<(), Error> {
    // print header
    let (year, day) = runnable.name();
    println!("----- {}, Day {:02} -----", year, day);

    // parse input file
    let start = Instant::now();
    let _ = runnable.parse(filetype)?;
    let duration = start.elapsed().as_micros();
    println!("{:03}.{:03}ms Parsing...", duration / 1000, duration % 1000);
    println!("");

    // start part 1
    let start = Instant::now();
    let part1sol = runnable.part1();
    let duration = start.elapsed().as_micros();
    println!("{:03}.{:03}ms Part 1...", duration / 1000, duration % 1000);
    for line in part1sol {
        println!("\t{}", line);
    }
    println!("");

    // start part 2
    let start = Instant::now();
    let part1sol = runnable.part2();
    let duration = start.elapsed().as_micros();
    println!("{:03}.{:03}ms Part 2...", duration / 1000, duration % 1000);
    for line in part1sol {
        println!("\t{}", line);
    }
    println!("");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_to_lines() {
        let stringlist = read_to_lines("sample.txt".to_string());
        assert_eq!(stringlist.len(), 3);
        assert_eq!(stringlist[0], "This is a sample text file.".to_string());
    }
}
