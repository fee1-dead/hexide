#![no_std]
#![forbid(unsafe_op_in_unsafe_fn)]
#![feature(new_uninit)]
#![feature(maybe_uninit_slice)]

extern crate alloc;

pub(crate) mod impl_;
mod iter;

use alloc::boxed::Box;
pub use iter::HexEncoder;

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

pub fn encode(bytes: impl AsRef<[u8]>) -> Vec<u8> {
    encode_boxed_slice(bytes).into()
}

#[inline]
pub fn encode_boxed_slice(bytes: impl AsRef<[u8]>) -> Box<[u8]> {
    encode_(bytes.as_ref())
}

#[inline]
pub fn encode_str(bytes: impl AsRef<[u8]>) -> String {
    // SAFETY: we only encode valid ASCII bytes so it is always valid UTF-8.
    unsafe { String::from_utf8_unchecked(encode(bytes)) }
}

pub fn encode_iter(bytes: impl IntoIterator<Item = u8>) -> Vec<u8> {
    HexEncoder::new(bytes).collect()
}

fn encode_(bytes: &[u8]) -> Box<[u8]> {
    #[cfg(feature = "nightly")]
    let mut slice = Box::new_uninit_slice(bytes.len() * 2);

    #[cfg(not(feature = "nightly"))]
    let mut slice = vec![0; bytes.len() * 2].into_boxed_slice();

    // SAFETY: the vector just allocated enough capacity for us,
    // and the pointers are valid for writes.
    unsafe {
        impl_::encode(bytes, &mut slice);
    }

    #[cfg(feature = "nightly")]
    // SAFETY: we just wrote everything into the box.
    unsafe {
        slice.assume_init()
    }

    #[cfg(not(feature = "nightly"))]
    slice
}

#[cfg(test)]
mod tests {
    use crate::encode_str;

    #[test]
    fn test_encode() {
        assert_eq!("01", encode_str([0x01]));
        assert_eq!(
            "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20",
            encode_str([
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0x10, 0x11, 0x12, 0x13, 0x14,
                0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F, 0x20
            ])
        );
    }
}
