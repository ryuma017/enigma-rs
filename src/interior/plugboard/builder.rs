use crate::alphabetic::Alphabetic;

use super::{plugs::Plugs, sockets::Sockets, Plugboard};

#[derive(Debug, Eq, PartialEq)]
pub struct PlugboardBuilder<'id> {
    sockets: Sockets<'id>,
    wiring: [<char as Alphabetic>::Alphabet; <char as Alphabetic>::N_ALPHABET],
}

impl<'id> PlugboardBuilder<'id> {
    pub(super) const DEFAULT_WIRING: [<char as Alphabetic>::Alphabet;
        <char as Alphabetic>::N_ALPHABET] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];

    pub(super) fn new() -> Self {
        Self {
            sockets: Sockets::new(),
            wiring: Self::DEFAULT_WIRING,
        }
    }

    pub fn with_plugs<const N: usize, F>(self, f: F) -> Plugboard
    where
        F: for<'plugboard_id> FnOnce(Sockets<'plugboard_id>) -> Plugs<N>,
    {
        let plugs = f(self.sockets);
        let mut wiring = self.wiring;
        plugs.into_iter().for_each(|plug| {
            let (l, r) = plug.into_parts();
            if [l, r] != [char::default(); 2] {
                wiring[l.index()] = r;
                wiring[r.index()] = l;
            }
        });
        Plugboard { wiring }
    }

    pub fn with_no_plugs(self) -> Plugboard {
        Plugboard {
            wiring: Self::DEFAULT_WIRING,
        }
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
