use core::ops::Deref;
use core::ops::DerefMut;

/// Mutable accessor for a single bit.
///
/// This type dereferences to an `&mut bool` which can be used to write the value of a single bit
/// in the process image.
#[derive(Debug)]
pub struct BitMut<'a> {
    buf: &'a mut u8,
    index: u8,
    value: bool,
}

impl<'a> BitMut<'a> {
    #[inline(always)]
    pub fn new(buf: &'a mut u8, index: u8) -> Self {
        let value = *buf & (1 << index) != 0;
        Self { buf, index, value }
    }
}

impl Deref for BitMut<'_> {
    type Target = bool;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for BitMut<'_> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl Drop for BitMut<'_> {
    #[inline(always)]
    fn drop(&mut self) {
        *self.buf &= !(1 << self.index);
        *self.buf |= u8::from(self.value) << self.index;
    }
}

/// Mutable accessor for a word.
///
/// This type dereferences to an `&mut u16` which can be used to write the value of a word in the
/// process image.
#[derive(Debug)]
pub struct WordMut<'a> {
    buf: &'a mut [u8; 2],
    value: u16,
}

impl<'a> WordMut<'a> {
    #[inline(always)]
    pub fn new(buf: &'a mut [u8; 2]) -> Self {
        let value = u16::from_le_bytes(*buf);
        Self { buf, value }
    }
}

impl Deref for WordMut<'_> {
    type Target = u16;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for WordMut<'_> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl Drop for WordMut<'_> {
    #[inline(always)]
    fn drop(&mut self) {
        *self.buf = self.value.to_le_bytes();
    }
}

/// Mutable accessor for a double word.
///
/// This type dereferences to an `&mut u32` which can be used to write the value of a double word
/// in the process image.
#[derive(Debug)]
pub struct DWordMut<'a> {
    buf: &'a mut [u8; 4],
    value: u32,
}

impl<'a> DWordMut<'a> {
    #[inline(always)]
    pub fn new(buf: &'a mut [u8; 4]) -> Self {
        let value = u32::from_le_bytes(*buf);
        Self { buf, value }
    }
}

impl Deref for DWordMut<'_> {
    type Target = u32;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for DWordMut<'_> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl Drop for DWordMut<'_> {
    #[inline(always)]
    fn drop(&mut self) {
        *self.buf = self.value.to_le_bytes();
    }
}

/// Mutable accessor for a long word.
///
/// This type dereferences to an `&mut u64` which can be used to write the value of a long word in
/// the process image.
#[derive(Debug)]
pub struct LWordMut<'a> {
    buf: &'a mut [u8; 8],
    value: u64,
}

impl<'a> LWordMut<'a> {
    #[inline(always)]
    pub fn new(buf: &'a mut [u8; 8]) -> Self {
        let value = u64::from_le_bytes(*buf);
        Self { buf, value }
    }
}

impl Deref for LWordMut<'_> {
    type Target = u64;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for LWordMut<'_> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl Drop for LWordMut<'_> {
    #[inline(always)]
    fn drop(&mut self) {
        *self.buf = self.value.to_le_bytes();
    }
}
