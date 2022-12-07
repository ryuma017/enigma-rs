#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Alphabet {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

impl Alphabet {
    pub const N_ALPHABET: usize = 26;

    pub fn index(&self) -> usize {
        (*self).into()
    }
}

impl From<Alphabet> for usize {
    fn from(alphabet: Alphabet) -> Self {
        alphabet as usize
    }
}
