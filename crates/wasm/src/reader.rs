use alloc::boxed::Box;
use core::cell::RefCell;

use WasmParseError::UnexpectedEndOfFile;

use crate::error::WasmParseError;
use crate::error::WasmParseError::{InvalidLEB128Encoding, OutOfBounds};
use crate::Result;

/// A `ByteReader` provides a simple mechanism for reading bytes sequentially
/// from a data source in memory. This struct is useful for scenarios where you
/// need to manually manage the position in a byte stream, such as when implementing
/// custom binary file parsers or decoding structured data from a raw byte buffer.
///
/// The `ByteReader` is generic over any data source that implements the `AsRef<[u8]>`
/// trait, which allows it to work with a variety of data types, including `Vec<u8>` and
/// byte slices (`&[u8]`). Internally, it keeps track of the current read position and
/// provides methods to read individual bytes and other primitive types from the data.
///
/// # Example
///
/// ```
/// use hal_wasm::reader::ByteReader;
/// let data: [u8;3] = [0x01, 0x02, 0x03];
/// let mut reader = ByteReader::new(data.as_ref());
///
/// assert_eq!(reader.read_u8().unwrap(), 0x01);
/// assert_eq!(reader.read_u8().unwrap(), 0x02);
/// assert_eq!(reader.read_u8().unwrap(), 0x03);
/// assert!(reader.read_u8().is_err()); // Out of bounds
/// ```
///
/// This struct does not perform any I/O operations and is designed to work entirely
/// with in-memory data.
pub struct ByteReader<'a> {
    data: &'a [u8],
    pos: RefCell<usize>,
}

impl<'a> ByteReader<'a> {
    /// Returns the total length of the data being read.
    ///
    /// # Returns
    ///
    /// A `usize` representing the total number of bytes in the data.
    ///
    fn length(&self) -> usize {
        self.data.as_ref().len()
    }

    /// Returns the current position within the data.
    ///
    /// # Returns
    ///
    /// A `usize` representing the current position (in bytes) of the reader.
    ///
    /// This function returns the current index in the data where the next read operation will occur.
    pub fn pos(&self) -> usize {
        *self.pos.borrow()
    }

    /// Creates a new `ByteReader` from a given data source that implements `AsRef<[u8]>`.
    ///
    /// # Arguments
    ///
    /// * `data` - The data source, typically a `Vec<u8>` or a slice (`&[u8]`).
    ///
    /// # Returns
    ///
    /// A new `ByteReader` handle.
    pub fn new(data: &'a [u8]) -> Self {
        ByteReader {
            data,
            pos: RefCell::new(0),
        }
    }

    /// Reads a single byte (`u8`) from the current reader position.
    ///
    /// # Returns
    ///
    /// A `Result` containing the read `u8` value, or a `ParseError` if the read fails.
    pub fn read_u8(&self) -> Result<u8> {
        let mut pos = self.pos.borrow_mut();

        if *pos + 1 > self.length() {
            return Err(UnexpectedEndOfFile);
        }

        let res = self.data.as_ref()[*pos];
        *pos += 1;
        Ok(res)
    }

    /// Reads a 16-bit unsigned integer (`u16`) from the current reader position.
    ///
    /// # Returns
    ///
    /// A `Result` containing the read `u16` value, or a `ParseError` if the read fails.
    pub fn read_u16(&self) -> Result<u16> {
        let mut pos = self.pos.borrow_mut();

        if *pos + 2 > self.length() {
            return Err(UnexpectedEndOfFile);
        }
        let _1 = self.data.as_ref()[*pos] as u16;
        let _2 = self.data.as_ref()[*pos + 1] as u16;
        let res = (_2 << 8) | _1;
        *pos += 2;
        Ok(res)
    }

    /// Reads a 32-bit unsigned integer (`u32`) from the current reader position.
    ///
    /// # Returns
    ///
    /// A `Result` containing the read `u32` value, or a `ParseError` if the read fails.
    pub fn read_u32(&self) -> Result<u32> {
        let mut pos = self.pos.borrow_mut();

        if *pos + 4 > self.length() {
            return Err(UnexpectedEndOfFile);
        }

        let _1 = self.data.as_ref()[*pos] as u32;
        let _2 = self.data.as_ref()[*pos + 1] as u32;
        let _3 = self.data.as_ref()[*pos + 2] as u32;
        let _4 = self.data.as_ref()[*pos + 3] as u32;

        let res = _4 << 24
            | _3 << 16
            | _2 << 8
            | _1;
        *pos += 4;
        Ok(res)
    }

