use std::cmp::Ordering::{Equal, Greater, Less};
use std::ops::{Add, Mul, Sub};
use std::sync::OnceLock;

use num_bigint::BigUint;

use crate::comb::Ed25519Comb;

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

const D: Field25519 = Field25519 {
    x: [
        0x00034dca135978a3,
        0x0001a8283b156ebd,
        0x0005e7a26001c029,
        0x000739c663a03cbb,
        0x00052036cee2b6ff,
    ],
};
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
const P: Field25519 = Field25519 {
    x: [
        2u64.pow(RADIX_POW) - 19,
        2u64.pow(RADIX_POW) - 1,
        2u64.pow(RADIX_POW) - 1,
        2u64.pow(RADIX_POW) - 1,
        2u64.pow(RADIX_POW) - 1,
    ],
};
const SCALED_P: Field25519 = Field25519 {
    x: [2 * P.x[0], 2 * P.x[1], 2 * P.x[2], 2 * P.x[3], 2 * P.x[4]],
};
const SQRT: Field25519 = Field25519 {
    x: [
        0x00061b274a0ea0b0,
        0x0000d5a5fc8f189d,
        0x0007ef5e9cbd0c60,
        0x00078595a6804c9e,
        0x0002b8324804fc1d,
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
pub const INFINITY: Point25519 = Point25519 {
    x: ZERO,
    y: ONE,
    z: ONE,
    t: ZERO,
};

static ED25519_COMB: OnceLock<Ed25519Comb> = OnceLock::new();

impl PartialEq for Field25519 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
    }
}

