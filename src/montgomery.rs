use crate::field::*;
use crypto_bigint::subtle::{Choice, ConditionallySelectable};

const A2_DIV4: u32 = 121666;

pub fn x_dbl(x: Fe, z: Fe) -> (Fe, Fe) {
    let xz_sum = add(x, z);
    let xz_diff = sub(x, z);
    let q = square(xz_sum);
    let r = square(xz_diff);
    let s = sub(q, r);
    (mul(q, r), mul(s, add(r, mul_u32(s, A2_DIV4))))
}

// x2, z2 = x(P), z(P), x3, z3 = x(Q), z(Q), x1 = x(P-Q), z1 = 1 omitted
pub fn x_add(x2: Fe, z2: Fe, x3: Fe, z3: Fe, x1: Fe) -> (Fe, Fe) {
    let pxz_sum = add(x2, z2);
    let pxz_diff = sub(x2, z2);
    let qxz_sum = add(x3, z3);
    let qxz_diff = sub(x3, z3);
    let u = mul(pxz_diff, qxz_sum);
    let v = mul(pxz_sum, qxz_diff);
    let uv_sum = add(u, v);
    let uv_diff = sub(u, v);
    let x = square(uv_sum);
    let z = mul(square(uv_diff), x1);
    (x, z)
}

fn cswap(swap: u8, x2: &mut Fe, z2: &mut Fe, x3: &mut Fe, z3: &mut Fe) {
    let tx = Fe::conditional_select(x2, x3, Choice::from(swap));
    let tz = Fe::conditional_select(z2, z3, Choice::from(swap));
    let ux = Fe::conditional_select(x3, x2, Choice::from(swap));
    let uz = Fe::conditional_select(z3, z2, Choice::from(swap));
    *x2 = tx;
    *z2 = tz;
    *x3 = ux;
    *z3 = uz;
}

pub fn ladder(k: [u8; 32], x1: Fe) -> Fe {
    let mut x2 = ONE;
    let mut z2 = ZERO;
    let mut x3 = x1;
    let mut z3 = ONE;

    let mut swap = 0u8;
    for t in (0..=254).rev() {
        let k_t = (k[t / 8] >> (t & 7)) & 1;
        swap ^= k_t;
        cswap(swap, &mut x2, &mut z2, &mut x3, &mut z3);
        swap = k_t;

        let (x2_new, z2_new) = x_dbl(x2, z2);
        let (x3_new, z3_new) = x_add(x2, z2, x3, z3, x1);

        x2 = x2_new;
        z2 = z2_new;
        x3 = x3_new;
        z3 = z3_new;
    }
    cswap(swap, &mut x2, &mut z2, &mut x3, &mut z3);
    mul(x2, invert(z2))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hex;

    fn decimal_to_le_bytes_32(dec: &str) -> [u8; 32] {
        assert!(!dec.is_empty(), "decimal string must not be empty");
        let mut out = [0u8; 32];
        for ch in dec.bytes() {
            assert!(ch.is_ascii_digit(), "invalid decimal digit in input");
            let digit = (ch - b'0') as u16;
            let mut carry = digit;
            for byte in &mut out {
                let acc = (*byte as u16) * 10 + carry;
                *byte = (acc & 0xff) as u8;
                carry = acc >> 8;
            }
            assert_eq!(carry, 0, "decimal value does not fit in 256 bits");
        }
        out
    }

    fn expected_le_bytes_from_dec(dec: &str) -> [u8; 32] {
        let hex = hex::hex_encode(&decimal_to_le_bytes_32(dec));
        hex::hex_decode::<32>(&hex).unwrap()
    }

    fn scalar_u8(x: u8) -> [u8; 32] {
        let mut k = [0u8; 32];
        k[0] = x;
        k
    }

    #[test]
    fn ladder_curve25519_known_points() {
        let x1 = from_bytes_le({
            let mut b = [0u8; 32];
            b[0] = 9;
            b
        });

        let x2 = ladder(scalar_u8(2), x1);
        let x3 = ladder(scalar_u8(3), x1);
        let x4 = ladder(scalar_u8(4), x1);
        let x5 = ladder(scalar_u8(5), x1);
        let x7 = ladder(scalar_u8(7), x1);

        let x2_expected = expected_le_bytes_from_dec(
            "14847277145635483483963372537557091634710985132825781088887140890597596352251",
        );
        let x3_expected = expected_le_bytes_from_dec(
            "12697861248284385512127539163427099897745340918349830473877503196793995869202",
        );
        let x4_expected = expected_le_bytes_from_dec(
            "55094879196667521951171181671895976763495004283458921215716618814842818532335",
        );
        let x5_expected = expected_le_bytes_from_dec(
            "29723531761959712214579609737676588517305008794118309711793522224007834336391",
        );
        let x7_expected = expected_le_bytes_from_dec(
            "6189616607995615193367150877376005513902989163470402290395604116858034460712",
        );

        assert_eq!(to_bytes_le(x2), x2_expected);
        assert_eq!(to_bytes_le(x3), x3_expected);
        assert_eq!(to_bytes_le(x4), x4_expected);
        assert_eq!(to_bytes_le(x5), x5_expected);
        assert_eq!(to_bytes_le(x7), x7_expected);
    }
}
