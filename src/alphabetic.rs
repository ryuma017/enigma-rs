pub(crate) trait Alphabetic {
    type Alphabet;
    const N_ALPHABET: usize = 26;

    fn index(&self) -> usize;
    fn from_index(index: usize) -> Self;
}

impl Alphabetic for char {
    type Alphabet = char;

    fn index(&self) -> usize {
        debug_assert!(*self >= 'A' && *self <= 'Z');
        (*self as u8 - 0x41).into()
    }

    fn from_index(index: usize) -> Self {
        debug_assert!(index < Self::N_ALPHABET);
        (index as u8 + 0x41).into()
    }
}

impl Alphabetic for u8 {
    type Alphabet = u8;

    fn index(&self) -> usize {
        debug_assert!(*self >= 0x41 && *self <= 0x5A);
        (*self - 0x41).into()
    }

    fn from_index(index: usize) -> Self {
        debug_assert!(index < Self::N_ALPHABET);
        index as u8 + 0x41
    }
}

#[cfg(test)]
mod tests {
    use super::Alphabetic;

    #[test]
    fn alphabetic_char() {
        for (i, c) in ('A'..='Z').enumerate() {
            assert_eq!(c.index(), i);
            assert_eq!(<char as Alphabetic>::from_index(i), c);
        }
    }

    #[test]
    fn alphabetic_u8() {
        for (i, c) in (0x41..=0x5A).enumerate() {
            assert_eq!(c.index(), i);
            assert_eq!(<u8 as Alphabetic>::from_index(i), c);
        }
    }
}
