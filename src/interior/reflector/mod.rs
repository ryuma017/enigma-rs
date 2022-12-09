pub mod kind;

use crate::alphabet::Alphabet;
pub use kind::Kind;

pub struct Reflector {
    wiring: [Alphabet; Alphabet::N_ALPHABET],
}

impl Reflector {
    pub fn new(kind: Kind) -> Self {
        Self {
            wiring: kind.wiring(),
        }
    }

    fn reflect(&self, input: Alphabet) -> Alphabet {
        self.wiring[input.index()]
    }
}

#[cfg(test)]
mod tests {
    use std::iter::zip;

    use super::{Kind, Reflector};
    use crate::alphabet::Alphabet::{self, *};

    #[test]
    fn reflector() {
        let reflector = Reflector::new(Kind::UKW_A);

        for (input, output) in zip(Alphabet::ALL, reflector.wiring) {
            assert_eq!(reflector.reflect(input), output);
        }
    }
}