    /// Reads a `u32` value encoded in LEB128 format from the current reader position.
    ///
    /// # Returns
    ///
    /// A `Result` containing the decoded `u32` value, or a `ParseError` if the read fails.
    pub fn read_leb128_u32(&self) -> Result<u32> {
        let mut result = 0u32;
        let mut shift = 0;

        loop {
            let byte = self.read_u8()?;

            // Add the lower 7 bits of the byte to the result
            result |= ((byte & 0x7F) as u32) << shift;

            // If the most significant bit (MSB) is not set, we are done
            if byte & 0x80 == 0 {
                break;
            }

            // If shift is 28 or more, we've read too many bytes for a u32
            if shift >= 28 {
                return Err(InvalidLEB128Encoding);
            }

            shift += 7;
        }

        Ok(result)
    }

    /// Reads an `i32` value encoded in LEB128 format from the current reader position.
    ///
    /// # Returns
    ///
    /// A `Result` containing the decoded `i32` value, or a `ParseError` if the read fails.
    pub fn read_leb128_i32(&self) -> Result<i32> {
        let mut result = 0i32;
        let mut shift = 0;

        loop {
            let byte = self.read_u8()?;
            result |= i32::from(byte & 0x7F) << shift;
            shift += 7;


            // If the high-order bit is not set, this is the last byte
            if byte & 0x80 == 0 {
                // If this was a signed value and the sign bit is set in the final byte
                if (byte & 0x40) != 0 && shift < 32 {
                    // Perform sign extension
                    result |= -(1 << shift);
                }
                return Ok(result);
            }

            // If we exceed the maximum shift for a 32-bit integer, return an error
            if shift >= 32 {
                return Err(InvalidLEB128Encoding);
            }
        }
    }

    /// Reads a 64-bit unsigned integer (`u64`) from the current reader position.
    ///
    /// # Returns
    ///
    /// A `Result` containing the read `u64` value, or a `ParseError` if the read fails.
    pub fn read_u64(&self) -> Result<u64> {
        let mut pos = self.pos.borrow_mut();

        if *pos + 8 > self.length() {
            return Err(UnexpectedEndOfFile);
        }

        let _1 = self.data.as_ref()[*pos] as u64;
        let _2 = self.data.as_ref()[*pos + 1] as u64;
        let _3 = self.data.as_ref()[*pos + 2] as u64;
        let _4 = self.data.as_ref()[*pos + 3] as u64;
        let _5 = self.data.as_ref()[*pos + 4] as u64;
        let _6 = self.data.as_ref()[*pos + 5] as u64;
        let _7 = self.data.as_ref()[*pos + 6] as u64;
        let _8 = self.data.as_ref()[*pos + 7] as u64;


        let res = _8 << 56
            | _7 << 48
            | _6 << 40
            | _5 << 32
            | _4 << 24
            | _3 << 16
            | _2 << 8
            | _1
            ;
        *pos += 8;
        Ok(res)
    }

    /// Reads a slice of bytes of a specified length from the current reader position.
    /// Advances the reader position by the length of the slice.
    ///
    /// # Arguments
    ///
    /// * `len` - The number of bytes to read.
    ///
    /// # Returns
    ///
    /// A `Result` containing the slice of bytes read, or a `ParseError` if there is not
    /// enough data left to read the requested number of bytes.
    // pub fn read_range(&mut self, len: usize) -> Result<&[u8]> {
    pub fn read_range(&self, len: usize) -> Result<Box<[u8]>> {
        let mut pos = self.pos.borrow_mut();

        let data = self.data.as_ref();

        if *pos + len > data.len() {
            return Err(UnexpectedEndOfFile);
        }

        let result = &data[*pos..*pos + len];
        *pos += len;

        Ok(Box::from(result))
    }

