use core::fmt::{Display, Formatter};

use Leb128Error::IncompleteEncoding;

use crate::leb128::Leb128Error::InvalidEncoding;

#[cfg_attr(test, derive(Debug))]
#[derive(PartialEq)]
pub enum Leb128Error {
    InvalidEncoding,
    IncompleteEncoding,
}

impl Display for Leb128Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Leb128Error::InvalidEncoding => write!(f, "Invalid leb128 encoding"),
            IncompleteEncoding => write!(f, "Incomplete leb128 encoding")
        }
    }
}

const CONTINUATION_BIT: u8 = 1 << 7;
const SIGN_BIT: u8 = 1 << 6;

#[inline(always)]
fn low_bits_of_byte(byte: u8) -> u8 {
    byte & !CONTINUATION_BIT
}


type Result<T> = ::core::result::Result<T, Leb128Error>;

pub trait Leb128: Sized {
    fn read_leb128(bytes: impl AsRef<[u8]>) -> Result<(Self, usize)>;
}

impl Leb128 for u32 {
    fn read_leb128(bytes: impl AsRef<[u8]>) -> Result<(Self, usize)> {
        let mut result = 0u32;
        let mut shift = 0;

        for (idx, byte) in bytes.as_ref().iter().clone().enumerate() {
            // 2nd byte onwards can never be 0x00
            if *byte == 0x00 && idx > 0 {
                return Err(InvalidEncoding);
            }
            // Add the lower 7 bits of the byte to the result
            result |= (low_bits_of_byte(*byte) as u32) << shift;
            // If the most significant bit (MSB) is not set, we are done
            if byte & CONTINUATION_BIT == 0 {
                return Ok((result, idx + 1));
            }
            // If shift is 28 or more, we've read too many bytes for an u32
            if shift >= 28 {
                return Err(InvalidEncoding);
            }
            shift += 7;
        }

        Err(IncompleteEncoding)
    }
}

impl Leb128 for i32 {
    fn read_leb128(bytes: impl AsRef<[u8]>) -> Result<(Self, usize)> {
        let mut result = 0i32;
        let mut shift = 0;

        for (idx, byte) in bytes.as_ref().iter().clone().enumerate() {
            // 2nd byte onwards can never be 0x00
            if *byte == 0x00 && idx > 0 {
                return Err(InvalidEncoding);
            }

            result |= i32::from(low_bits_of_byte(*byte)) << shift;
            shift += 7;

            // If the high-order bit is not set, this is the last byte
            if byte & CONTINUATION_BIT == 0 {
                // If this was a signed value and the sign bit is set in the final byte
                if shift < 32 && (SIGN_BIT & byte) == SIGN_BIT {
                    result |= !0 << shift;
                }

                return Ok((result as i32, idx + 1));
            }

            // If we exceed the maximum shift for a 32-bit integer, return an error
            if shift >= 32 {
                return Err(InvalidEncoding);
            }
        }

        Err(IncompleteEncoding)
    }
}

#[cfg(test)]
mod tests {
    use alloc::{format, vec};

    use crate::leb128::Leb128Error::{IncompleteEncoding, InvalidEncoding};

    use super::*;

    #[test]
    fn u32_ok() {
        for (given, expected, expected_consumption) in [
            (vec![0x00], 0, 1),
            (vec![0x00, 0x00, 0x00, 0x01], 0, 1),
            (vec![0x01], 1, 1),
            (vec![0x7F], 127, 1),
            (vec![0xFF, 0x01], 255, 2),
            (vec![0x80, 0x01], 128, 2),
            (vec![0x80, 0x01, 0x00, 0x00], 128, 2),
            (vec![0xE5, 0x8E, 0x26], 624485, 3),
            (vec![0xFF, 0xFF, 0xFF, 0xFF, 0x0F], u32::MAX, 5),
            (vec![0xFF, 0xFF, 0xFF, 0xFF, 0x0F, 0x01], u32::MAX, 5),
            (vec![0x80, 0x80, 0x80, 0x80, 0x01], 268435456, 5), // Minimum 5-byte value
            (vec![0x81, 0x01], 129, 2), // Simple multi-byte encoding
            (vec![0x01, 0x80], 1, 1), // Should only consume the first byte
            (vec![0xC0, 0xBB, 0x78], 1973696, 3), // Encoded with non-trivial continuation
            (vec![0xE5, 0x8E, 0x26, 0x80], 624485, 3), // Should consume only necessary bytes
            (vec![0x01, 0x01], 1, 1), // Non-canonical encoding (second byte ignored)
            (vec![0x00, 0x80, 0x00], 0, 1), // Leading zeros with continuation bit
        ] {
            let (result, consumed) = u32::read_leb128(&given).unwrap();
            assert_eq!(result, expected, "expected {} but got {} for {:#04X?}", expected, result, given);
            assert_eq!(consumed, expected_consumption, "expected to consume {} but got {} for {:#04X?}", expected_consumption, consumed, given);
        }
    }

    #[test]
    fn u32_invalid() {
        for (given, expected) in [
            (vec![0xFF], IncompleteEncoding), // Incomplete with one byte
            (vec![0x80], IncompleteEncoding), // missing bytes
            (vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF], InvalidEncoding), // Too many bytes for a valid u32
            (vec![0x80, 0x80], IncompleteEncoding), // Missing continuation for multi-byte
            (vec![0x80, 0x80, 0x80, 0x80, 0x80], InvalidEncoding), // More than 5 bytes
            (vec![0xFF, 0x00, 0x00, 0x00, 0x00], InvalidEncoding), // Improper use of continuation bits
        ] {
            let result = u32::read_leb128(&given);
            assert_eq!(result, Err(expected))
        }
    }


    #[test]
    fn i32_ok() {
        for (given, expected, expected_consumption) in [
            (vec![0x00], 0, 1),
            (vec![0x01], 1, 1),
            (vec![0x7F], -1, 1),
            (vec![0x00], 0, 1),
            (vec![0x80, 0x01], 128, 2),
            (vec![0xc0, 0xc4, 0x07], 123456, 3),
            (vec![0xff, 0xff, 0xff, 0xff, 0x07], i32::MAX, 5),
            (vec![0xc0, 0xbb, 0x78], -123456, 3),
            (vec![0x7f], -1, 1),
            (vec![0x80, 0x7f], -128, 2),
            (vec![0x80, 0x80, 0x80, 0x80, 0x78], i32::MIN, 5),
        ] {
            let (result, consumed) = i32::read_leb128(&given).expect(format!(" {:#04X?}", given).as_ref());
            assert_eq!(result, expected, "expected {} but got {} for {:#04X?}", expected, result, given);
            assert_eq!(consumed, expected_consumption, "expected to consume {} but got {} for {:#04X?}", expected_consumption, consumed, given);
        }
    }

    #[test]
    fn i32_invalid() {
        for (given, expected) in [
            (vec![0x80], IncompleteEncoding), // Incomplete sequence with one byte
            (vec![0x80, 0x80], IncompleteEncoding), // Missing continuation for multi-byte sequence
            (vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF], InvalidEncoding), // Too many bytes for a valid i32
            (vec![0x80, 0x80, 0x80, 0x80, 0x80], InvalidEncoding), // More than 5 bytes, which is invalid for i32
            (vec![0xFF, 0x00, 0x00, 0x00, 0x00], InvalidEncoding), // Improper use of continuation bits
        ] {
            let result = i32::read_leb128(&given);
            assert_eq!(result, Err(expected), "{:#04X?}", given)
        }
    }
}
