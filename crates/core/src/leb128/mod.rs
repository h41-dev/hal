use core::fmt::{Display, Formatter};

#[cfg_attr(test, derive(Debug))]
#[derive(PartialEq)]
pub enum Leb128EncodingError {
    InvalidEncoding,
    IncompleteEncoding,
}

impl Display for Leb128EncodingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Leb128EncodingError::InvalidEncoding => write!(f, "Invalid leb128 encoding"),
            Leb128EncodingError::IncompleteEncoding => write!(f, "Incomplete leb128 encoding")
        }
    }
}

type Result<T> = ::core::result::Result<T, Leb128EncodingError>;

pub trait Leb128: Sized {
    fn read_leb128(bytes: impl AsRef<[u8]>) -> Result<(Self, usize)>;
}

impl Leb128 for u32 {
    fn read_leb128(bytes: impl AsRef<[u8]>) -> Result<(Self, usize)> {
        let mut result = 0u64;
        let mut shift = 0;

        for (idx, byte) in bytes.as_ref().iter().clone().enumerate() {
            if idx > 0 {
                // Following byte can never b 0x00
                if *byte == 0x00 {
                    return Err(Leb128EncodingError::InvalidEncoding);
                }
            }
            // Add the lower 7 bits of the byte to the result
            result |= ((byte & 0x7F) as u64) << shift;
            // If the most significant bit (MSB) is not set, we are done
            if byte & 0x80 == 0 {
                // Do not accept values bigger than u32::MAX
                if result > u32::MAX as u64 {
                    return Err(Leb128EncodingError::InvalidEncoding);
                }
                return Ok((result as u32, idx + 1));
            }
            // If shift is 28 or more, we've read too many bytes for an u32
            if shift >= 28 {
                return Err(Leb128EncodingError::InvalidEncoding);
            }
            shift += 7;
        }

        Err(Leb128EncodingError::IncompleteEncoding)
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;
    use crate::leb128::Leb128EncodingError::{IncompleteEncoding, InvalidEncoding};
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
            (vec![0xFF, 0xFF, 0xFF, 0xFF, 0x1F], InvalidEncoding), // Overflow in 5th byte
            (vec![0xFF, 0x00, 0x00, 0x00, 0x00], InvalidEncoding), // Improper use of continuation bits
        ] {
            let result = u32::read_leb128(&given);
            assert_eq!(result, Err(expected))
        }
    }

    #[test]
    fn u32_edge_cases() {
        for (given, expected, expected_consumption) in [
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


    // // (vec![0x80, 0x80, 0x80, 0x80, 0x01], 268435456, 5), // Minimum 5-byte value
    // // (vec![0x80, 0x80, 0x80, 0x80, 0x10], 2147483648, 5), // Mid 5-byte value (exact middle of u32 range)
    // (vec![0x81, 0x01], 129, 2), // Simple multi-byte encoding
    // (vec![0x01, 0x80], 1, 1), // Should only consume the first byte
    // // (vec![0xC0, 0xBB, 0x78], 129984, 3), // Encoded with non-trivial continuation
    // (vec![0xE5, 0x8E, 0x26, 0x80], 624485, 3), // Should consume only necessary bytes
    // (vec![0x01, 0x01], 1, 1), // Non-canonical encoding (second byte ignored)
    // (vec![0x00, 0x80, 0x00], 0, 1), // Leading zeros with continuation bit
}
