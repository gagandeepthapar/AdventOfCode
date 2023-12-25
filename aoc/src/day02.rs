use std::io::Error;

pub struct EventSolver {
    all_games: Vec<Vec<Turn>>,
    max_red: i32,
    max_green: i32,
    max_blue: i32,
}

impl EventSolver {
    pub fn new() -> Self {
        Self {
            all_games: Vec::new(),
            max_red: 12,
            max_green: 13,
            max_blue: 14,
        }
    }
}

struct Turn {
    red: i32,
    green: i32,
    blue: i32,
}

impl Turn {
    fn parse_string(turn: &str) -> Turn {
        let split_turn: Vec<(i32, String)> = turn
            .split(", ")
            .map(|num_col| {
                let (num, col) = num_col.split_once(" ").unwrap();
                (num.parse().unwrap(), col.to_string().to_uppercase())
            })
            .collect();

        let mut turn = Turn {
            red: 0,
            green: 0,
            blue: 0,
        };

        for (num, col) in split_turn {
            match &col[0..1] {
                "R" => turn.red = num,
                "B" => turn.blue = num,
                "G" => turn.green = num,
                _ => println!("Invalid Input!"),
            };
        }

        turn
    }
}

impl aoclib::Runner for EventSolver {
    fn name(&self) -> (usize, usize) {
        (2023, 02)
    }

    fn parse(&mut self, filetype: aoclib::Loader) -> Result<(), Error> {
        let (year, day) = self.name();

        let folder = match filetype {
            aoclib::Loader::Sample => "samples/",
            aoclib::Loader::Actual => "inputs/",
        };

        let filename = format!("{}{}_{:02}.txt", folder, year, day);
        let lines: Vec<String> = aoclib::read_to_lines(filename);

        self.all_games = lines
            .iter()
            .map(|s| {
                s.split_once(": ")
                    .unwrap() // ["Game n", "a red, b green, c blue; ..."]
                    .1 // ["a red, b green, c blue; ..."]
                    .split("; ") // ["a red, b green, c blue", "..."]
                    .map(|s| Turn::parse_string(s))
                    .collect()
            })
            .collect();

        Ok(())
    }

    fn part1(&mut self) -> Vec<String> {
        let mut total = 0;
        let mut maxred: bool;
        let mut maxblue: bool;
        let mut maxgreen: bool;

        for (idx, game) in self.all_games.iter().enumerate() {
            maxred = game
                .iter()
                .max_by_key(|turn| turn.red)
                .map(|turn| turn.red)
                .map(|maxnum| maxnum <= self.max_red)
                .unwrap();
            maxblue = game
                .iter()
                .max_by_key(|turn| turn.blue)
                .map(|turn| turn.blue)
                .map(|maxnum| maxnum <= self.max_blue)
                .unwrap();
            maxgreen = game
                .iter()
                .max_by_key(|turn| turn.green)
                .map(|turn| turn.green)
                .map(|maxnum| maxnum <= self.max_green)
                .unwrap();

            if maxred && maxgreen && maxblue {
                total += idx + 1;
            }
        }
        vec![total.to_string()]
    }

    fn part2(&mut self) -> Vec<String> {
        let mut total = 0;
        let mut maxred: i32;
        let mut maxblue: i32;
        let mut maxgreen: i32;

        for (_idx, game) in self.all_games.iter().enumerate() {
            maxred = game
                .iter()
                .max_by_key(|turn| turn.red)
                .map(|turn| turn.red)
                .unwrap();
            maxblue = game
                .iter()
                .max_by_key(|turn| turn.blue)
                .map(|turn| turn.blue)
                .unwrap();
            maxgreen = game
                .iter()
                .max_by_key(|turn| turn.green)
                .map(|turn| turn.green)
                .unwrap();

            total += maxred * maxgreen * maxblue;
        }
        vec![total.to_string()]
    }
}
