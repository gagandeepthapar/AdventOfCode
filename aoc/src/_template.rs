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
        (0000, 00)
    }

    fn parse(&mut self, filetype: aoclib::Loader) -> () {
        let (year, day) = self.name();

        let folder = match filetype {
            aoclib::Loader::Sample => "samples/",
            aoclib::Loader::Actual => "inputs/",
        };

        let filename = format!("{}{}_{:02}.txt", folder, year, day);

        self.lines = aoclib::read_to_lines(filename);
    }

    fn part1(&mut self) -> Vec<String> {
        vec!["Unsolved".to_string()]
    }

    fn part2(&mut self) -> Vec<String> {
        vec!["Unsolved".to_string()]
    }
}
