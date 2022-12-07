use crate::alphabet::Alphabet::{self, *};

use super::{plugs::Plugs, sockets::Sockets, Plugboard};

#[derive(Debug, Eq, PartialEq)]
pub struct PlugboardBuilder<'id> {
    sockets: Sockets<'id>,
    wiring: [Alphabet; Alphabet::N_ALPHABET],
}

impl PlugboardBuilder<'_> {
    pub(super) const DEFAULT_WIRING: [Alphabet; Alphabet::N_ALPHABET] = [
        A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    ];

    pub(super) fn new() -> Self {
        Self {
            sockets: Sockets::new(),
            wiring: Self::DEFAULT_WIRING,
        }
    }

    pub fn with_no_plugs(self) -> Plugboard {
        Plugboard {
            wiring: Self::DEFAULT_WIRING,
        }
    }
}

impl<'id> PlugboardBuilder<'id> {
    pub fn with_plugs<const CNT: usize, F>(self, f: F) -> Plugboard
    where
        F: for<'plugboard_id> FnOnce(Sockets<'plugboard_id>) -> Plugs<CNT>,
    {
        let plugs = f(self.sockets);
        let mut wiring = self.wiring;
        plugs.into_iter().for_each(|plug| {
            let (Some(l), Some(r)) = plug.into_parts() else { return; };
            wiring[l.index()] = r;
            wiring[r.index()] = l;
        });
        Plugboard { wiring }
    }
}

impl Default for PlugboardBuilder<'_> {
    fn default() -> Self {
        Self {
            sockets: Sockets::new(),
            wiring: Self::DEFAULT_WIRING,
        }
    }
}
