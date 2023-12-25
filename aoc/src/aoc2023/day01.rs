use aho_corasick::AhoCorasick;
use std::io::Error;

#[derive(Default)]
pub struct EventSolver {
    pub lines: Vec<String>,
}

impl EventSolver {
    pub fn new() -> Self {
        Self::default()
    }
}

impl aoclib::Runner for EventSolver {
    fn name(&self) -> (usize, usize) {
        (2023, 01)
    }

    fn parse(&mut self, filetype: aoclib::Loader) -> Result<(), Error> {
        let (year, day) = self.name();

        let folder = match filetype {
            aoclib::Loader::Sample => "samples/",
            aoclib::Loader::Actual => "inputs/",
        };

        let filename = format!("{}{}_{:02}.txt", folder, year, day);

        self.lines = aoclib::read_to_lines(filename);
        Ok(())
    }

    fn part1(&mut self) -> Vec<String> {
        let mut total = 0;

        // for each line in input...
        for line in &self.lines {
            let nums = line
                .chars() // separate line into chars
                .filter(|c| c.is_ascii_digit()) // filter out non-ascii digits (0-9)
                .map(|c| c as u8 - b'0') // subtract b'0' to normalize and conv to u8
                .map(|c| c as i32) // convert to i32
                .collect::<Vec<_>>(); // collect into vec

            // add to total
            total += *nums.first().unwrap() * 10 + *nums.last().unwrap();
        }

        // return as Vec<String>
        vec![total.to_string()]
    }

    fn part2(&mut self) -> Vec<String> {
        let mut total = 0;

        // setup array for spelling->digit grab
        let spellmap = vec![
            "one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6", "seven",
            "7", "eight", "8", "nine", "9",
        ];

        // setup Aho Corasick algorithm to find overlapping string matches
        let ac = AhoCorasick::new(spellmap).unwrap();

        for line in &self.lines {
            let matches = ac.find_overlapping_iter(line).collect::<Vec<_>>(); // finds indices of
                                                                              // all matches
            let first = matches.first().unwrap().pattern().as_i32() / 2 + 1; // get the integer
                                                                             // form as i32 using
                                                                             // /2+1
            let last = matches.last().unwrap().pattern().as_i32() / 2 + 1; // same procedure for
                                                                           // second digit

            total += first * 10 + last; // update total
        }

        vec![total.to_string()]
    }
}
