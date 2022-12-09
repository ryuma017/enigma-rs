use crate::alphabet::Alphabet::{self, *};

#[allow(clippy::upper_case_acronyms)]
pub enum Kind {
    I,
    II,
    III,
    IV,
    V,
    VI,
    VII,
    VIII,
}

impl Kind {
    pub(super) fn wiring(&self) -> [Alphabet; Alphabet::N_ALPHABET] {
        match self {
            Kind::I => [
                E, K, M, F, L, G, D, Q, V, Z, N, T, O, W, Y, H, X, U, S, P, A, I, B, R, C, J,
            ],
            Kind::II => [
                A, J, D, K, S, I, R, U, X, B, L, H, W, T, M, C, Q, G, Z, N, P, Y, F, V, O, E,
            ],
            Kind::III => [
                B, D, F, H, J, L, C, P, R, T, X, V, Z, N, Y, E, I, W, G, A, K, M, U, S, Q, O,
            ],
            Kind::IV => [
                E, S, O, V, P, Z, J, A, Y, Q, U, I, R, H, X, L, N, F, T, G, K, D, C, M, W, B,
            ],
            Kind::V => [
                V, Z, B, R, G, I, T, Y, U, P, S, D, N, H, L, X, A, W, M, J, Q, O, F, E, C, K,
            ],
            Kind::VI => [
                J, P, G, V, O, U, M, F, Y, Q, B, E, N, H, Z, R, D, K, A, S, X, L, I, C, T, W,
            ],
            Kind::VII => [
                N, Z, J, H, G, R, C, X, M, Y, S, W, B, O, U, F, A, I, V, L, P, E, K, Q, D, T,
            ],
            Kind::VIII => [
                F, K, Q, H, T, L, X, O, C, B, J, S, P, D, Z, R, A, M, E, W, N, I, U, Y, G, V,
            ],
        }
    }

    pub(super) fn notches(&self) -> [Option<Alphabet>; 2] {
        match self {
            Kind::I => [Some(Q), None],
            Kind::II => [Some(E), None],
            Kind::III => [Some(V), None],
            Kind::IV => [Some(J), None],
            Kind::V => [Some(Z), None],
            Kind::VI | Kind::VII | Kind::VIII => [Some(Z), Some(M)],
        }
    }
}
