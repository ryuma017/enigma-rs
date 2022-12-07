use std::ops::{Index, IndexMut};

use super::sockets::Socket;

/// `Plugs<N>::plug` just takes ownership of the pair of sockets,
/// and returns a new Plugs<N+1>
#[derive(Debug, Eq, PartialEq, Default)]
pub struct Plugs<const N: usize = 0>([Plug; Plugs::MAX_PLUGS]);

impl Plugs {
    const MAX_PLUGS: usize = 13;

    pub fn new() -> Self {
        Default::default()
    }
}

impl<const N: usize> IntoIterator for Plugs<N> {
    type Item = Plug;
    type IntoIter = std::array::IntoIter<Plug, { Plugs::MAX_PLUGS }>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<const N: usize> Index<usize> for Plugs<N> {
    type Output = Plug;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const N: usize> IndexMut<usize> for Plugs<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

// FIXME: Remove boilerplate
impl Plugs<0> {
    pub fn plug<const LHS_SYMBOL: char, const RHS_SYMBOL: char>(
        mut self,
        _lhs: Socket<LHS_SYMBOL>,
        _rhs: Socket<RHS_SYMBOL>,
    ) -> Plugs<1> {
        let plug = Plug(LHS_SYMBOL, RHS_SYMBOL);
        self[0] = plug;
        Plugs(self.0)
    }
}

impl Plugs<1> {
    pub fn plug<const LHS_SYMBOL: char, const RHS_SYMBOL: char>(
        mut self,
        _lhs: Socket<LHS_SYMBOL>,
        _rhs: Socket<RHS_SYMBOL>,
    ) -> Plugs<2> {
        let plug = Plug(LHS_SYMBOL, RHS_SYMBOL);
        self[1] = plug;
        Plugs(self.0)
    }
}

impl Plugs<2> {
    pub fn plug<const LHS_SYMBOL: char, const RHS_SYMBOL: char>(
        mut self,
        _lhs: Socket<LHS_SYMBOL>,
        _rhs: Socket<RHS_SYMBOL>,
    ) -> Plugs<3> {
        let plug = Plug(LHS_SYMBOL, RHS_SYMBOL);
        self[2] = plug;
        Plugs(self.0)
    }
}

impl Plugs<3> {
    pub fn plug<const LHS_SYMBOL: char, const RHS_SYMBOL: char>(
        mut self,
        _lhs: Socket<LHS_SYMBOL>,
        _rhs: Socket<RHS_SYMBOL>,
    ) -> Plugs<4> {
        let plug = Plug(LHS_SYMBOL, RHS_SYMBOL);
        self[3] = plug;
        Plugs(self.0)
    }
}

impl Plugs<4> {
    pub fn plug<const LHS_SYMBOL: char, const RHS_SYMBOL: char>(
        mut self,
        _lhs: Socket<LHS_SYMBOL>,
        _rhs: Socket<RHS_SYMBOL>,
    ) -> Plugs<5> {
        let plug = Plug(LHS_SYMBOL, RHS_SYMBOL);
        self[4] = plug;
        Plugs(self.0)
    }
}

impl Plugs<5> {
    pub fn plug<const LHS_SYMBOL: char, const RHS_SYMBOL: char>(
        mut self,
        _lhs: Socket<LHS_SYMBOL>,
        _rhs: Socket<RHS_SYMBOL>,
    ) -> Plugs<6> {
        let plug = Plug(LHS_SYMBOL, RHS_SYMBOL);
        self[5] = plug;
        Plugs(self.0)
    }
}

impl Plugs<6> {
    pub fn plug<const LHS_SYMBOL: char, const RHS_SYMBOL: char>(
        mut self,
        _lhs: Socket<LHS_SYMBOL>,
        _rhs: Socket<RHS_SYMBOL>,
    ) -> Plugs<7> {
        let plug = Plug(LHS_SYMBOL, RHS_SYMBOL);
        self[6] = plug;
        Plugs(self.0)
    }
}

impl Plugs<7> {
    pub fn plug<const LHS_SYMBOL: char, const RHS_SYMBOL: char>(
        mut self,
        _lhs: Socket<LHS_SYMBOL>,
        _rhs: Socket<RHS_SYMBOL>,
    ) -> Plugs<8> {
        let plug = Plug(LHS_SYMBOL, RHS_SYMBOL);
        self[7] = plug;
        Plugs(self.0)
    }
}

impl Plugs<8> {
    pub fn plug<const LHS_SYMBOL: char, const RHS_SYMBOL: char>(
        mut self,
        _lhs: Socket<LHS_SYMBOL>,
        _rhs: Socket<RHS_SYMBOL>,
    ) -> Plugs<9> {
        let plug = Plug(LHS_SYMBOL, RHS_SYMBOL);
        self[8] = plug;
        Plugs(self.0)
    }
}

impl Plugs<9> {
    pub fn plug<const LHS_SYMBOL: char, const RHS_SYMBOL: char>(
        mut self,
        _lhs: Socket<LHS_SYMBOL>,
        _rhs: Socket<RHS_SYMBOL>,
    ) -> Plugs<10> {
        let plug = Plug(LHS_SYMBOL, RHS_SYMBOL);
        self[9] = plug;
        Plugs(self.0)
    }
}

impl Plugs<10> {
    pub fn plug<const LHS_SYMBOL: char, const RHS_SYMBOL: char>(
        mut self,
        _lhs: Socket<LHS_SYMBOL>,
        _rhs: Socket<RHS_SYMBOL>,
    ) -> Plugs<11> {
        let plug = Plug(LHS_SYMBOL, RHS_SYMBOL);
        self[10] = plug;
        Plugs(self.0)
    }
}

impl Plugs<11> {
    pub fn plug<const LHS_SYMBOL: char, const RHS_SYMBOL: char>(
        mut self,
        _lhs: Socket<LHS_SYMBOL>,
        _rhs: Socket<RHS_SYMBOL>,
    ) -> Plugs<12> {
        let plug = Plug(LHS_SYMBOL, RHS_SYMBOL);
        self[11] = plug;
        Plugs(self.0)
    }
}

impl Plugs<12> {
    pub fn plug<const LHS_SYMBOL: char, const RHS_SYMBOL: char>(
        mut self,
        _lhs: Socket<LHS_SYMBOL>,
        _rhs: Socket<RHS_SYMBOL>,
    ) -> Plugs<13> {
        let plug = Plug(LHS_SYMBOL, RHS_SYMBOL);
        self[12] = plug;
        Plugs(self.0)
    }
}

#[derive(Debug, Eq, PartialEq, Default)]
pub struct Plug(char, char);

impl Plug {
    pub fn new<const LHS_SYMBOL: char, const RHS_SYMBOL: char>() -> Plug {
        Plug(LHS_SYMBOL, RHS_SYMBOL)
    }

    pub const fn into_parts(self) -> (char, char) {
        (self.0, self.1)
    }
}
