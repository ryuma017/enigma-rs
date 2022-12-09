use crate::alphabet::Alphabet::{self, *};

#[allow(non_camel_case_types)]
pub enum Kind {
    UKW_A,
    UKW_B,
    UKW_C,
}

impl Kind {
    pub(super) fn wiring(&self) -> [Alphabet; Alphabet::N_ALPHABET] {
        match self {
            Kind::UKW_A => [
                E, J, M, Z, A, L, Y, X, V, B, W, F, C, R, Q, U, O, N, T, S, P, I, K, H, G, D,
            ],
            Kind::UKW_B => [
                Y, R, U, H, Q, S, L, D, P, X, N, G, O, K, M, I, E, B, F, Z, C, W, V, J, A, T,
            ],
            Kind::UKW_C => [
                F, V, P, J, I, A, O, Y, E, D, R, Z, X, W, G, C, T, K, U, Q, S, B, N, M, H, L,
            ],
        }
    }
}
