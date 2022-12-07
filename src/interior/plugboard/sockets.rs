use std::marker::PhantomData;

use crate::alphabet::Alphabet;

#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq)]
pub struct Sockets<'id> {
    pub A: Socket<'id>,
    pub B: Socket<'id>,
    pub C: Socket<'id>,
    pub D: Socket<'id>,
    pub E: Socket<'id>,
    pub F: Socket<'id>,
    pub G: Socket<'id>,
    pub H: Socket<'id>,
    pub I: Socket<'id>,
    pub J: Socket<'id>,
    pub K: Socket<'id>,
    pub L: Socket<'id>,
    pub M: Socket<'id>,
    pub N: Socket<'id>,
    pub O: Socket<'id>,
    pub P: Socket<'id>,
    pub Q: Socket<'id>,
    pub R: Socket<'id>,
    pub S: Socket<'id>,
    pub T: Socket<'id>,
    pub U: Socket<'id>,
    pub V: Socket<'id>,
    pub W: Socket<'id>,
    pub X: Socket<'id>,
    pub Y: Socket<'id>,
    pub Z: Socket<'id>,
}

impl Sockets<'_> {
    pub(crate) fn new() -> Self {
        Self {
            A: Socket::A(PhantomData),
            B: Socket::B(PhantomData),
            C: Socket::C(PhantomData),
            D: Socket::D(PhantomData),
            E: Socket::E(PhantomData),
            F: Socket::F(PhantomData),
            G: Socket::G(PhantomData),
            H: Socket::H(PhantomData),
            I: Socket::I(PhantomData),
            J: Socket::J(PhantomData),
            K: Socket::K(PhantomData),
            L: Socket::L(PhantomData),
            M: Socket::M(PhantomData),
            N: Socket::N(PhantomData),
            O: Socket::O(PhantomData),
            P: Socket::P(PhantomData),
            Q: Socket::Q(PhantomData),
            R: Socket::R(PhantomData),
            S: Socket::S(PhantomData),
            T: Socket::T(PhantomData),
            U: Socket::U(PhantomData),
            V: Socket::V(PhantomData),
            W: Socket::W(PhantomData),
            X: Socket::X(PhantomData),
            Y: Socket::Y(PhantomData),
            Z: Socket::Z(PhantomData),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Socket<'id> {
    A(PhantomData<&'id fn(()) -> ()>),
    B(PhantomData<&'id fn(()) -> ()>),
    C(PhantomData<&'id fn(()) -> ()>),
    D(PhantomData<&'id fn(()) -> ()>),
    E(PhantomData<&'id fn(()) -> ()>),
    F(PhantomData<&'id fn(()) -> ()>),
    G(PhantomData<&'id fn(()) -> ()>),
    H(PhantomData<&'id fn(()) -> ()>),
    I(PhantomData<&'id fn(()) -> ()>),
    J(PhantomData<&'id fn(()) -> ()>),
    K(PhantomData<&'id fn(()) -> ()>),
    L(PhantomData<&'id fn(()) -> ()>),
    M(PhantomData<&'id fn(()) -> ()>),
    N(PhantomData<&'id fn(()) -> ()>),
    O(PhantomData<&'id fn(()) -> ()>),
    P(PhantomData<&'id fn(()) -> ()>),
    Q(PhantomData<&'id fn(()) -> ()>),
    R(PhantomData<&'id fn(()) -> ()>),
    S(PhantomData<&'id fn(()) -> ()>),
    T(PhantomData<&'id fn(()) -> ()>),
    U(PhantomData<&'id fn(()) -> ()>),
    V(PhantomData<&'id fn(()) -> ()>),
    W(PhantomData<&'id fn(()) -> ()>),
    X(PhantomData<&'id fn(()) -> ()>),
    Y(PhantomData<&'id fn(()) -> ()>),
    Z(PhantomData<&'id fn(()) -> ()>),
}

impl From<Socket<'_>> for Alphabet {
    fn from(socket: Socket) -> Self {
        match socket {
            Socket::A(_) => Alphabet::A,
            Socket::B(_) => Alphabet::B,
            Socket::C(_) => Alphabet::C,
            Socket::D(_) => Alphabet::D,
            Socket::E(_) => Alphabet::E,
            Socket::F(_) => Alphabet::F,
            Socket::G(_) => Alphabet::G,
            Socket::H(_) => Alphabet::H,
            Socket::I(_) => Alphabet::I,
            Socket::J(_) => Alphabet::J,
            Socket::K(_) => Alphabet::K,
            Socket::L(_) => Alphabet::L,
            Socket::M(_) => Alphabet::M,
            Socket::N(_) => Alphabet::N,
            Socket::O(_) => Alphabet::O,
            Socket::P(_) => Alphabet::P,
            Socket::Q(_) => Alphabet::Q,
            Socket::R(_) => Alphabet::R,
            Socket::S(_) => Alphabet::S,
            Socket::T(_) => Alphabet::T,
            Socket::U(_) => Alphabet::U,
            Socket::V(_) => Alphabet::V,
            Socket::W(_) => Alphabet::W,
            Socket::X(_) => Alphabet::X,
            Socket::Y(_) => Alphabet::Y,
            Socket::Z(_) => Alphabet::Z,
        }
    }
}
