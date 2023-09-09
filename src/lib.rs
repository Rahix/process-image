#![cfg_attr(not(test), no_std)]

mod access;
pub use access::{BitMut, DWordMut, LWordMut, WordMut};

/// Read tag values from a process image with absolute addressing.
///
/// Addresses must be aligned to the size of the datatype (i.e. word=2, dword=4, lword=8).
///
/// Multi-byte datatypes are always accessed in big-endian order.
///
/// # Example
/// ```
/// let pi = [0x00; 16];
///
/// // Bit access
/// let b1: bool = process_image::tag!(&pi, X, 0, 0);   // %MX0.0
/// let b2: bool = process_image::tag!(&pi, 0, 1);      // %MX0.1
///
/// // Byte access
/// let by: u8 = process_image::tag!(&pi, B, 1);        // %MB1
///
/// // Word access
/// let w: u16 = process_image::tag!(&pi, W, 2);        // %MW2
///
/// // Double word access
/// let d: u32 = process_image::tag!(&pi, D, 4);        // %MD4
///
/// // Long word access
/// let l: u64 = process_image::tag!(&pi, L, 8);        // %ML8
/// ```
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
        u16::from_be_bytes(buffer[$addr..$addr + 2].try_into().unwrap())
    }};
    ($buf:expr, D, $addr:literal) => {{
        let buffer: &[u8] = $buf;
        assert!($addr % 4 == 0, "Double word address must be divisible by 4");
        u32::from_be_bytes(buffer[$addr..$addr + 4].try_into().unwrap())
    }};
    ($buf:expr, L, $addr:literal) => {{
        let buffer: &[u8] = $buf;
        assert!($addr % 8 == 0, "Long word address must be divisible by 8");
        u64::from_be_bytes(buffer[$addr..$addr + 8].try_into().unwrap())
    }};
    ($buf:expr, $addr1:literal, $addr2:literal) => {{
        let buffer: &[u8] = $buf;
        buffer[$addr1] & (1 << $addr2) != 0
    }};
}

