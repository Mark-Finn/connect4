use std::cmp::Ordering;
use crate::transposition_table::TranspositionTable;
use crate::position::{Position, BOARD_SIZE, WIDTH, MAX_SCORE, MIN_SCORE};

pub struct SolverOptions {
    pub weak: bool,
}

impl SolverOptions {
    fn new() -> Self {
        Self {
            weak: false,
        }
    }
}

pub struct Solver {
    options: SolverOptions,
    transposition_table: TranspositionTable,
    node_count: u64,
}

impl Solver {
    pub fn new(options: Option<SolverOptions>) -> Self {
        Self {
            options: options.unwrap_or(SolverOptions::new()),
            transposition_table: TranspositionTable::new(),
            node_count: 0,
        }
    }

    pub fn solve(&mut self, position: impl Position) -> i8 {
        if position.can_win_next_move() {
            return position.max_possible_score();
        }

        let mut min = position.min_possible_score();
        let mut max = position.max_possible_score();
        if self.options.weak {
            min = -1;
            max = 1;
        }

        loop {
            let mut median = min + (max - min) / 2;
            let half_min = min / 2;
            let half_max = max / 2;
            if median <= 0 && half_min < median {
                median = half_min
            }
            else if median >= 0 && half_max > median {
                median = half_max
            }
            let score = self.negamax(position.clone(), median, median + 1);

            if score <= median {
                max = score;
            }
            else {
                min = score;
            }

            if min >= max {
                return score;
            }
        }
    }

    pub fn analyze(&mut self, position: impl Position, weak: bool) -> Vec<Option<i8>> {
        (0..WIDTH)
            .map(|col| {
                if !position.can_play(col) {
                    None
                }
                else if position.is_winning_move(col) {
                    Some(position.max_possible_score())
                }
                else {
                    let mut position2 = position.clone();
                    position2.play(col);
                    Some(-self.solve(position2))
                }
            })
            .collect()
    }

    pub fn save(&self) -> std::io::Result<()> {
        self.transposition_table.save()
    }

    fn negamax(&mut self, position: impl Position, mut alpha: i8, mut beta: i8) -> i8 {
        debug_assert!(alpha < beta);

        self.node_count += 1;

        let mut next_moves_with_weight = position.get_moves_with_weight();
        if next_moves_with_weight.len() == 0 {
            return position.min_possible_score();
        }

        if position.get_move_count() >= (BOARD_SIZE - 2) {
            return 0;
        }

        let mut min: i8 = position.next_min_possible_score();
        let mut max: i8 = position.next_max_possible_score();
        let key = position.symmetric_key();
        match self.transposition_table.get(key) {
            Some(val) => {
                if val > MAX_SCORE - MIN_SCORE + 1 {
                    min = val + 2 * MIN_SCORE - MAX_SCORE - 2;
                }
                else {
                    max = val + MIN_SCORE - 1;
                }
            },
            None => (),
        }

        if alpha < min {
            alpha = min;
            if alpha >= beta {
                return alpha;
            }
        }

        if beta > max {
            beta = max;
            if alpha >= beta {
                return beta;
            }
        }

        fn order(a: &(u8, u8), b: &(u8, u8)) -> Ordering {
            let mid = WIDTH / 2;

            let (a_col, a_weight) = a;
            let (b_col, b_weight) = b;
            if a_weight == b_weight {
                mid.abs_diff(*a_col).cmp(&mid.abs_diff(*b_col))
            } else {
                b_weight.cmp(&a_weight)
            }
        }
        next_moves_with_weight.sort_by(order);

        for (col, _) in next_moves_with_weight {
            let mut position2 = position.clone();
            position2.play(col);
            let score = -self.negamax(position2, -beta, -alpha);
            if score >= beta {
                self.transposition_table.put(key, score + MAX_SCORE - 2 * MIN_SCORE + 2);
                return score;
            }
            if score > alpha {
                alpha = score;
            }
        }

        self.transposition_table.put(key, alpha - MIN_SCORE + 1);
        return alpha;
    }
}


