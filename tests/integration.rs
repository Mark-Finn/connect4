#[cfg(test)]
mod integration {
    use connect4::{position_factory, reader};
    use connect4::solver::{Solver, SolverOptions};

    #[test]
    fn test_easy_end_games() {
        test_file("./data/Test_End_Easy", false, 1000, 0);
    }

    #[test]
    fn test_easy_end_games_weak() {
        test_file("./data/Test_End_Easy", true, 1000, 0);
    }

    #[test]
    fn test_easy_mid_games() {
        test_file("./data/Test_Mid_Easy", false, 1000, 0);
    }

    #[test]
    fn test_easy_mid_games_weak() {
        test_file("./data/Test_Mid_Easy", true, 1000, 0);
    }

    #[test]
    fn test_medium_mid_games() {
        test_file("./data/Test_Mid_Medium", false, 100, 0);
    }

    #[test]
    fn test_medium_mid_games_weak() {
        test_file("./data/Test_Mid_Medium", true, 100, 0);
    }

    #[test]
    fn test_easy_start_games() {
        test_file("./data/Test_Start_Easy", false, 1000, 0);
    }

    #[test]
    fn test_easy_start_games_weak() {
        test_file("./data/Test_Start_Easy", true, 1000, 0);
    }

    #[test]
    fn test_medium_start_games() {
        test_file("./data/Test_Start_Medium", false, 1, 0);
    }

    #[test]
    fn test_medium_start_games_weak() {
        test_file("./data/Test_Start_Medium", true, 100, 0);
    }

    #[test]
    fn test_hard_start_games() {
        test_file("./data/Test_Start_Hard", false, 0, 100);
    }

    #[test]
    fn test_hard_start_games_weak() {
        test_file("./data/Test_Start_Hard", true, 0, 100);
    }

    fn test_file(filepath: &str, weak: bool, limit: usize, skip: usize) {
        let mut solver = Solver::new(Some(SolverOptions {
            weak,
        }));
        for (moves, expected) in read_tests(filepath, limit, skip) {
            let position = position_factory::create(moves.as_str()).unwrap();
            let actual = solver.solve(position);
            if !weak {
                assert_eq!(expected, actual);
            } else {
                assert_eq!(unit_value(expected), unit_value(actual));
            }
        }
    }

    fn read_tests(filepath: &str, limit: usize, skip: usize) -> impl Iterator<Item=(String, i8)>  {
        assert!(limit <= 1000);
        assert!(skip < 1000);

        reader::read_positions(filepath)
            .map(|line| reader::line_to_position_score(line.unwrap()))
            .skip(skip)
            .take(limit)
    }

    fn unit_value(v: i8) -> i8 {
        (v >> 7) - (-v >> 7)
    }
}