/// Mutable access to tag values from a process image with absolute addressing.
///
/// Addresses must be aligned to the size of the datatype (i.e. word=2, dword=4, lword=8).
///
/// Multi-byte datatypes are always accessed in big-endian order.
///
/// # Example
/// ```
/// let mut pi = [0x00; 16];
///
/// // Bit access
/// *process_image::tag_mut!(&mut pi, X, 0, 0) = true;  // %MX0.0
/// *process_image::tag_mut!(&mut pi, 0, 1) = true;     // %MX0.1
///
/// // Byte access
/// *process_image::tag_mut!(&mut pi, B, 1) = 42u8;     // %MB1
///
/// // Word access
/// *process_image::tag_mut!(&mut pi, W, 2) = 1337u16;  // %MW2
///
/// // Double word access
/// *process_image::tag_mut!(&mut pi, D, 4) = 0xdeadbeef; // %MD4
///
/// // Long word access
/// *process_image::tag_mut!(&mut pi, L, 8) = 1;        // %ML8
/// ```
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
        $crate::WordMut::new((&mut buffer[$addr..$addr + 2]).try_into().unwrap())
    }};
    ($buf:expr, D, $addr:literal) => {{
        let buffer: &mut [u8] = $buf;
        assert!($addr % 4 == 0, "Double word address must be divisible by 4");
        $crate::DWordMut::new((&mut buffer[$addr..$addr + 4]).try_into().unwrap())
    }};
    ($buf:expr, L, $addr:literal) => {{
        let buffer: &mut [u8] = $buf;
        assert!($addr % 8 == 0, "Long word address must be divisible by 8");
        $crate::LWordMut::new((&mut buffer[$addr..$addr + 8]).try_into().unwrap())
    }};
    ($buf:expr, $addr1:literal, $addr2:literal) => {{
        let buffer: &mut [u8] = $buf;
        $crate::BitMut::new(&mut buffer[$addr1], $addr2)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! tag_method {
    ($vis:vis, $name:ident, mut, X, $addr1:literal, $addr2:literal) => {
        #[inline(always)]
        $vis fn $name(&mut self) -> $crate::BitMut<'_> {
            $crate::BitMut::new(&mut self.buf[$addr1], $addr2)
        }
    };
    ($vis:vis, $name:ident, mut, B, $addr:literal) => {
        #[inline(always)]
        $vis fn $name(&mut self) -> &mut u8 {
            &mut self.buf[$addr]
        }
    };
    ($vis:vis, $name:ident, mut, W, $addr:literal) => {
        #[inline(always)]
        $vis fn $name(&mut self) -> $crate::WordMut<'_> {
            assert!($addr % 2 == 0, "Word address must be divisible by 2");
            $crate::WordMut::new((&mut self.buf[$addr..$addr + 2]).try_into().unwrap())
        }
    };
    ($vis:vis, $name:ident, mut, D, $addr:literal) => {
        #[inline(always)]
        $vis fn $name(&mut self) -> $crate::DWordMut<'_> {
            assert!($addr % 4 == 0, "Double word address must be divisible by 4");
            $crate::DWordMut::new((&mut self.buf[$addr..$addr + 4]).try_into().unwrap())
        }
    };
    ($vis:vis, $name:ident, mut, L, $addr:literal) => {
        #[inline(always)]
        $vis fn $name(&mut self) -> $crate::LWordMut<'_> {
            assert!($addr % 8 == 0, "Long word address must be divisible by 8");
            $crate::LWordMut::new((&mut self.buf[$addr..$addr + 8]).try_into().unwrap())
        }
    };
    ($vis:vis, $name:ident, mut, $addr1:literal, $addr2:literal) => {
        #[inline(always)]
        $vis fn $name(&mut self) -> $crate::BitMut<'_> {
            $crate::BitMut::new(&mut self.buf[$addr1], $addr2)
        }
    };
    ($vis:vis, $name:ident, const, X, $addr1:literal, $addr2:literal) => {
        #[inline(always)]
        $vis fn $name(&self) -> bool {
            self.buf[$addr1] & (1 << $addr2) != 0
        }
    };
    ($vis:vis, $name:ident, const, B, $addr:literal) => {
        #[inline(always)]
        $vis fn $name(&self) -> u8 {
            self.buf[$addr]
        }
    };
    ($vis:vis, $name:ident, const, W, $addr:literal) => {
        #[inline(always)]
        $vis fn $name(&self) -> u16 {
            assert!($addr % 2 == 0, "Word address must be divisible by 2");
            u16::from_be_bytes(self.buf[$addr..$addr + 2].try_into().unwrap())
        }
    };
    ($vis:vis, $name:ident, const, D, $addr:literal) => {
        #[inline(always)]
        $vis fn $name(&self) -> u32 {
            assert!($addr % 4 == 0, "Double word address must be divisible by 4");
            u16::from_be_bytes(self.buf[$addr..$addr + 4].try_into().unwrap())
        }
    };
    ($vis:vis, $name:ident, const, L, $addr:literal) => {
        #[inline(always)]
        $vis fn $name(&self) -> u64 {
            assert!($addr % 8 == 0, "Long word address must be divisible by 8");
            u64::from_be_bytes(self.buf[$addr..$addr + 8].try_into().unwrap())
        }
    };
    ($vis:vis, $name:ident, const, $addr1:literal, $addr2:literal) => {
        #[inline(always)]
        $vis fn $name(&self) -> bool {
            self.buf[$addr1] & (1 << $addr2) != 0
        }
    };
}

