use std::ops::{Add, Mul, Sub};

const BYTES: usize = 32;
const FIELD_BITS: usize = 255;
const LIMB_SZ: usize = 5;
const RADIX_POW: u32 = 51;
const LIMIT: u64 = 1 << RADIX_POW;
const MASK: u64 = LIMIT - 1;

#[derive(Clone, Copy, Debug)]
pub struct Field25519 {
    x: [u64; LIMB_SZ],
}

pub struct MulResult {
    x: [u128; LIMB_SZ * 2],
}

#[derive(Clone, Copy, Debug)]
pub struct Point25519 {
    pub x: Field25519,
    pub y: Field25519,
    pub z: Field25519,
    pub t: Field25519,
}

// const D: Field25519 = Field25519 {
//     x: [
//         0x00034dca135978a3,
//         0x0001a8283b156ebd,
//         0x0005e7a26001c029,
//         0x000739c663a03cbb,
//         0x00052036cee2b6ff,
//     ],
// };
const K: Field25519 = Field25519 {
    x: [
        0x00069b9426b2f159,
        0x00035050762add7a,
        0x0003cf44c0038052,
        0x0006738cc7407977,
        0x0002406d9dc56dff,
    ],
};
const ZERO: Field25519 = Field25519 {
    x: [0x0, 0x0, 0x0, 0x0, 0x0],
};
const ONE: Field25519 = Field25519 {
    x: [0x1, 0x0, 0x0, 0x0, 0x0],
};
const TWO: Field25519 = Field25519 {
    x: [0x2, 0x0, 0x0, 0x0, 0x0],
};
const SCALED_P: Field25519 = Field25519 {
    x: [
        2 * (2u64.pow(RADIX_POW) - 19),
        2 * (2u64.pow(RADIX_POW) - 1),
        2 * (2u64.pow(RADIX_POW) - 1),
        2 * (2u64.pow(RADIX_POW) - 1),
        2 * (2u64.pow(RADIX_POW) - 1),
    ],
};

pub const G: Point25519 = Point25519 {
    x: Field25519 {
        x: [
            0x00062d608f25d51a,
            0x000412a4b4f6592a,
            0x00075b7171a4b31d,
            0x0001ff60527118fe,
            0x000216936d3cd6e5,
        ],
    },
    y: Field25519 {
        x: [
            0x0006666666666658,
            0x0004cccccccccccc,
            0x0001999999999999,
            0x0003333333333333,
            0x0006666666666666,
        ],
    },
    z: ONE,
    t: Field25519 {
        x: [
            0x00068ab3a5b7dda3,
            0x00000eea2a5eadbb,
            0x0002af8df483c27e,
            0x000332b375274732,
            0x00067875f0fd78b7,
        ],
    },
};
const INFINITY: Point25519 = Point25519 {
    x: ZERO,
    y: ONE,
    z: ONE,
    t: ZERO,
};

impl PartialEq for Field25519 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
    }
}

impl Add for Field25519 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut result = Self { x: [0; LIMB_SZ] };

        for j in 0..LIMB_SZ {
            result.x[j] = self.x[j] + other.x[j];
        }

        result
    }
}

impl Sub for Field25519 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut result = Self { x: [0; LIMB_SZ] };

        for j in 0..LIMB_SZ {
            result.x[j] = SCALED_P.x[j] + self.x[j] - other.x[j];
        }

        result
    }
}

impl Mul for Field25519 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut unreduced = MulResult {
            x: [0; LIMB_SZ * 2],
        };

        for i in 0..LIMB_SZ {
            for j in 0..LIMB_SZ {
                unreduced.x[i + j] += self.x[i] as u128 * other.x[j] as u128;
            }
        }

        Field25519::from(unreduced)
    }
}

impl Field25519 {
    fn sqr(&self) -> Field25519 {
        let mut unreduced = MulResult {
            x: [0; LIMB_SZ * 2],
        };

        for i in 0..LIMB_SZ {
            debug_assert!(
                self.x[i] < (1 << 53),
                "mul operands exceed expected bound (< 2^53)"
            );
            unreduced.x[i + i] += self.x[i] as u128 * self.x[i] as u128;
        }

        for i in 0..LIMB_SZ {
            for j in i + 1..LIMB_SZ {
                unreduced.x[i + j] += (self.x[i] as u128 * self.x[j] as u128) << 1;
            }
        }

        Field25519::from(unreduced)
    }

    fn sqr_n_times(&self, n: u8) -> Field25519 {
        let mut result = Field25519 { x: self.x.clone() };

        for _ in 0..n {
            result = result.sqr();
        }

        result
    }
}

impl From<Vec<u8>> for Field25519 {
    fn from(x: Vec<u8>) -> Field25519 {
        let mut result: Field25519 = Field25519 { x: [0; LIMB_SZ] };
        let mut cur_bits: u32 = 0;
        let mut i: usize = 0;
        let mut element: u64 = 0;

        for j in 0..x.len() {
            if cur_bits >= RADIX_POW - 8 {
                let remaining = RADIX_POW - cur_bits;
                let mask: u64 = (1 << remaining) - 1;

                element |= ((x[j] as u64) & mask) << cur_bits;
                result.x[i] = element;
                i += 1;

                element = (x[j] >> remaining) as u64;
                cur_bits = 8 - remaining;
            } else {
                element |= (x[j] as u64) << cur_bits;
                cur_bits += 8;
            }
        }

        result
    }
}

