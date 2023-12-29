use crate::bit_board::BitBoard;
use crate::position::{Position, HEIGHT, WIDTH, BOARD_SIZE};

const BOARD_SIZE_PLUS_ONE: u8 = BOARD_SIZE + 1;

pub fn create(s: &str) -> Result<impl Position, String> {
    match s.len() as u8 {
        BOARD_SIZE_PLUS_ONE => {
            let mut chars = s.chars().fuse();
            let player_up_next = chars.next().unwrap();
            if !matches!(player_up_next, '1' | '2') {
                return Err(format!("'{}' is not a valid starting player", player_up_next));
            }
            let mut position: u64 = 0;
            let mut mask: u64 = 0;
            let mut move_count: u8 = 0;
            let mut player_moves: u8 = 0;
            let mut index: u8 = 0;
            while let Some(c) = chars.next() {
                if c != '0' {
                    move_count += 1;

                    let row = (HEIGHT - 1) - (index / WIDTH);
                    let col = index % WIDTH;
                    let value = 1 << (WIDTH * col + row);

                    mask |= value;
                    if c == player_up_next {
                        position |= value;
                        player_moves += 1;
                    }
                }
                index += 1;
            }

            if move_count / 2  == player_moves {
                Ok(BitBoard { position, mask, move_count })
            } else {
                Err("One player has made at least one extra move".to_string())
            }
        },
        BOARD_SIZE_PLUS_ONE.. => Err("'value' is too long to parse!".to_string()),
        _ => {
            let mut bit_board = BitBoard::new();
            for c in s.chars() {
                match c.to_digit(10) {
                    Some(i) if i as u8 >= WIDTH => return Err(format!("{} is too large", i)),
                    Some(i) if bit_board.can_play(i as u8) => bit_board.play(i as u8),
                    Some(i) => return Err(format!("Cannot play in column {} on move {}", i, bit_board.move_count + 1)),
                    None => return Err(format!("{} is not a digit", c)),
                }
            }
            Ok(bit_board)
        },
    }
}
