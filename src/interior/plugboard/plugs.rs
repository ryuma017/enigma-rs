use std::ops::{Index, IndexMut};

use crate::alphabet::Alphabet;

use super::sockets::Socket;

#[derive(Debug, Eq, PartialEq, Default)]
pub struct Plugs<const N: usize = 0>([Plug; Plugs::MAX_PLUGS]);

impl Plugs {
    const MAX_PLUGS: usize = 13;

    pub fn new() -> Self {
        Self([Plug::default(); Plugs::MAX_PLUGS])
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

// --- nightly feature --- //

pub trait Satisfied {}

pub struct Condition<const PRED: bool>;

impl Satisfied for Condition<true> {}

trait PlugAble {
    const N_PLUGS: usize;
}

impl<const N: usize> PlugAble for Plugs<N> {
    const N_PLUGS: usize = N;
}

impl<const N: usize> Plugs<N>
where
    Condition<{ N < Plugs::MAX_PLUGS }>: Satisfied,
{
    pub fn plug(mut self, lhs: Socket<'_>, rhs: Socket<'_>) -> Plugs<{ N + 1 }> {
        let plug = Plug::new(lhs, rhs);
        self[N] = plug;
        Plugs(self.0)
    }
}

// --- end of nightly feature --- //

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub struct Plug(Option<Alphabet>, Option<Alphabet>);

impl Plug {
    fn new(lhs: Socket<'_>, rhs: Socket<'_>) -> Plug {
        Plug(Some(lhs.into()), Some(rhs.into()))
    }

    pub(super) const fn into_parts(self) -> (Option<Alphabet>, Option<Alphabet>) {
        (self.0, self.1)
    }
}
