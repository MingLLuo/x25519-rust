use crate::field::*;
use crate::montgomery;
pub fn x25519(mut m: [u8; 32], u: [u8; 32]) -> [u8; 32] {
    set_scalar(&mut m);
    let mut u = u;
    // Following RFC 7748 Decode U-coordinate,
    // We should ignore the MSB of it.
    u[31] &= 127;
    let x1 = from_bytes_le(u);
    let out = montgomery::ladder(m, x1);
    to_bytes_le(out)
}

fn set_scalar(k: &mut [u8; 32]) {
    k[0] &= 248;
    k[31] &= 127;
    k[31] |= 64;
}

#[cfg(test)]
mod tests {
    use crate::hex;
    use super::*;

    fn decode32(s: &str) -> [u8; 32] {
        hex::hex_decode::<32>(s).unwrap()
    }

    #[test]
    fn rfc7748_vector_1_matches() {
        let m = decode32("a546e36bf0527c9d3b16154b82465edd62144c0ac1fc5a18506a2244ba449ac4");
        let u = decode32("e6db6867583030db3594c1a424b15f7c726624ec26b3353b10a903a6d0ab1c4c");
        let expected = decode32("c3da55379de9c6908e94ea4df28d084f32eccf03491c71f754b4075577a28552");

        assert_eq!(x25519(m, u), expected);
    }

    #[test]
    fn rfc7748_vector_2_matches() {
        let m = decode32("4b66e9d4d1b4673c5ad22691957d6af5c11b6421e0ea01d42ca4169e7918ba0d");
        let u = decode32("e5210f12786811d3f4b7959d0538ae2c31dbe7106fc03c3efc4cd549c715a493");
        let expected = decode32("95cbde9476e8907d7aade45cb4b873f88b595a68799fa152e6f8f7647aac7957");

        assert_eq!(x25519(m, u), expected);
    }
}
