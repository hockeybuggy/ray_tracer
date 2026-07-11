use std::cell::Cell;

pub struct Sequence {
    numbers: Vec<f64>,
    index: Cell<usize>,
}

pub fn sequence(numbers: &[f64]) -> Sequence {
    Sequence {
        numbers: numbers.to_vec(),
        index: Cell::new(0),
    }
}

impl Sequence {
    pub fn constant(value: f64) -> Sequence {
        sequence(&[value])
    }

    /// A `count`-long sequence of pseudo-random values in `[0.0, 1.0)`,
    /// generated from `seed` with a fixed algorithm. Same seed, same
    /// numbers, every run: renders stay reproducible for fixture tests
    /// instead of drawing on OS randomness.
    pub fn random(count: usize, seed: u64) -> Sequence {
        let mut state = seed.max(1);
        let numbers = (0..count)
            .map(|_| {
                // xorshift64
                state ^= state << 13;
                state ^= state >> 7;
                state ^= state << 17;
                (state >> 11) as f64 / (1u64 << 53) as f64
            })
            .collect::<Vec<f64>>();
        sequence(&numbers)
    }

    pub fn next(&self) -> f64 {
        let i = self.index.get();
        let value = self.numbers[i];
        self.index.set((i + 1) % self.numbers.len());
        value
    }
}

#[cfg(test)]
mod sequences_tests {
    use crate::sequences;

    #[test]
    fn test_a_number_generator_returns_a_cyclic_sequence_of_numbers() {
        let generator = sequences::sequence(&[0.1, 0.5, 1.0]);

        assert_eq!(generator.next(), 0.1);
        assert_eq!(generator.next(), 0.5);
        assert_eq!(generator.next(), 1.0);
        assert_eq!(generator.next(), 0.1);
    }

    #[test]
    fn test_random_is_deterministic_for_a_given_seed() {
        let a = sequences::Sequence::random(10, 42);
        let b = sequences::Sequence::random(10, 42);

        for _ in 0..10 {
            assert_eq!(a.next(), b.next());
        }
    }

    #[test]
    fn test_random_values_stay_within_zero_and_one() {
        let generator = sequences::Sequence::random(1000, 7);

        for _ in 0..1000 {
            let value = generator.next();
            assert!((0.0..1.0).contains(&value), "value {} out of range", value);
        }
    }
}
