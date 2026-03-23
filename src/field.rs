use crypto_bigint::modular::ConstMontyForm;
use crypto_bigint::{Encoding, U256, impl_modulus};

// p = 2^255 - 19
impl_modulus!(
    P25519,
    U256,
    "7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffed"
);

pub type Fe = ConstMontyForm<P25519, 4>;

pub const ZERO: Fe = Fe::ZERO;
pub const ONE: Fe = Fe::ONE;

// little-endian byte array to field element
pub fn from_bytes_le(bytes: [u8; 32]) -> Fe {
    let x = U256::from_le_bytes(bytes);
    Fe::new(&x)
}

pub fn to_bytes_le(x: Fe) -> [u8; 32] {
    x.retrieve().to_le_bytes()
}

pub fn add(a: Fe, b: Fe) -> Fe {
    a + b
}

pub fn sub(a: Fe, b: Fe) -> Fe {
    a - b
}

pub fn mul(a: Fe, b: Fe) -> Fe {
    a * b
}

pub fn square(a: Fe) -> Fe {
    a * a
}

pub fn mul_u32(a: Fe, k: u32) -> Fe {
    let k = Fe::new(&U256::from_u32(k));
    a * k
}

pub fn invert(a: Fe) -> Fe {
    // a^(p-2) where p = 2^255 - 19, so p-2 = 2^255 - 21.
    let mut acc = ONE;
    let base = a;

    // e = (2^255 - 1) - 20 => all 1 bits except bit2 and bit4 are 0 in the low end.
    let mut exp_bits = [1u8; 255];
    exp_bits[2] = 0;
    exp_bits[4] = 0;

    for &bit in exp_bits.iter().rev() {
        acc = square(acc);
        if bit == 1 {
            acc = mul(acc, base);
        }
    }
    acc
}