impl From<MulResult> for Field25519 {
    fn from(unreduced: MulResult) -> Field25519 {
        let mut folded: [u128; LIMB_SZ] = [0; LIMB_SZ];
        for j in 0..LIMB_SZ {
            folded[j] = unreduced.x[j] + 19 * unreduced.x[j + LIMB_SZ];
            debug_assert!(
                folded[j] < (1u128 << 112),
                "Unreduced values after Solinas fold larger than expected"
            );
        }

        let mut carry: u128 = 0;
        let mut product = Field25519 { x: [0; LIMB_SZ] };
        let mut sum: u128;

        for j in 0..LIMB_SZ {
            sum = folded[j] + carry;
            product.x[j] = (sum as u64) & MASK;
            carry = sum >> RADIX_POW;
        }

        carry *= 19;
        loop {
            for j in 0..LIMB_SZ {
                sum = product.x[j] as u128 + carry;
                product.x[j] = (sum as u64) & MASK;
                carry = sum >> RADIX_POW;

                if carry == 0 {
                    break;
                }
            }

            if carry == 0 {
                break;
            }
            carry *= 19;
        }

        product
    }
}

impl From<Field25519> for [u8; BYTES] {
    fn from(u: Field25519) -> [u8; BYTES] {
        let mut bytes: [u8; BYTES] = [0; BYTES];
        let mut offset: u32 = 0;
        let mut i: usize = 0;

        for j in 0..BYTES {
            if offset + 8 >= RADIX_POW && i != LIMB_SZ - 1 {
                let this_limb = RADIX_POW - offset;
                let next_limb = 8 - this_limb;

                bytes[j] = (u.x[i] >> offset) as u8;
                let mask: u64 = (1 << next_limb) - 1;
                bytes[j] |= ((u.x[i + 1] & mask) as u8) << this_limb;

                offset = next_limb;
                i += 1;
            } else {
                bytes[j] = (u.x[i] >> offset) as u8;
                offset += 8;
            }
        }

        bytes
    }
}

impl Add for Point25519 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let x1 = self.x;
        let y1 = self.y;
        let z1 = self.z;
        let t1 = self.t;
        let x2 = rhs.x;
        let y2 = rhs.y;
        let z2 = rhs.z;
        let t2 = rhs.t;

        let a = (y1 - x1) * (y2 - x2);
        let b = (y1 + x1) * (y2 + x2);
        let c = t1 * K * t2;
        let d = z1 * TWO * z2;
        let e = b - a;
        let f = d - c;
        let g = d + c;
        let h = b + a;

        Self {
            x: e * f,
            y: g * h,
            z: f * g,
            t: e * h,
        }
    }
}

impl Mul<&[u8]> for Point25519 {
    type Output = Self;

    fn mul(self, rhs: &[u8]) -> Self {
        let mut r0: Self = INFINITY;
        let mut r1: Self = self.clone();

        for i in (0..FIELD_BITS).rev() {
            if test_bit(rhs, i) {
                r0 = r0 + r1;
                r1 = r1.double();
            } else {
                r1 = r1 + r0;
                r0 = r0.double();
            }
        }

        r0
    }
}

#[inline]
fn test_bit(n: &[u8], j: usize) -> bool {
    let byte = j >> 3;
    let bit = j & 0b111;

    ((n[byte] >> bit) & 1) == 1
}

impl From<Point25519> for [u8; BYTES] {
    fn from(value: Point25519) -> Self {
        let mut bytes: [u8; BYTES] = value.y.into();
        let sign: u8 = value.x.x[0] as u8 & 1;
        bytes[BYTES - 1] |= sign << 7;

        bytes
    }
}

impl Point25519 {
    fn double(self) -> Self {
        let x1 = self.x;
        let y1 = self.y;
        let z1 = self.z;

        let a = x1.sqr();
        let b = y1.sqr();
        let t = z1.sqr();
        let c = t + t;
        let h = a + b;
        let e = h - (x1 + y1).sqr();
        let g = a - b;
        let f = c + g;

        Self {
            x: e * f,
            y: g * h,
            t: e * h,
            z: f * g,
        }
    }

    pub fn normalize(self) -> Self {
        let z = self.z;
        let z2 = z.sqr();
        let z8 = z2.sqr().sqr();
        let z9 = z8 * z;
        let z11 = z9 * z2;
        let z22 = z11.sqr();
        let z_5_0 = z22 * z9;

        let z_10_0 = z_5_0.sqr_n_times(5) * z_5_0;
        let z_20_0 = z_10_0.sqr_n_times(10) * z_10_0;
        let z_40_0 = z_20_0.sqr_n_times(20) * z_20_0;
        let z_50_0 = z_40_0.sqr_n_times(10) * z_10_0;
        let z_100_0 = z_50_0.sqr_n_times(50) * z_50_0;
        let z_200_0 = z_100_0.sqr_n_times(100) * z_100_0;
        let z_250_0 = z_200_0.sqr_n_times(50) * z_50_0;

        let zinv = z_250_0.sqr_n_times(5) * z11;

        let x3 = self.x * zinv;
        let y3 = self.y * zinv;
        let t3 = x3 * y3;

        Self {
            x: x3,
            y: y3,
            z: ONE,
            t: t3,
        }
    }
}

#[cfg(test)]
#[path = "unit_tests/edwards25519_test.rs"]
mod edwards25519_test;
