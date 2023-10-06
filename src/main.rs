#![warn(clippy::pedantic)]
mod monte_carlo;
mod permutation_generator;
mod simulate;

pub const NUMBER_OF_CARDS: u8 = 52;

fn main() {
    println!("Simulating with {NUMBER_OF_CARDS} Cards");
    println!("{}", simulate::simulate_all());
    // monte_carlo::simulate_continuously();
}

#[cfg(test)]
mod test {
    use crate::permutation_generator::PermutationGenerator;

    #[test]
    #[allow(clippy::unreadable_literal)]
    pub fn permutation_generator_detects_overruns() {
        let mut perm_gen = PermutationGenerator {
            pos1: 44,
            pos2: 46,
            pos3: 48,
            pos4: 50,
        };
        assert_eq!(Into::<u64>::into(&perm_gen), 0b01010101 << 44);
        assert_eq!(perm_gen.next(), Some(0b10010101 << 44));
        assert_eq!(perm_gen.next(), Some(0b01100101 << 44));
        assert_eq!(perm_gen.next(), Some(0b10100101 << 44));
        assert_eq!(perm_gen.next(), Some(0b11000101 << 44));
        assert_eq!(perm_gen.next(), Some(0b00111001 << 44));
        assert_eq!(perm_gen.next(), Some(0b01011001 << 44));
        assert_eq!(perm_gen.next(), Some(0b10011001 << 44));
        assert_eq!(perm_gen.next(), Some(0b01101001 << 44));
        assert_eq!(perm_gen.next(), Some(0b10101001 << 44));
        assert_eq!(perm_gen.next(), Some(0b11001001 << 44));
        assert_eq!(perm_gen.next(), Some(0b01110001 << 44));
        assert_eq!(perm_gen.next(), Some(0b10110001 << 44));
        assert_eq!(perm_gen.next(), Some(0b11010001 << 44));
        assert_eq!(perm_gen.next(), Some(0b11100001 << 44));
    }
}
