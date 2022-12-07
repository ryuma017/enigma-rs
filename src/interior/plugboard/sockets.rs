use std::marker::PhantomData;

#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Default)]
pub struct Sockets<'id> {
    pub A: Socket<'id, 'A'>,
    pub B: Socket<'id, 'B'>,
    pub C: Socket<'id, 'C'>,
    pub D: Socket<'id, 'D'>,
    pub E: Socket<'id, 'E'>,
    pub F: Socket<'id, 'F'>,
    pub G: Socket<'id, 'G'>,
    pub H: Socket<'id, 'H'>,
    pub I: Socket<'id, 'I'>,
    pub J: Socket<'id, 'J'>,
    pub K: Socket<'id, 'K'>,
    pub L: Socket<'id, 'L'>,
    pub M: Socket<'id, 'M'>,
    pub N: Socket<'id, 'N'>,
    pub O: Socket<'id, 'O'>,
    pub P: Socket<'id, 'P'>,
    pub Q: Socket<'id, 'Q'>,
    pub R: Socket<'id, 'R'>,
    pub S: Socket<'id, 'S'>,
    pub T: Socket<'id, 'T'>,
    pub U: Socket<'id, 'U'>,
    pub V: Socket<'id, 'V'>,
    pub W: Socket<'id, 'W'>,
    pub X: Socket<'id, 'X'>,
    pub Y: Socket<'id, 'Y'>,
    pub Z: Socket<'id, 'Z'>,
}

impl<'id> Sockets<'id> {
    pub(super) fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

#[derive(Debug, Eq, PartialEq, Default)]
pub struct Socket<'id, const SYMBOL: char> {
    _plugboard_id: PhantomData<&'id fn(()) -> ()>,
}
