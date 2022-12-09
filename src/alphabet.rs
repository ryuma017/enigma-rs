#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Alphabet {
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

    pub(crate) const ALL: [Self; Self::N_ALPHABET] = [
        Alphabet::A,
        Alphabet::B,
        Alphabet::C,
        Alphabet::D,
        Alphabet::E,
        Alphabet::F,
        Alphabet::G,
        Alphabet::H,
        Alphabet::I,
        Alphabet::J,
        Alphabet::K,
        Alphabet::L,
        Alphabet::M,
        Alphabet::N,
        Alphabet::O,
        Alphabet::P,
        Alphabet::Q,
        Alphabet::R,
        Alphabet::S,
        Alphabet::T,
        Alphabet::U,
        Alphabet::V,
        Alphabet::W,
        Alphabet::X,
        Alphabet::Y,
        Alphabet::Z,
    ];

    pub fn index(&self) -> usize {
        (*self).into()
    }
}

impl From<Alphabet> for usize {
    fn from(alphabet: Alphabet) -> Self {
        alphabet as usize
    }
}

impl From<usize> for Alphabet {
    fn from(index: usize) -> Self {
        match index {
            0 => Alphabet::A,
            1 => Alphabet::B,
            2 => Alphabet::C,
            3 => Alphabet::D,
            4 => Alphabet::E,
            5 => Alphabet::F,
            6 => Alphabet::G,
            7 => Alphabet::H,
            8 => Alphabet::I,
            9 => Alphabet::J,
            10 => Alphabet::K,
            11 => Alphabet::L,
            12 => Alphabet::M,
            13 => Alphabet::N,
            14 => Alphabet::O,
            15 => Alphabet::P,
            16 => Alphabet::Q,
            17 => Alphabet::R,
            18 => Alphabet::S,
            19 => Alphabet::T,
            20 => Alphabet::U,
            21 => Alphabet::V,
            22 => Alphabet::W,
            23 => Alphabet::X,
            24 => Alphabet::Y,
            25 => Alphabet::Z,
            _ => unreachable!("`Alphabet::from(usize)` must be in range 0..=25"),
        }
    }
}
