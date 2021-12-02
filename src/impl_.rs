use core::num::NonZeroU8;

#[cfg(feature = "nightly")]
pub type Byte = core::mem::MaybeUninit<u8>;

#[cfg(not(feature = "nightly"))]
pub type Byte = u8;

#[inline(always)]
fn b(b: u8) -> Byte {
    #[cfg(feature = "nightly")]
    {
        core::mem::MaybeUninit::new(b)
    }

    #[cfg(not(feature = "nightly"))]
    {
        b
    }
}

/// # Safety
///
/// to_start must point to a location that is valid for writes. All pointers from `to_start`
/// up until (not including) `to_end` must also be valid for writes.
pub unsafe fn encode(src: &[u8], to: &mut [Byte]) {
    assert!(to.len() >= (src.len() * 2));

    for (from, to) in src.iter().zip(to.chunks_exact_mut(2)) {
        let (hi, lo) = one_byte_lower(*from);

        to[0] = b(hi.get());
        to[1] = b(lo.get());
    }
}

static LOWER: &[u8] = b"0123456789abcdef";

/// # Safety
///
/// `b` must have at least four leading zeros.
#[inline]
unsafe fn four_bits_lower(b: u8) -> NonZeroU8 {
    // SAFETY: `LOWER`'s valid index range is 0..16,
    // which is the valid range for four bit integers.
    //
    // No ASCII byte in LOWER is zero.
    unsafe {
        let byte = *LOWER.get_unchecked(b as usize);

        NonZeroU8::new_unchecked(byte)
    }
}

#[inline]
pub fn one_byte_lower(b: u8) -> (NonZeroU8, NonZeroU8) {
    // SAFETY: `b >> 4` creates four leading zeros and
    // the `0b1111` bitmask creates four leading zeros.
    unsafe { (four_bits_lower(b >> 4), four_bits_lower(b & 0b1111)) }
}
