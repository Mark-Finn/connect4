use std::env;
use std::fs::File;
use std::io::Write;
use connect4::opening_book::{Explorer, get_opening_book, save_opening_book};
use connect4::{position_factory, reader};
use connect4::position::Position;
use connect4::solver::Solver;
use connect4::transposition_table::TranspositionTable;


fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "solve" => solve(args[2].as_str()),
        "create_position" => create_position(args[2].parse::<u8>().unwrap()),
        "work" => work(
            args[2].parse::<u8>().unwrap(),
            Some((args[3].parse::<usize>().unwrap(), args[4].parse::<usize>().unwrap())),
        ),
        "work_all" => work(args[2].parse::<u8>().unwrap(), None),
        "create_book" => create_book(args[2].as_str()),
        "utilization" => opening_book_utilization(),
        "best_move" => best_move(args[2].as_str()),
        "analyze" => analyze(args[2].as_str()),
        _ => println!("Command Not Recognized"),
    }
}

fn analyze(position_str: &str) {
    let mut solver = Solver::new(None);
    let position = position_factory::create(position_str).unwrap();
    println!("{:?}", solver.analyze(position).iter().map(|col|
        if col.is_some() {
            col.unwrap().to_string()
        }
        else {
            "_".to_string()
        }
    ).collect::<Vec<String>>());
}

fn best_move(position_str: &str) {
    let mut solver = Solver::new(None);
    let position = position_factory::create(position_str).unwrap();
    println!("{}", solver.best_move(position).unwrap_or(0));
}

fn opening_book_utilization() {
    let (used, total) = get_opening_book().utilization();
    println!("{}/{} = {}%", used, total, 100 * used / total);
}

fn create_book(solved_filepath: &str) {
    let mut book: TranspositionTable = get_opening_book();
    for line_result in reader::read_positions(solved_filepath) {
        let (position_string, score) = reader::line_to_position_score(line_result.unwrap());
        let position = position_factory::create(position_string.as_str()).unwrap();
        book.put(position.symmetric_key(), score);
    }
    book.put(0, 1);
    let _ = save_opening_book(book);
}

fn solve(pos: &str) {
    let mut solver = Solver::new(None);
    let position = position_factory::create(pos).unwrap();
    println!("{}", solver.solve(position));
}

fn create_position(depth: u8) {
    let position = position_factory::create("").unwrap();
    let mut explorer = Explorer::new();
    explorer.explore(position, String::new(), depth);

    let filepath = format!("./data/{}_positions", depth);
    let mut file = File::create(filepath.as_str()).unwrap();
    file.write_all(explorer.output.join("\n").as_bytes()).unwrap();
}

fn work(depth: u8, skip_take: Option<(usize, usize)>) {
    let mut solver = Solver::new(None);

    let (skip, take) = skip_take.unwrap_or((0, usize::MAX));

    let read_filepath = format!("./data/{}_positions", depth);
    let write_filepath = if skip_take.is_some() {
        format!("./data/{}_positions_solved_{}_to_{}", depth, skip + 1, skip + take)
    }
    else {
        format!("./data/{}_positions_solved", depth)
    };
    let mut file = File::create(write_filepath.as_str()).unwrap();


    let results = reader::read_positions(read_filepath.as_str());
    let filtered_results = results
        .skip(skip)
        .take(take);

    for line_result in filtered_results {
        let line = line_result.unwrap();
        let position = position_factory::create(line.as_str()).unwrap();
        let score = solver.solve(position);
        file.write_all(format!("{} {}\n", line, score).as_bytes()).unwrap();
    }
}