#[macro_export]
macro_rules! process_image {
    (
        $( #[$meta:meta] )*
        $vis:vis struct $ProcessImage:ident, mut $ProcessImageMut:ident: $SIZE:literal {
            $(
                $( #[$field_meta:meta] )*
                $field_vis:vis $field_name:ident: ($($tag:tt)+)
            ),*
            $(,)?
        }
    ) => {
        $( #[$meta] )*
        $vis struct $ProcessImage<'a> {
            buf: &'a [u8; $SIZE],
        }

        impl<'a> $ProcessImage<'a> {
            $(
                $( #[$field_meta] )*
                $crate::tag_method!($vis, $field_name, const, $($tag)+);
            )*
        }

        impl<'a> From<&'a [u8; $SIZE]> for $ProcessImage<'a> {
            #[inline(always)]
            fn from(buf: &'a [u8; $SIZE]) -> Self {
                Self { buf }
            }
        }

        impl<'a> TryFrom<&'a [u8]> for $ProcessImage<'a> {
            type Error = ::core::array::TryFromSliceError;

            #[inline(always)]
            fn try_from(buf: &'a [u8]) -> Result<Self, Self::Error> {
                buf.try_into().map(|buf| Self { buf })
            }
        }

        $( #[$meta] )*
        $vis struct $ProcessImageMut<'a> {
            buf: &'a mut [u8; $SIZE],
        }

        impl<'a> From<&'a mut [u8; $SIZE]> for $ProcessImageMut<'a> {
            #[inline(always)]
            fn from(buf: &'a mut [u8; $SIZE]) -> Self {
                Self { buf }
            }
        }

        impl<'a> TryFrom<&'a mut [u8]> for $ProcessImageMut<'a> {
            type Error = ::core::array::TryFromSliceError;

            #[inline(always)]
            fn try_from(buf: &'a mut [u8]) -> Result<Self, Self::Error> {
                buf.try_into().map(|buf| Self { buf })
            }
        }

        impl<'a> $ProcessImageMut<'a> {
            $(
                $( #[$field_meta] )*
                $crate::tag_method!($vis, $field_name, mut, $($tag)+);
            )*
        }
    };
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

        assert_eq!(tag!(&pi, W, 2), 0x80ff);
        assert_eq!(*tag_mut!(&mut pi, W, 2), 0x80ff);
        assert_eq!(tag!(&pi, D, 0), 0x54aa80ff);
        assert_eq!(*tag_mut!(&mut pi, D, 0), 0x54aa80ff);

        *tag_mut!(&mut pi, W, 2) = 0xbeef;
        assert_eq!(tag!(&pi, W, 2), 0xbeef);
    }

    process_image! {
        pub struct TestPi, mut TestPiMut: 4 {
            pub btn_start: (X, 1, 0),
            pub btn_stop: (1, 1),
            pub btn_reset: (X, 1, 2),
            pub speed: (W, 2),
            pub length: (B, 0),
        }
    }

    #[test]
    fn pi_macro_smoke1() {
        let mut pi_buffer = [128, 0x55, 0xde, 0xad];

        let pi = TestPi::try_from(&pi_buffer).unwrap();
        assert_eq!(pi.btn_start(), true);
        assert_eq!(pi.btn_stop(), false);
        assert_eq!(pi.btn_reset(), true);
        assert_eq!(pi.speed(), 0xdead);
        assert_eq!(pi.length(), 128);

        let mut pi = TestPiMut::try_from(&mut pi_buffer).unwrap();
        assert_eq!(*pi.btn_start(), true);
        assert_eq!(*pi.btn_stop(), false);
        assert_eq!(*pi.btn_reset(), true);
        assert_eq!(*pi.speed(), 0xdead);
        assert_eq!(*pi.length(), 128);

        *pi.btn_start() = false;
        *pi.btn_stop() = true;

        *pi.speed() = 1337;
        *pi.length() = 1;

        let pi = TestPi::try_from(&pi_buffer).unwrap();
        assert_eq!(pi.btn_start(), false);
        assert_eq!(pi.btn_stop(), true);
        assert_eq!(pi.btn_reset(), true);
        assert_eq!(pi.speed(), 1337);
        assert_eq!(pi.length(), 1);

        assert_eq!(tag!(&pi_buffer, 1, 0), false);
        assert_eq!(tag!(&pi_buffer, W, 2), 1337);
        assert_eq!(tag!(&pi_buffer, B, 0), 1);
    }
}
