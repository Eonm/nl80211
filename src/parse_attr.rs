use std::convert::TryInto;

/// Parse a vec of bytes as hex String
pub fn parse_hex(input: &Vec<u8>) -> String {
    let value: Vec<char> = hex::encode_upper(input).chars().collect();
    let split = value
        .chunks(2)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>();

    split.join(":")
}

/// Parse a vec of bytes as a String
pub fn parse_string(input: &Vec<u8>) -> String {
    String::from_utf8_lossy(&input).to_owned().to_string()
}

/// Parse a vec of bytes as u8
pub fn parse_u8(input: &Vec<u8>) -> u8 {
    let to_array =
        |slice: &[u8]| -> [u8; 1] { slice.try_into().expect("slice with incorrect length") };

    u8::from_le_bytes(to_array(input))
}

/// Parse a vec of bytes as i8
pub fn parse_i8(input: &Vec<u8>) -> i8 {
    let to_array =
        |slice: &[u8]| -> [u8; 1] { slice.try_into().expect("slice with incorrect length") };

    i8::from_le_bytes(to_array(input))
}

/// Parse a vec of bytes as u16
pub fn parse_u16(input: &Vec<u8>) -> u16 {
    let to_array =
        |slice: &[u8]| -> [u8; 2] { slice.try_into().expect("slice with incorrect length") };

    u16::from_le_bytes(to_array(input))
}

/// Parse a vec of bytes as u32
pub fn parse_u32(input: &Vec<u8>) -> u32 {
    let to_array =
        |slice: &[u8]| -> [u8; 4] { slice.try_into().expect("slice with incorrect length") };

    u32::from_le_bytes(to_array(input))
}

/// Parse a vec of bytes as i32
pub fn parse_i32(input: &Vec<u8>) -> i32 {
    let to_array =
        |slice: &[u8]| -> [u8; 4] { slice.try_into().expect("slice with incorrect length") };

    i32::from_le_bytes(to_array(input))
}

/// Parse a vec of bytes as u64
pub fn parse_u64(input: &Vec<u8>) -> u64 {
    let to_array =
        |slice: &[u8]| -> [u8; 8] { slice.try_into().expect("slice with incorrect length") };

    u64::from_le_bytes(to_array(input))
}

#[cfg(test)]
mod test_type_conversion {
    use super::*;

    #[test]
    fn test_parse_hex() {
        let bytes_input: Vec<u8> = vec![255, 255, 255, 255, 255, 255];
        assert_eq!(parse_hex(&bytes_input), "FF:FF:FF:FF:FF:FF".to_string());
    }

    #[test]
    fn test_parse_string() {
        let input_string = "test".to_string();
        let bytes_string = input_string.as_bytes().to_vec();
        assert_eq!(parse_string(&bytes_string), input_string);
    }

    #[test]
    fn test_parse_u8() {
        assert_eq!(parse_u8(&vec![8]), 8 as u8);
    }

    #[test]
    #[should_panic]
    fn test_parse_u8_should_panic() {
        assert_eq!(parse_u8(&vec![8, 0]), 8 as u8);
    }

    #[test]
    fn test_parse_i8() {
        assert_eq!(parse_i8(&vec![8]), 8 as i8);
    }

    #[test]
    #[should_panic]
    fn test_parse_i8_should_panic() {
        assert_eq!(parse_i8(&vec![8, 0]), 8 as i8);
    }

    #[test]
    fn test_parse_u16() {
        assert_eq!(parse_u16(&vec![1, 0]), 1 as u16);
    }

    #[test]
    #[should_panic]
    fn test_parse_u16_should_panic() {
        assert_eq!(parse_u16(&vec![1, 0, 0]), 1 as u16);
        assert_eq!(parse_u16(&vec![1]), 1 as u16);
    }

    #[test]
    fn test_parse_u32() {
        assert_eq!(parse_u32(&vec![1, 0, 0, 0]), 1 as u32);
    }

    #[test]
    #[should_panic]
    fn test_parse_u32_should_panic() {
        assert_eq!(parse_u32(&vec![1, 0, 0, 0, 0]), 1 as u32);
        assert_eq!(parse_u32(&vec![1, 0, 0]), 1 as u32);
    }

    #[test]
    fn test_parse_i32() {
        assert_eq!(parse_i32(&vec![1, 0, 0, 0]), 1 as i32);
    }

    #[test]
    #[should_panic]
    fn test_parse_i32_should_panic() {
        assert_eq!(parse_i32(&vec![1, 0, 0, 0, 0]), 1 as i32);
        assert_eq!(parse_i32(&vec![1, 0, 0]), 1 as i32);
    }

    #[test]
    fn test_parse_u64() {
        assert_eq!(parse_u64(&vec![1, 0, 0, 0, 0, 0, 0, 0]), 1 as u64);
    }

    #[test]
    #[should_panic]
    fn test_parse_u64_should_panic() {
        assert_eq!(parse_u64(&vec![1, 0, 0, 0, 0, 0, 0, 0, 0]), 1 as u64);
        assert_eq!(parse_u64(&vec![1, 0, 0]), 1 as u64);
    }
}
