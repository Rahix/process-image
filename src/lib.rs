#![cfg_attr(not(test), no_std)]

mod access;
pub use access::{BitMut, WordMut, DWordMut, LWordMut};

#[macro_export]
macro_rules! tag {
    ($buf:expr, X, $addr1:literal, $addr2:literal) => {{
        let buffer: &[u8] = $buf;
        buffer[$addr1] & (1 << $addr2) != 0
    }};
    ($buf:expr, B, $addr:literal) => {{
        let buffer: &[u8] = $buf;
        buffer[$addr]
    }};
    ($buf:expr, W, $addr:literal) => {{
        let buffer: &[u8] = $buf;
        assert!($addr % 2 == 0, "Word address must be divisible by 2");
        u16::from_le_bytes(buffer[$addr..$addr+2].try_into().unwrap())
    }};
    ($buf:expr, D, $addr:literal) => {{
        let buffer: &[u8] = $buf;
        assert!($addr % 4 == 0, "Double word address must be divisible by 4");
        u32::from_le_bytes(buffer[$addr..$addr+4].try_into().unwrap())
    }};
    ($buf:expr, L, $addr:literal) => {{
        let buffer: &[u8] = $buf;
        assert!($addr % 8 == 0, "Long word address must be divisible by 8");
        u64::from_le_bytes(buffer[$addr..$addr+8].try_into().unwrap())
    }};
    ($buf:expr, $addr1:literal, $addr2:literal) => {{
        let buffer: &[u8] = $buf;
        buffer[$addr1] & (1 << $addr2) != 0
    }};
}

#[macro_export]
macro_rules! tag_mut {
    ($buf:expr, X, $addr1:literal, $addr2:literal) => {{
        let buffer: &mut [u8] = $buf;
        $crate::BitMut::new(&mut buffer[$addr1], $addr2)
    }};
    ($buf:expr, B, $addr:literal) => {{
        let buffer: &mut [u8] = $buf;
        &mut buffer[$addr]
    }};
    ($buf:expr, W, $addr:literal) => {{
        let buffer: &mut [u8] = $buf;
        assert!($addr % 2 == 0, "Word address must be divisible by 2");
        $crate::WordMut::new((&mut buffer[$addr..$addr+2]).try_into().unwrap())
    }};
    ($buf:expr, D, $addr:literal) => {{
        let buffer: &mut [u8] = $buf;
        assert!($addr % 4 == 0, "Double word address must be divisible by 4");
        $crate::DWordMut::new((&mut buffer[$addr..$addr+4]).try_into().unwrap())
    }};
    ($buf:expr, L, $addr:literal) => {{
        let buffer: &mut [u8] = $buf;
        assert!($addr % 8 == 0, "Long word address must be divisible by 8");
        $crate::LWordMut::new((&mut buffer[$addr..$addr+8]).try_into().unwrap())
    }};
}


#[cfg(test)]
mod tests {
    #[test]
    fn tag_macro_smoke1() {
        let mut pi = [0x55, 0xaa, 0x00, 0xff];

        assert_eq!(tag!(&pi, X, 2, 0), false);
        assert_eq!(tag!(&pi, 3, 0), true);
        assert_eq!(tag!(&pi, X, 0, 0), true);
        assert_eq!(tag!(&pi, 0, 1), false);

        *tag_mut!(&mut pi, X, 0, 0) = false;
        assert_eq!(tag!(&pi, X, 0, 0), false);

        assert_eq!(tag!(&pi, B, 2), 0x00);
        *tag_mut!(&mut pi, X, 2, 7) = true;
        assert_eq!(tag!(&pi, B, 2), 0x80);

        assert_eq!(tag!(&pi, W, 2), 0xff80);
        assert_eq!(*tag_mut!(&mut pi, W, 2), 0xff80);
        assert_eq!(tag!(&pi, D, 0), 0xff80aa54);
        assert_eq!(*tag_mut!(&mut pi, D, 0), 0xff80aa54);

        *tag_mut!(&mut pi, W, 2) = 0xbeef;
        assert_eq!(tag!(&pi, W, 2), 0xbeef);
    }
}
