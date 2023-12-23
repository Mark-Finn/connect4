use std::fs::File;
use std::io::Write;
use connect4::opening_book::Explorer;
use connect4::{position_factory, reader};
use connect4::solver::Solver;

fn main() {
    create_position(9);
    work(9);
}

fn create_position(depth: u8) {
    let position = position_factory::create("").unwrap();
    let mut explorer = Explorer::new();
    explorer.explore(position, String::new(), depth);

    explorer.output.reverse();
    let filepath = format!("./data/{}_positions_rev", depth);
    let mut file = File::create(filepath.as_str()).unwrap();
    file.write_all(explorer.output.join("\n").as_bytes()).unwrap();
}

fn work(depth: u8) {
    let mut solver = Solver::new(None);

    let read_filepath = format!("./data/{}_positions_rev", depth);
    let write_filepath = format!("./data/{}_positions_rev_solved", depth);
    let mut file = File::create(write_filepath.as_str()).unwrap();
    for line_result in reader::read_positions(read_filepath.as_str()) {
        let line = line_result.unwrap();
        let position = position_factory::create(line.as_str()).unwrap();
        let score = solver.solve(position);
        file.write_all(format!("{} {}\n", line, score).as_bytes()).unwrap();
    }
}