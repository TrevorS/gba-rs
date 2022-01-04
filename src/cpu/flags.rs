pub struct Flags {
    zero: bool,
    sub: bool,
    half: bool,
    carry: bool,
    ime: bool,
    ime_pending: bool,
}

impl Default for Flags {
    fn default() -> Self {
        Self {
            zero: true,
            sub: false,
            half: true,
            carry: true,
            ime: false,
            ime_pending: false,
        }
    }
}
