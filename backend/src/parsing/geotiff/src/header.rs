use std::io::SeekFrom;
use crate::error::{HeaderErrorState, TIFFErrorState};
use crate::util::{ByteOrder, FromBytes};

pub fn parse_header(buffer: &[u8]) -> Result<(ByteOrder, SeekFrom), TIFFErrorState> {
    if buffer.len() != 8 {
        return Err(TIFFErrorState::HeaderError(HeaderErrorState::InvalidLength(buffer.len())));
    }

    let byte_order = match buffer[0..2] {
        [73, 73] => ByteOrder::LittleEndian,
        [77, 77] => ByteOrder::BigEndian,
        _ => return Err(TIFFErrorState::HeaderError(HeaderErrorState::UnexpectedByteOrder([buffer[0], buffer[1]])))
    };

    let magic_numbers = &buffer[2..4];
    if !match byte_order {
        ByteOrder::LittleEndian => (magic_numbers[0] == 42) && (magic_numbers[1] == 0),
        ByteOrder::BigEndian => (magic_numbers[0] == 0) && (magic_numbers[1] == 42)
    } {
        return Err(TIFFErrorState::HeaderError(HeaderErrorState::UnexpectedMagicNumber([magic_numbers[0], magic_numbers[1]])));
    }
    let ifd_offset = SeekFrom::Start(
        u32::from_bytes(&buffer[4..8], &byte_order) as u64
    );
    return Ok((byte_order, ifd_offset));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_header_valid() {
        // A valid TIFF header, using little-endian byte order, with an IFD offset of 8
        let buffer = [0x49, 0x49, 0x2A, 0x00, 0x08, 0x00, 0x00, 0x00];
        let result = parse_header(&buffer);
        assert!(result.is_ok());
        let (byte_order, ifd_offset) = result.unwrap();
        assert_eq!(byte_order, ByteOrder::LittleEndian);
        assert_eq!(ifd_offset, SeekFrom::Start(8));
    }

    #[test]
    fn test_parse_header_invalid_length() {
        let buffer = [0; 7]; // Buffer with insufficient length of 7 bytes
        let result = parse_header(&buffer);
        assert!(matches!(result, Err(TIFFErrorState::HeaderError(HeaderErrorState::InvalidLength(_)))));
    }

    #[test]
    fn test_parse_header_unexpected_byte_order() {
        let buffer = [0x00, 0x00, 0x2A, 0x00, 0x08, 0x00, 0x00, 0x00];
        let result = parse_header(&buffer);
        assert!(matches!(result, Err(TIFFErrorState::HeaderError(HeaderErrorState::UnexpectedByteOrder(_)))));
    }

    #[test]
    fn test_parse_header_unexpected_magic_number() {
        let buffer = [0x49, 0x49, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00];
        let result = parse_header(&buffer);
        assert!(matches!(result, Err(TIFFErrorState::HeaderError(HeaderErrorState::UnexpectedMagicNumber(_)))));
    }

    #[test]
    fn test_parse_header_valid_big_endian() {
        // A valid TIFF header, using big-endian byte order, with an IFD offset of 8
        let buffer = [0x4D, 0x4D, 0x00, 0x2A, 0x00, 0x00, 0x00, 0x08];
        let result = parse_header(&buffer);
        assert!(result.is_ok());
        let (byte_order, ifd_offset) = result.unwrap();
        assert_eq!(byte_order, ByteOrder::BigEndian);
        assert_eq!(ifd_offset, SeekFrom::Start(8));
    }

    #[test]
    fn test_parse_header_unexpected_magic_number_big_endian() {
        // Incorrect magic number in big-endian format
        let buffer = [0x4D, 0x4D, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08];
        let result = parse_header(&buffer);
        assert!(matches!(result, Err(TIFFErrorState::HeaderError(HeaderErrorState::UnexpectedMagicNumber(_)))));
    }

    #[test]
    fn test_parse_header_specific_error_values() {
        // Specific error value assertions for unexpected byte order
        let buffer = [0x00, 0x00, 0x2A, 0x00, 0x08, 0x00, 0x00, 0x00];
        if let Err(TIFFErrorState::HeaderError(HeaderErrorState::UnexpectedByteOrder(error_bytes))) = parse_header(&buffer) {
            assert_eq!(error_bytes, [0x00, 0x00]);
        } else {
            panic!("Expected UnexpectedByteOrder error");
        }

        // Specific error value assertions for unexpected magic number
        let buffer = [0x49, 0x49, 0x00, 0x3B, 0x08, 0x00, 0x00, 0x00];
        if let Err(TIFFErrorState::HeaderError(HeaderErrorState::UnexpectedMagicNumber(error_bytes))) = parse_header(&buffer) {
            assert_eq!(error_bytes, [0x00, 0x3B]);
        } else {
            panic!("Expected UnexpectedMagicNumber error");
        }
    }

    #[test]
    fn test_parse_header_zero_ifd_offset() {
        // Test case with a zero IFD offset
        let buffer = [0x49, 0x49, 0x2A, 0x00, 0x00, 0x00, 0x00, 0x00];
        let result = parse_header(&buffer);
        assert!(result.is_ok());
        let (_, ifd_offset) = result.unwrap();
        assert_eq!(ifd_offset, SeekFrom::Start(0));
    }

    #[test]
    fn test_parse_header_high_ifd_offset() {
        // Test case with a very high IFD offset
        let buffer = [0x49, 0x49, 0x2A, 0x00, 0xFF, 0xFF, 0xFF, 0xFF];
        let result = parse_header(&buffer);
        assert!(result.is_ok());
        let (_, ifd_offset) = result.unwrap();
        assert_eq!(ifd_offset, SeekFrom::Start(4294967295));
    }
    #[test]
    fn test_parse_header_empty_buffer() {
        let buffer = [];
        let result = parse_header(&buffer);
        assert!(matches!(result, Err(TIFFErrorState::HeaderError(HeaderErrorState::InvalidLength(0)))));
    }
    #[test]
    fn test_parse_header_buffer_too_long() {
        let buffer = [0x49, 0x49, 0x2A, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00]; // 9 bytes, longer than the expected 8 bytes
        let result = parse_header(&buffer);
        assert!(matches!(result, Err(TIFFErrorState::HeaderError(HeaderErrorState::InvalidLength(9)))));
    }
    #[test]
    fn test_parse_header_invalid_magic_number_with_correct_byte_order() {
        let buffer = [0x49, 0x49, 0xFF, 0xFF, 0x08, 0x00, 0x00, 0x00]; // Used an invalid magic number
        let result = parse_header(&buffer);
        assert!(matches!(result, Err(TIFFErrorState::HeaderError(HeaderErrorState::UnexpectedMagicNumber([0xFF, 0xFF])))));
    }
    #[test]
    fn test_parse_header_max_ifd_offset() {
        // Using the maximum u32 value as the IFD offset
        let buffer = [0x49, 0x49, 0x2A, 0x00, 0xFF, 0xFF, 0xFF, 0xFF];
        let result = parse_header(&buffer);
        assert!(result.is_ok());
        let (_, ifd_offset) = result.unwrap();
        assert_eq!(ifd_offset, SeekFrom::Start(u32::MAX as u64));
    }

}

