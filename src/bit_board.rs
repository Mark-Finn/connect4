use std::cmp::min;
use crate::position::{Position, BOARD_SIZE, HEIGHT, WIDTH, BoardType};

// position/mask are 49-bit representations of the board
// position:
//     empty spaces and current player's pieces are '0's
//     opponent's pieces are '1's
// mask:
//     empty spaces are '0's
//     all pieces are '1's
// the bottom left of the board is the first bit (0) and the top right is the final bit (48)
// this means each step up a column is equivalent to multiplying by two numerically
// the top row is needed, but not part of the game board
//                              .  .  .  .  .  .  .
//                              5 12 19 26 33 40 47
//                              4 11 18 25 32 39 46
//                              3 10 17 24 31 38 45
//                              2  9 16 23 30 37 44
//                              1  8 15 22 29 36 43
//                              0  7 14 21 28 35 42
#[derive(Clone)]
pub struct BitBoard {
    pub position: BoardType,
    pub mask: BoardType,
    pub move_count: u8,
}

impl Position for BitBoard {
    fn can_play(&self, col: u8) -> bool {
        self.mask & Self::top_mask(col) == 0
    }

    fn play(&mut self, col: u8) {
        debug_assert!(self.can_play(col));
        self.position ^= self.mask;
        self.mask |= self.mask + Self::bottom_mask(col);
        self.move_count += 1;
    }

    fn is_winning_move(&self, col: u8) -> bool {
        let new_position = self.position_from_col(col);
        return Self::alignment_horizontal(new_position)
            || Self::alignment_positive_diagonal(new_position)
            || Self::alignment_negative_diagonal(new_position)
            || Self::alignment_vertical(new_position);
    }

    fn get_move_count(&self) -> u8 {
        self.move_count
    }

    fn get_moves_with_weight(&self) -> Vec<(u8, u8)> {
        let mut possible_mask = self.possible();
        let opponent_wins = Self::winning_positions(self.position ^ self.mask, self.mask);
        let forced_moves = possible_mask & opponent_wins;
        if forced_moves > 0 {
            if (forced_moves & (forced_moves - 1)) > 0 {
                return Vec::new();
            }
            else {
                possible_mask = forced_moves;
            }
        }
        let non_losing_moves = possible_mask & !(opponent_wins >> 1);

        (0..WIDTH)
            .filter(|col| (self.column_mask(*col) & non_losing_moves) > 0 && self.can_play(*col))
            .map(|col| (col, Self::winning_positions(self.position_from_col(col), self.mask).count_ones() as u8))
            .collect::<Vec<_>>()
    }

    fn can_win_next_move(&self) -> bool {
        for col in 0..WIDTH {
            if self.is_winning_move(col) && self.can_play(col) {
                return true;
            }
        }
        return false;
    }

    fn key(&self) -> BoardType {
        self.position + self.mask
    }

    fn symmetric_key(&self) -> BoardType {
        min(self.key(), self.reverse_key())
    }

    fn stringify(&self) -> String {
        String::from(self)
    }
}

impl BitBoard {
    const BOTTOM_MASK: BoardType = {
        let mut bottom_mask = 0;
        let mut col = 0;
        while col < WIDTH {
            bottom_mask += 1 << (col * (HEIGHT + 1));
            col += 1;
        }
        bottom_mask
    };

    const BOARD_MASK: BoardType = Self::BOTTOM_MASK * ((1 << HEIGHT) - 1);

    pub fn new() -> Self {
        BitBoard {
            position: 0,
            mask: 0,
            move_count: 0,
        }
    }

    fn possible(&self) -> BoardType {
        (self.mask + Self::BOTTOM_MASK) & Self::BOARD_MASK
    }

    fn column_mask(&self, col: u8) -> BoardType {
        ((1 << HEIGHT) - 1) << (col * (HEIGHT + 1))
    }

    fn reverse_key(&self) -> BoardType {
        let mut position: BoardType = 0;
        let mut mask: BoardType = 0;
        for col in (0..WIDTH).rev() {
            let column_mask = self.column_mask(col);
            let shift: i8 = (2 * (col as i8) - (WIDTH as i8 - 1)) * WIDTH as i8;
            let position_col_value = self.position & column_mask;
            let mask_col_value = self.mask & column_mask;
            if shift > 0 {
                position += position_col_value >> shift;
                mask += mask_col_value >> shift;
            }
            else {
                let abs_shift = shift.abs();
                position += position_col_value << abs_shift;
                mask += mask_col_value << abs_shift;
            }
        }
        position + mask
    }

