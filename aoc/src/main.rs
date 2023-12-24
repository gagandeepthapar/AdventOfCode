use aoclib::Runner;

mod day01;

fn main() {
    // set selector
    let solve_selector = aoclib::Selector::All;

    // set file type
    let input_file_type = aoclib::Loader::Actual;

    // instantiate all aoc events
    let mut day01_solver = day01::EventSolver::new();
    // let mut d2 = day01::Solution2023_1::new();

    // create vector
    let aoc_event_vector: Vec<&mut dyn Runner> = vec![&mut day01_solver];

    // run solution
    match solve_selector {
        aoclib::Selector::All => {
            for day_solver in aoc_event_vector {
                aoclib::run_solution(day_solver, input_file_type.clone());
            }
        }
        aoclib::Selector::One(_day) => {}
    }
}
