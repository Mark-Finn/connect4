pub const WIDTH: u8 = 7;
pub const HEIGHT: u8 = 6;

pub const BOARD_SIZE: u8 = WIDTH * HEIGHT;

pub const MIN_SCORE: i8 = -1 * (BOARD_SIZE / 2 + 3) as i8;
pub const MAX_SCORE: i8 = ((BOARD_SIZE + 1) / 2 - 3) as i8;

pub trait Position: Clone {
    fn can_play(&self, col: u8) -> bool;

    fn play(&mut self, col: u8);

    fn is_winning_move(&self, col: u8) -> bool;

    fn get_move_count(&self) -> u8;

    fn get_moves_with_weight(&self) -> Vec<(u8, u8)>;

    fn can_win_next_move(&self) -> bool;

    fn key(&self) -> u64;

    fn symmetric_key(&self) -> u64;

    fn stringify(&self) -> String;

    fn min_possible_score(&self) -> i8 {
        -1 * (BOARD_SIZE - self.get_move_count()) as i8 / 2
    }

    fn next_min_possible_score(&self) -> i8 {
        -1 * (BOARD_SIZE - 2 - self.get_move_count()) as i8 / 2
    }

    fn max_possible_score(&self) -> i8 {
        ((BOARD_SIZE + 1 - self.get_move_count()) / 2) as i8
    }

    fn next_max_possible_score(&self) -> i8 {
        ((BOARD_SIZE - 1 - self.get_move_count()) / 2) as i8
    }
}
