//! construct all possible positional variations and count how many fulfull the condition and how many do not.
//!
//!Checking King position with last King in position 49,      0.003 ms since last update,    257.823 ms since start
//!From
//!    73291484176 Total combinations,
//!    20614202836 (28.126% of total) were invalid,
//!    25616014800 (34.951% of total, 48.628% of valid) contained neighboring K&Q,
//!    25232642982 (34.428% of total, 47.900% of valid) contained K&Q separated by one other card,
//!    13908281274 (18.977% of total, 26.403% of valid) had no such neighboring K&Q

use std::{fmt::Display, time::Instant};

use crate::permutation_generator::PermutationGenerator;

#[derive(Default)]
pub struct CombinationCounter {
    pub total: u64,
    pub invalid: u64,
    pub has_kq_neighbors: u64,
    pub has_kq_separation_1: u64,
    pub has_neither: u64,
}

impl Display for CombinationCounter {
    #[allow(clippy::cast_precision_loss)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "From\n\
        {:15} Total combinations,\n\
        {:15} ({:6.3}% of total) were invalid,\n\
        {:15} ({:6.3}% of total, {:6.3}% of valid) contained neighboring K&Q,\n\
        {:15} ({:6.3}% of total, {:6.3}% of valid) contained K&Q separated by one other card,\n\
        {:15} ({:6.3}% of total, {:6.3}% of valid) had no such neighboring K&Q",
            self.total,
            self.invalid,
            self.invalid as f64 / self.total as f64 * 100.0,
            self.has_kq_neighbors,
            self.has_kq_neighbors as f64 / self.total as f64 * 100.0,
            self.has_kq_neighbors as f64 / (self.total - self.invalid) as f64 * 100.0,
            self.has_kq_separation_1,
            self.has_kq_separation_1 as f64 / self.total as f64 * 100.0,
            self.has_kq_separation_1 as f64 / (self.total - self.invalid) as f64 * 100.0,
            self.has_neither,
            self.has_neither as f64 / self.total as f64 * 100.0,
            self.has_neither as f64 / (self.total - self.invalid) as f64 * 100.0
        )
    }
}

impl CombinationCounter {
    pub fn count_for_positions(&mut self, pos_k: u64, pos_q: u64) {
        self.total += 1;
        if Self::is_valid_king_and_queen_combination(pos_k, pos_q) {
            let has_sep_0 = Self::has_kq_neighbors(pos_k, pos_q);
            let has_sep_1 = Self::has_kq_separation_1(pos_k, pos_q);
            if has_sep_0 {
                self.has_kq_neighbors += 1;
            }
            if has_sep_1 {
                self.has_kq_separation_1 += 1;
            }
            if !has_sep_0 && !has_sep_1 {
                self.has_neither += 1;
            }
        } else {
            self.invalid += 1;
        }
    }

    pub fn is_valid_king_and_queen_combination(pos_k: u64, pos_q: u64) -> bool {
        pos_k & pos_q == 0
    }

    pub fn has_kq_neighbors(pos_k: u64, pos_q: u64) -> bool {
        (pos_k << 1) & pos_q | (pos_q << 1) & pos_k != 0
    }

    pub fn has_kq_separation_1(pos_k: u64, pos_q: u64) -> bool {
        (pos_k << 2) & pos_q | (pos_q << 2) & pos_k != 0
    }
}

#[allow(clippy::cast_precision_loss)]
pub fn simulate_all() -> CombinationCounter {
    let mut combination_counter = CombinationCounter::default();
    let start_timing = Instant::now();
    let mut update_timing = Instant::now();
    let mut update_semaphore = 123;

    let permutations_kings = PermutationGenerator::default();
    for positions_kings in permutations_kings {
        if update_semaphore != positions_kings.trailing_zeros() {
            update_semaphore = positions_kings.trailing_zeros();
            println!(
                "Checking King position with last King in position {:2}, {:10.3} ms since last update, {:10.3} ms since start",
                update_semaphore + 1, 
                update_timing.elapsed().as_millis() as f64 / 1e3, 
                start_timing.elapsed().as_millis() as f64 / 1e3
            );
            update_timing = Instant::now();
        }

        let permutations_queens = PermutationGenerator::default();
        for positions_queens in permutations_queens {
            combination_counter.count_for_positions(positions_kings, positions_queens);
        }
    }

    combination_counter
}
