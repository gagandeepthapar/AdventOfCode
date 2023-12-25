use aoclib::Runner;

mod aoc2023 {
    pub mod day01;
    pub mod day02;
}

// mod day01;
// mod day02;

fn main() {
    // set selector
    let solve_selector = aoclib::Selector::All;

    // set file type
    let input_file_type = aoclib::Loader::Actual;

    // instantiate all aoc events
    let mut day01_solver = aoc2023::day01::EventSolver::new();
    let mut day02_solver = aoc2023::day02::EventSolver::new();

    // create vector
    let mut aoc_event_vector: Vec<&mut dyn Runner> = vec![&mut day01_solver, &mut day02_solver];

    // run solution
    match solve_selector {
        aoclib::Selector::All => {
            for day_solver in aoc_event_vector {
                let _ = aoclib::run_solution(day_solver, input_file_type.clone());
            }
        }
        aoclib::Selector::One(day) => {
            let day_solver = &mut aoc_event_vector[day - 1];
            let _ = aoclib::run_solution(*day_solver, input_file_type);
        }
    }
}
