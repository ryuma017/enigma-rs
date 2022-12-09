pub mod kind;

use crate::alphabet::Alphabet;
pub use kind::Kind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rotor {
    wiring: [Alphabet; Alphabet::N_ALPHABET],
    notches: [Option<usize>; 2],
    offset: usize,
    // ring_setting: usize,
}

impl Rotor {
    pub fn new(kind: Kind) -> Self {
        Self {
            wiring: kind.wiring().to_owned(),
            notches: kind
                .notches()
                .map(|option| option.map(|notch| notch.index())),
            offset: usize::default(),
            // ring_setting: usize::default(),
        }
    }

    pub fn with_initial_position(mut self, initial_position: Alphabet) -> Self {
        self.wiring.rotate_left(initial_position.index());
        self.offset = initial_position.index();
        self
    }

    // FIXME: this is not working correctly
    pub fn with_ring_setting(mut self, key: Alphabet) -> Self {
        self.wiring.rotate_right(key.index());
        self.wiring = self.wiring.map(|alphabet| {
            Alphabet::from((alphabet.index() + key.index()).rem_euclid(Alphabet::N_ALPHABET))
        });
        self
    }

    pub(crate) fn step(&mut self) {
        self.wiring.rotate_left(1);
        self.offset = (self.offset + 1).rem_euclid(Alphabet::N_ALPHABET);
    }

    // FIXME: this is not working correctly
    pub(crate) fn map_forward(&self, input: Alphabet) -> Alphabet {
        self.wiring[input.index()]
    }

    fn map_backward(&self, input: Alphabet) -> Alphabet {
        unimplemented!()
    }

    fn is_at_notch(&self) -> bool {
        let current_position = self.offset;
        self.notches.contains(&Some(current_position))
    }
}

#[cfg(test)]
mod tests {
    use super::{Kind, Rotor};
    use crate::alphabet::Alphabet::*;

    #[test]
    fn rotor_map_forward() {
        let mut rotor = Rotor::new(Kind::I)
            .with_initial_position(R)
            .with_ring_setting(T);

        let input = [A, A, A, A, A];
        let mut output = vec![];
        for c in input.into_iter() {
            rotor.step();
            output.push(rotor.map_forward(c));
        }
        assert_eq!(output, [C, X, D, F, Y]);
    }
}