    /// Seeks to a new position based on the provided offset.
    ///
    /// # Arguments
    ///
    /// * `offset` - The offset to move the reader by. It can be positive (to move forward)
    ///   or negative (to move backward).
    ///
    /// # Returns
    ///
    /// A `Result` containing the new position after applying the offset or a `ParseError`
    /// if the computed position is out of bounds.
    pub fn seek(&self, offset: isize) -> Result<usize> {
        let mut pos = self.pos.borrow_mut();
        let new_pos = if offset.is_negative() {
            // Ensure we do not go below 0
            pos.saturating_sub(offset.abs() as usize)
        } else {
            // Ensure we do not go beyond the end of the data
            pos.saturating_add(offset as usize)
        };

        let data_len = self.length();

        if new_pos > data_len {
            Err(OutOfBounds)
        } else {
            *pos = new_pos;
            Ok(*pos)
        }
    }

    /// Checks if the reader has reached the end of the file.
    ///
    /// # Returns
    ///
    /// A `bool` indicating whether the reader is at the end of the file.
    pub fn eof(&self) -> bool {
        *self.pos.borrow() >= self.length()
    }
}

#[cfg(test)]
mod tests {
    use alloc::boxed::Box;
    use crate::error::WasmParseError;
    use crate::error::WasmParseError::OutOfBounds;
    use crate::reader::ByteReader;

    #[test]
    fn read_empty() {
        let data: &[u8] = &[];
        let ti = ByteReader::new(data);

        assert!(ti.read_u8().is_err());
        assert!(ti.read_u16().is_err());
        assert!(ti.read_u32().is_err());
        assert!(ti.read_u64().is_err());
    }

    #[test]
    fn read_u8() {
        let data: &[u8] = &[0x05, 0x06, 0x07, 0x08];
        let ti = ByteReader::new(data);

        assert_eq!(ti.read_u8().unwrap(), 0x05);
        assert_eq!(ti.read_u8().unwrap(), 0x06);
        assert_eq!(ti.read_u8().unwrap(), 0x07);
        assert_eq!(ti.read_u8().unwrap(), 0x08);
    }

    #[test]
    fn read_u16() {
        let data: &[u8] = &[0x05, 0x06, 0x07, 0x08];
        let ti = ByteReader::new(data);

        assert_eq!(ti.read_u16().unwrap(), 0x0605); // Little-endian: 0x0506
        assert_eq!(ti.read_u16().unwrap(), 0x0807); // Little-endian: 0x0708
    }

    #[test]
    fn read_u32() {
        let data: &[u8] = &[0x05, 0x06, 0x07, 0x08];
        let ti = ByteReader::new(data);

        assert_eq!(ti.read_u32().unwrap(), 0x08070605); // Little-endian: 0x05060708
    }

    #[test]
    fn read_u64() {
        let data: &[u8] = &[0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10];
        let ti = ByteReader::new(data);

        assert_eq!(ti.read_u64().unwrap(), 0x100F0E0D0C0B0A09); // Little-endian: 0x090A0B0C0D0E0F10
    }

    #[test]
    fn read_range() {
        let data: &[u8] = &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        let ti = ByteReader::new(data);

        assert_eq!(
            ti.read_range(4).unwrap().as_ref(),
            [0x01, 0x02, 0x03, 0x04]
        );
        assert_eq!(ti.read_u8().unwrap(), 0x05);
        assert_eq!(ti.read_range(2).unwrap().as_ref(), [0x06, 0x07]);
        assert_eq!(ti.read_u8().unwrap(), 0x08);
    }

    #[test]
    fn read_range_out_of_bounds() {
        let data: &[u8] = &[0x01, 0x02, 0x03, 0x04];
        let ti = ByteReader::new(data);

        ti.seek(3).unwrap();
        assert!(ti.read_range(2).is_err());
    }

    #[test]
    fn seek() {
        let data = b"Hello, world!";
        let reader = ByteReader::new(&data[..]);

        // Test seeking forward within bounds
        assert_eq!(reader.seek(7).unwrap(), 7);
        assert_eq!(reader.seek(3).unwrap(), 10);

        // Test seeking backward within bounds
        assert_eq!(reader.seek(-5).unwrap(), 5);
        assert_eq!(reader.seek(-10).unwrap(), 0); // Should clamp to 0

        // Test seeking beyond the data length
        assert_eq!(reader.seek(50).err().unwrap(), OutOfBounds);
    }

    #[test]
    fn read_leb128_u32_single_byte() {
        let data = [0x7F]; // 127 in LEB128
        let ti = ByteReader::new(&data);
        let result = ti.read_leb128_u32().unwrap();
        assert_eq!(result, 127);
    }

