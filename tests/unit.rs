#[cfg(test)]
mod unit {
    use connect4::position::Position;
    use connect4::position_factory;

    #[test]
    fn test_vertical_win() {
        let position = position_factory::create("010101").unwrap();
        assert!(position.is_winning_move(0));
    }

    #[test]
    fn test_horizontal_win() {
        let position = position_factory::create("001122").unwrap();
        assert!(position.is_winning_move(3));
    }

    #[test]
    fn test_positive_diagonal_win() {
        let position = position_factory::create("0112232336").unwrap();
        assert!(position.is_winning_move(3));
    }

    #[test]
    fn test_negative_diagonal_win() {
        let position = position_factory::create("6554434331").unwrap();
        assert!(position.is_winning_move(3));
    }
}