    fn winning_positions(position: BoardType, mask: BoardType) -> BoardType {
        let all_winning_positions = Self::vertical_winning_positions(position)
            | Self::horizontal_winning_positions(position)
            | Self::positive_diagonal_winning_positions(position)
            | Self::negative_diagonal_winning_positions(position);
        all_winning_positions & (Self::BOARD_MASK ^ mask)
    }

    fn vertical_winning_positions(position: BoardType) -> BoardType {
        (position << 1) & (position << 2) & (position << 3)
    }

    fn horizontal_winning_positions(position: BoardType) -> BoardType {
        let x = HEIGHT + 1;
        let x3 = x * 3;
        let m = (position << x) & (position << 2 * x);
        let n = m >> x3;
        return m & (position << x3)
            | m & (position >> x)
            | n & (position << x)
            | n & (position >> x3)
    }

    fn positive_diagonal_winning_positions(position: BoardType) -> BoardType {
        let x = HEIGHT + 2;
        let x3 = x * 3;
        let m = (position << x) & (position << (2 * x));
        let n = m >> x3;
        return m & (position << x3)
            | m & (position >> x)
            | n & (position << x)
            | n & (position >> x3)
    }

    fn negative_diagonal_winning_positions(position: BoardType) -> BoardType {
        let x = HEIGHT;
        let x3 = x * 3;
        let m = (position << x) & (position << (2 * x));
        let n = m >> x3;
        return m & (position << x3)
            | m & (position >> x)
            | n & (position << x)
            | n & (position >> x3)
    }

    fn position_from_col(&self, col: u8) -> BoardType {
        let flipped_position = self.position ^ self.mask;
        let new_mask = self.mask | (self.mask + Self::bottom_mask(col));
        flipped_position ^ new_mask
    }

    fn bottom_mask(col: u8) -> BoardType {
        1 << col*(HEIGHT + 1)
    }

    fn top_mask(col: u8) -> BoardType {
        (1 << (HEIGHT - 1)) << (col * (HEIGHT + 1))
    }

    fn alignment_horizontal(position: BoardType) -> bool {
        let x = HEIGHT + 1;
        let m = position & (position >> x);
        m & (m >> (2 * x)) > 0
    }

    fn alignment_positive_diagonal(position: BoardType) -> bool {
        let x = HEIGHT + 2;
        let m = position & (position >> x);
        m & (m >> (2 * x)) > 0
    }

    fn alignment_negative_diagonal(position: BoardType) -> bool {
        let x = HEIGHT;
        let m = position & (position >> x);
        m & (m >> (2 * x)) > 0
    }

    fn alignment_vertical(position: BoardType) -> bool {
        let x = 1;
        let m = position & (position >> x);
        m & (m >> (2 * x)) > 0
    }


}

impl From<&BitBoard> for String {
    fn from(value: &BitBoard) -> Self {
        let player;
        let opponent;
        if value.move_count % 2 == 0 {
            player = '2';
            opponent = '1';
        }
        else {
            player = '1';
            opponent = '2';
        }
        mask_stringify(|mask|
            if mask & value.mask == 0 {
                '0'
            }
            else if mask & value.position == 0 {
                player
            }
            else {
                opponent
            }
        )
    }
}

fn mask_stringify(func: impl Fn(BoardType) -> char) -> String {
    let mut s = String::with_capacity(((BOARD_SIZE * 2) + HEIGHT) as usize);
    for row in (0..HEIGHT).rev() {
        for col in 0..WIDTH {
            s.push(func(1 << (WIDTH * col + row)));
            s.push(' ');
        }
        s.push('\n');
    }
    return s;
}

// very useful for debugging
#[allow(dead_code)]
fn stringify_position(position: BoardType) -> String {
    mask_stringify(|mask|
        if position & mask > 0 {
            '1'
        }
        else {
            '0'
        }
    )
}