    #[test]
    fn read_leb128_u32_multiple_bytes() {
        let data = [0xE5, 0x8E, 0x26]; // 624485 in LEB128
        let ti = ByteReader::new(&data);
        let result = ti.read_leb128_u32().unwrap();
        assert_eq!(result, 624485);
    }

    #[test]
    fn read_leb128_u32_max_u32() {
        let data = [0xFF, 0xFF, 0xFF, 0xFF, 0x0F]; // Maximum u32 in LEB128
        let ti = ByteReader::new(&data);
        let result = ti.read_leb128_u32().unwrap();
        assert_eq!(result, 4294967295); // Max u32 value
    }

    #[test]
    fn read_leb128_u32_unexpected_eof() {
        let data = [0x80]; // Incomplete LEB128 encoding
        let ti = ByteReader::new(&data);
        let result = ti.read_leb128_u32();
        assert!(matches!(result, Err(WasmParseError::UnexpectedEndOfFile)));
    }

    #[test]
    fn read_leb128_u32_invalid_encoding() {
        let data = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF]; // Too many bytes for a valid u32
        let ti = ByteReader::new(&data);
        let result = ti.read_leb128_u32();
        assert!(matches!(result, Err(WasmParseError::InvalidLEB128Encoding)));
    }


    #[test]
    fn eof_empty_data() {
        let ti = ByteReader::new(&[]);
        assert!(ti.eof());
    }

    #[test]
    fn eof_non_empty_data_not_at_end() {
        let data = [0x7F]; // 127 in LEB128
        let ti = ByteReader::new(&data);
        assert!(!ti.eof());
        let _ = ti.read_leb128_u32().unwrap();
        assert!(ti.eof());
    }

    #[test]
    fn read_leb128_i32_positive_single_byte() {
        let data = [0x3F]; // 63 in LEB128
        let ti = ByteReader::new(&data);
        let result = ti.read_leb128_i32().unwrap();
        assert_eq!(result, 63);
        assert!(ti.eof());
    }

    #[test]
    fn read_leb128_i32_negative_single_byte() {
        let data = [0x41]; // -63 in LEB128
        let ti = ByteReader::new(&data);
        let result = ti.read_leb128_i32().unwrap();
        assert_eq!(result, -63);
        assert!(ti.eof());
    }

    #[test]
    fn read_leb128_i32_positive_multiple_bytes() {
        let data = [0xE5, 0x8E, 0x26]; // 624485 in LEB128
        let ti = ByteReader::new(&data);
        let result = ti.read_leb128_i32().unwrap();
        assert_eq!(result, 624485);
        assert!(ti.eof());
    }

    #[test]
    fn read_leb128_i32_negative_multiple_bytes() {
        let data = [0x9B, 0xF1, 0x59]; // -624485 in LEB128
        let ti = ByteReader::new(&data);
        let result = ti.read_leb128_i32().unwrap();
        assert_eq!(result, -624485);
        assert!(ti.eof());
    }

    #[test]
    fn read_leb128_i32_max_i32() {
        let data = [0xFF, 0xFF, 0xFF, 0xFF, 0x07]; // Maximum i32 in LEB128 (2147483647)
        let ti = ByteReader::new(&data);
        let result = ti.read_leb128_i32().unwrap();
        assert_eq!(result, i32::MAX); // Max i32 value
        assert!(ti.eof());
    }

    #[test]
    fn read_leb128_i32_min_i32() {
        let data = [0x80, 0x80, 0x80, 0x80, 0x78]; // Minimum i32 in LEB128 (-2147483648)
        let ti = ByteReader::new(&data);
        let result = ti.read_leb128_i32().unwrap();
        assert_eq!(result, i32::MIN);
        assert!(ti.eof());
    }

    #[test]
    fn read_leb128_i32_unexpected_eof() {
        let data = [0x80]; // Incomplete LEB128 encoding
        let ti = ByteReader::new(&data);
        let result = ti.read_leb128_i32();
        assert!(matches!(result, Err(WasmParseError::UnexpectedEndOfFile)));
        assert!(ti.eof());
    }

    #[test]
    fn read_leb128_i32_invalid_encoding() {
        let data = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF]; // Too many bytes for a valid i32
        let ti = ByteReader::new(&data);
        let result = ti.read_leb128_i32();
        assert!(matches!(result, Err(WasmParseError::InvalidLEB128Encoding)));
    }
}
