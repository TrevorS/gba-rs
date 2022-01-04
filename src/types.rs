pub type Byte = u8;
pub type Word = u16;

#[inline]
pub fn to_word(b1: Byte, b2: Byte) -> Word {
    (b1 as Word) << 8 | b2 as Word
}

#[inline]
pub fn from_word(w: Word) -> (Byte, Byte) {
    ((w >> 8) as Byte, (w & 0xff) as Byte)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_word() {
        assert_eq!(258, to_word(1, 2));
    }

    #[test]
    fn test_from_word() {
        assert_eq!((1, 2), from_word(258));
    }
}
