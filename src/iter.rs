use core::iter::FusedIterator;
use core::num::NonZeroU8;

use crate::impl_::one_byte_lower;

/// Encode an iterator of bytes as ASCII hex bytes.
pub struct HexEncoder<I> {
    inner: I,
    extra: Option<NonZeroU8>,
}

impl<I: Iterator<Item = u8>> HexEncoder<I> {
    pub fn new(iterator: impl IntoIterator<IntoIter = I>) -> Self {
        Self {
            inner: iterator.into_iter(),
            extra: None,
        }
    }
}

impl<I: Iterator<Item = u8>> Iterator for HexEncoder<I> {
    type Item = u8;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let base = self.extra.is_some() as usize;

        let (lower, upper) = self.inner.size_hint();

        (lower * 2 + base, upper.map(|n| n * 2 + base))
    }

    fn count(self) -> usize {
        self.inner.count() * 2 + self.extra.is_some() as usize
    }

    fn next(&mut self) -> Option<u8> {
        if let Some(extra) = self.extra {
            return Some(extra.get());
        }

        match self.inner.next() {
            Some(byte) => {
                let (first, second) = one_byte_lower(byte);

                self.extra = Some(second);

                Some(first.get())
            }
            None => None,
        }
    }
}

impl<I: ExactSizeIterator<Item = u8>> ExactSizeIterator for HexEncoder<I> {}
impl<I: FusedIterator<Item = u8>> FusedIterator for HexEncoder<I> {}
