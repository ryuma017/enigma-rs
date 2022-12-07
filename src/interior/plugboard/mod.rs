mod builder;
mod plugs;
mod sockets;

pub use plugs::Plugs;

use self::builder::PlugboardBuilder;
use crate::alphabet::Alphabet;

#[derive(Debug, Eq, PartialEq)]
pub struct Plugboard {
    wiring: [Alphabet; Alphabet::N_ALPHABET],
}

impl Plugboard {
    pub fn build<'id>() -> PlugboardBuilder<'id> {
        PlugboardBuilder::new()
    }

    pub(crate) fn swap(this: &Self, input: Alphabet) -> Alphabet {
        this.wiring[input.index()]
    }
}

impl Default for Plugboard {
    fn default() -> Self {
        Self {
            wiring: PlugboardBuilder::DEFAULT_WIRING,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::iter::zip;

    use super::{Plugboard, Plugs};

    #[test]
    fn plugboard() {
        let plugboard = Plugboard::build().with_plugs(|sockets| {
            Plugs::new()
                .plug(sockets.A, sockets.B)
                .plug(sockets.C, sockets.D)
                .plug(sockets.E, sockets.F)
                .plug(sockets.G, sockets.H)
                .plug(sockets.I, sockets.J)
                .plug(sockets.K, sockets.L)
                .plug(sockets.M, sockets.N)
                .plug(sockets.O, sockets.P)
                .plug(sockets.Q, sockets.R)
                .plug(sockets.S, sockets.T)
            // .plug(sockets.U, sockets.V) // compile error
        });

        for (input, output) in zip(
            Plugboard::default().wiring.into_iter(),
            plugboard.wiring.into_iter(),
        ) {
            assert_eq!(Plugboard::swap(&plugboard, input), output);
        }
    }
}
