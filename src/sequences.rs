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
}
