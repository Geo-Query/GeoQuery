// ByteOrder enum.
#[derive(Debug, Clone, PartialEq)]
pub enum ByteOrder {
    LittleEndian,
    BigEndian
}

// Integer parsing impls.
// Implemented for u16, u32, and f64
pub trait FromBytes {
    fn from_bytes(bytes: &[u8], byte_order: &ByteOrder) -> Self;
}

impl FromBytes for u16 {
    fn from_bytes(bytes: &[u8], byte_order: &ByteOrder) -> Self {
        let bytes: [u8; 2] = bytes.try_into().unwrap();
        return match byte_order {
            ByteOrder::LittleEndian => u16::from_le_bytes(bytes),
            ByteOrder::BigEndian => u16::from_be_bytes(bytes)
        }
    }
}

impl FromBytes for u32 {
    fn from_bytes(bytes: &[u8], byte_order: &ByteOrder) -> Self {
        let bytes: [u8; 4] = bytes.try_into().unwrap();
        return match byte_order {
            ByteOrder::LittleEndian => u32::from_le_bytes(bytes),
            ByteOrder::BigEndian => u32::from_be_bytes(bytes)
        }
    }
}
impl FromBytes for f64 {
    fn from_bytes(bytes: &[u8], byte_order: &ByteOrder) -> Self {
        let bytes: [u8; 8] = bytes.try_into().unwrap();
        return match byte_order {
            ByteOrder::LittleEndian => f64::from_le_bytes(bytes),
            ByteOrder::BigEndian => f64::from_be_bytes(bytes)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u16_endianness() {
        // Test LittleEndian
        let bytes_le = [0x01, 0x02]; // Represents the number 0x0201
        let result_le = u16::from_bytes(&bytes_le, &ByteOrder::LittleEndian);
        assert_eq!(result_le, 0x0201);

        // Test BigEndian
        let bytes_be = [0x01, 0x02]; // Represents the number 0x0102
        let result_be = u16::from_bytes(&bytes_be, &ByteOrder::BigEndian);
        assert_eq!(result_be, 0x0102);
    }

    #[test]
    fn test_u32_endianness() {
        // Test LittleEndian
        let bytes_le = [0x01, 0x02, 0x03, 0x04]; // Represents the number 0x04030201
        let result_le = u32::from_bytes(&bytes_le, &ByteOrder::LittleEndian);
        assert_eq!(result_le, 0x04030201);

        // Test BigEndian
        let bytes_be = [0x01, 0x02, 0x03, 0x04]; // Represents the number 0x01020304
        let result_be = u32::from_bytes(&bytes_be, &ByteOrder::BigEndian);
        assert_eq!(result_be, 0x01020304);
    }

    #[test]
    fn test_f64_endianness() {
        // Test LittleEndian
        let bytes_le = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x40]; // Represents a 64-bit floating-point number 3.0
        let result_le = f64::from_bytes(&bytes_le, &ByteOrder::LittleEndian);
        assert_eq!(result_le, 3.0);

        // Test BigEndian
        let bytes_be = [0x40, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]; // Represents a 64-bit floating-point number 3.0
        let result_be = f64::from_bytes(&bytes_be, &ByteOrder::BigEndian);
        assert_eq!(result_be, 3.0);
    }

    #[test]
    fn test_f64_extreme_values() {
        // Test maximum positive value
        let max_positive = f64::MAX.to_le_bytes();
        let result_max_positive_le = f64::from_bytes(&max_positive, &ByteOrder::LittleEndian);
        assert_eq!(result_max_positive_le, f64::MAX);

        let max_positive_be = f64::MAX.to_be_bytes();
        let result_max_positive_be = f64::from_bytes(&max_positive_be, &ByteOrder::BigEndian);
        assert_eq!(result_max_positive_be, f64::MAX);

        // Test maximum negative value
        let max_negative = f64::MIN.to_le_bytes();
        let result_max_negative_le = f64::from_bytes(&max_negative, &ByteOrder::LittleEndian);
        assert_eq!(result_max_negative_le, f64::MIN);

        let max_negative_be = f64::MIN.to_be_bytes();
        let result_max_negative_be = f64::from_bytes(&max_negative_be, &ByteOrder::BigEndian);
        assert_eq!(result_max_negative_be, f64::MIN);
    }
    #[test]
    fn test_byte_order_enum_completeness() {
        assert_eq!(ByteOrder::LittleEndian, ByteOrder::LittleEndian);
        assert_eq!(ByteOrder::BigEndian, ByteOrder::BigEndian);
    }


}

