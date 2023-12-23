#[cfg(test)]
mod unit {
    use connect4::position::BitBoard;
    use connect4::position::Position;
    use connect4::transposition_table::TranspositionTable;

    #[test]
    fn test_vertical_win() {
        let position = BitBoard::try_from("010101").unwrap();
        assert!(position.is_winning_move(0));
    }

    #[test]
    fn test_horizontal_win() {
        let position = BitBoard::try_from("001122").unwrap();
        assert!(position.is_winning_move(3));
    }

    #[test]
    fn test_positive_diagonal_win() {
        let position = BitBoard::try_from("0112232336").unwrap();
        assert!(position.is_winning_move(3));
    }

    #[test]
    fn test_negative_diagonal_win() {
        let position = BitBoard::try_from("6554434331").unwrap();
        assert!(position.is_winning_move(3));
    }
}