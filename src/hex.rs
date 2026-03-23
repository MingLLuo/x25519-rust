pub fn hex_decode<const N: usize>(s: &str) -> Result<[u8; N], String> {
    if s.len() != N * 2 {
        return Err(format!(
            "Invalid length: expected {}, got {}",
            N * 2,
            s.len()
        ));
    }

    let mut bytes = [0u8; N];
    for i in 0..N {
        let byte_str = &s[i * 2..i * 2 + 2];
        match u8::from_str_radix(byte_str, 16) {
            Ok(byte) => bytes[i] = byte,
            Err(_) => return Err(format!("Invalid hex character in '{}'", byte_str)),
        }
    }
    Ok(bytes)
}

pub fn hex_encode(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &byte in bytes {
        s.push_str(&format!("{:02x}", byte));
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hex_decode() {
        assert_eq!(
            hex_decode::<5>("deadbeef00").unwrap(),
            [0xde, 0xad, 0xbe, 0xef, 0x00]
        );
        assert!(hex_decode::<2>("123").is_err());
        assert!(hex_decode::<1>("zz").is_err());
    }

    #[test]
    fn test_hex_encode() {
        let bytes = [0xde, 0xad, 0xbe, 0xef, 0x00];
        assert_eq!(hex_encode(&bytes), "deadbeef00");
    }
}
