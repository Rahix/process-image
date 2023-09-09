use core::ops::Deref;
use core::ops::DerefMut;

#[derive(Debug)]
pub struct BitMut<'a> {
    buf: &'a mut u8,
    index: u8,
    value: bool,
}

impl<'a> BitMut<'a> {
    pub fn new(buf: &'a mut u8, index: u8) -> Self {
        let value = *buf & (1 << index) != 0;
        Self { buf, index, value }
    }
}

impl Deref for BitMut<'_> {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for BitMut<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl Drop for BitMut<'_> {
    fn drop(&mut self) {
        *self.buf &= !(1 << self.index);
        *self.buf |= u8::from(self.value) << self.index;
    }
}

#[derive(Debug)]
pub struct WordMut<'a> {
    buf: &'a mut [u8; 2],
    value: u16,
}

impl<'a> WordMut<'a> {
    pub fn new(buf: &'a mut [u8; 2]) -> Self {
        let value = u16::from_le_bytes(*buf);
        Self { buf, value }
    }
}

impl Deref for WordMut<'_> {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for WordMut<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl Drop for WordMut<'_> {
    fn drop(&mut self) {
        *self.buf = self.value.to_le_bytes();
    }
}

#[derive(Debug)]
pub struct DWordMut<'a> {
    buf: &'a mut [u8; 4],
    value: u32,
}

impl<'a> DWordMut<'a> {
    pub fn new(buf: &'a mut [u8; 4]) -> Self {
        let value = u32::from_le_bytes(*buf);
        Self { buf, value }
    }
}

impl Deref for DWordMut<'_> {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for DWordMut<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl Drop for DWordMut<'_> {
    fn drop(&mut self) {
        *self.buf = self.value.to_le_bytes();
    }
}

#[derive(Debug)]
pub struct LWordMut<'a> {
    buf: &'a mut [u8; 8],
    value: u64,
}

impl<'a> LWordMut<'a> {
    pub fn new(buf: &'a mut [u8; 8]) -> Self {
        let value = u64::from_le_bytes(*buf);
        Self { buf, value }
    }
}

impl Deref for LWordMut<'_> {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for LWordMut<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl Drop for LWordMut<'_> {
    fn drop(&mut self) {
        *self.buf = self.value.to_le_bytes();
    }
}