impl PartialOrd for Field25519 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        for j in (0..LIMB_SZ).rev() {
            if self.x[j] < other.x[j] {
                return Some(Less);
            } else if self.x[j] > other.x[j] {
                return Some(Greater);
            }
        }

        Some(Equal)
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
    pub fn reduce(&self) -> Self {
        let mut x = [0u64; LIMB_SZ];
        let mut k: u64 = 0;

        for j in 0..LIMB_SZ {
            let sum = self.x[j] + k;
            x[j] = sum & MASK;
            k = sum >> RADIX_POW;
        }
        x[0] += 19 * k;

        for j in 0..LIMB_SZ - 1 {
            k = x[j] >> RADIX_POW;
            x[j] &= MASK;
            x[j + 1] += k;
        }

        let mut t = [0u64; LIMB_SZ];
        t[0] = x[0].wrapping_add(19);
        k = t[0] >> RADIX_POW;
        t[0] &= MASK;

        for j in 1..LIMB_SZ {
            t[j] = x[j] + k;
            k = t[j] >> RADIX_POW;
            t[j] &= MASK
        }

        let mask = 0u64.wrapping_sub(k);
        for j in 0..LIMB_SZ {
            x[j] = (x[j] & !mask) | (t[j] & mask);
        }

        Self { x }
    }

    pub fn neg(&self) -> Field25519 {
        let mut result = Self { x: [0; LIMB_SZ] };

        let mut t: i128;
        let mut k: i128 = 0;

        for j in 0..LIMB_SZ {
            t = P.x[j] as i128 - self.x[j] as i128 + k;
            result.x[j] = t as u64;
            k = t >> 64;
        }

        result
    }

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

    pub fn select(&self, other: &Self, mask: u64) -> Self {
        let mut result = Self {
            x: [0x0, 0x0, 0x0, 0x0, 0x0],
        };

        let a = self.x.as_ref();
        let b = other.x.as_ref();

        for i in 0..LIMB_SZ {
            result.x[i] = (a[i] & !mask) | (b[i] & mask);
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

        let mut k: u128 = 0;
        for j in 0..LIMB_SZ {
            let sum = folded[j] + k;
            folded[j] = sum & MASK as u128;
            k = sum >> RADIX_POW;
        }

        folded[0] += 19 * k;
        k = 0;
        for j in 0..LIMB_SZ {
            let sum = folded[j] + k;
            folded[j] = sum & MASK as u128;
            k = sum >> RADIX_POW;
        }

        let mut x = [0u64; 5];
        for j in 0..LIMB_SZ {
            x[j] = folded[j] as u64;
        }
        let mut l = k as u64;

        x[0] += 19 * l;
        l = 0;
        for j in 0..LIMB_SZ {
            let sum = x[j] + l;
            x[j] = sum & MASK;
            l = sum >> RADIX_POW;
        }

        debug_assert_eq!(l, 0);

        let mut t = [0u64; LIMB_SZ];
        t[0] = x[0] + 19;
        l = t[0] >> RADIX_POW;
        t[0] &= MASK;

        for j in 1..LIMB_SZ {
            let sum = x[j] + l;
            t[j] = sum & MASK;
            l = sum >> RADIX_POW;
        }

        let mask = 0u64.wrapping_sub(l);
        let mut result = [0u64; LIMB_SZ];
        for j in 0..LIMB_SZ {
            result[j] = (x[j] & !mask) | (t[j] & mask);
        }

        Field25519 { x: result }
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
        let mut padded = vec![0u8; BYTES];
        padded[..rhs.len()].copy_from_slice(rhs);

        let mut r0: Self = INFINITY;
        let mut r1: Self = self.clone();

        for i in (0..FIELD_BITS).rev() {
            if test_bit(&padded, i) {
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

impl TryFrom<(BigUint, BigUint)> for Point25519 {
    type Error = &'static str;

    fn try_from((x, y): (BigUint, BigUint)) -> Result<Self, Self::Error> {
        let xf = Field25519::from(x.to_bytes_le());
        let yf = Field25519::from(y.to_bytes_le());
        let x2 = xf.sqr();
        let y2 = yf.sqr();

        let left = (x2 + y2).reduce();
        let right = (ONE + D * x2 * y2).reduce();

        if left != right {
            return Err("Point is not on curve");
        } else {
            Ok(Self {
                x: xf,
                y: yf,
                z: ONE,
                t: xf * yf,
            })
        }
    }
}

impl TryFrom<Vec<u8>> for Point25519 {
    type Error = &'static str;

    fn try_from(mut value: Vec<u8>) -> Result<Self, Self::Error> {
        let x_0 = value[31] >> 7;
        value[31] &= 0b01111111;

        let y = Field25519::from(value);
        if !(y < P) {
            return Err("y coordinate not a value mod p - invalid encoding");
        }

        let u = (y.sqr() - ONE).reduce();
        let v = D * y.sqr() + ONE;

        let v2 = v.sqr();
        let v3 = v2 * v;
        let v6 = v3.sqr();
        let v7 = v6 * v;

        let z = u * v7;
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
        let pow_p58 = z_250_0.sqr_n_times(2) * z;

        let x = u * v3 * pow_p58;
        let vx2 = v * x.sqr();

        let x = if vx2 == u {
            x
        } else if vx2 == u.neg() {
            x * SQRT
        } else {
            return Err("No square root exists for y mod p - invalid encoding");
        };

        if x_0 == 1 && x == ZERO {
            Err("x = 0 and x_0 = 1 - invalid encoding")
        } else {
            let x = if x_0 == (x.x[0] as u8 & 1) {
                x
            } else {
                x.neg()
            };

            Ok(Self {
                x,
                y,
                z: ONE,
                t: x * y,
            })
        }
    }
}

impl Point25519 {
    pub fn double(self) -> Self {
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

    pub fn is_infinity(&self) -> bool {
        self.x == ZERO && self.y == self.z && self.t == ZERO
    }
}

pub fn edwards25519_comb() -> &'static Ed25519Comb {
    ED25519_COMB.get_or_init(|| Ed25519Comb::new(4))
}

#[cfg(test)]
#[path = "unit_tests/edwards25519_test.rs"]
mod edwards25519_test;
