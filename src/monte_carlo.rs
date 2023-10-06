//! simulate shuffling and checking in a monte carlo fashiong. I.E. continue doing it until the average stabilizes

use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::simulate::CombinationCounter;
use crate::NUMBER_OF_CARDS;

static K: u8 = 1;
static Q: u8 = 2;

pub fn simulate_continuously() {
    let mut inital_deck = [0u8; 52];
    inital_deck[0] = K;
    inital_deck[1] = K;
    inital_deck[2] = K;
    inital_deck[3] = K;
    inital_deck[4] = Q;
    inital_deck[5] = Q;
    inital_deck[6] = Q;
    inital_deck[7] = Q;
    inital_deck.shuffle(&mut thread_rng());
    let mut deck = inital_deck;

    let mut combination_counter = CombinationCounter::default();

    for (i, _) in (0..).enumerate() {
        deck.shuffle(&mut thread_rng());
        let k_pos = vec_to_kingu64(&deck);
        let q_pos = vec_to_queenu64(&deck);
        combination_counter.count_for_positions(k_pos, q_pos);

        if i % 1_000_000 == 0 {
            println!("{combination_counter}");
        }
    }
}

fn vec_to_kingu64(slice: &[u8; NUMBER_OF_CARDS as usize]) -> u64 {
    let mut re = 0u64;
    for (index, elem) in slice.iter().enumerate() {
        if elem == &K {
            re |= 1 << index;
        }
    }
    re
}
fn vec_to_queenu64(slice: &[u8; NUMBER_OF_CARDS as usize]) -> u64 {
    let mut re = 0u64;
    for (index, elem) in slice.iter().enumerate() {
        if elem == &Q {
            re |= 1 << index;
        }
    }
    re
}
