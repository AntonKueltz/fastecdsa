use std::cmp::Ordering::{Equal, Greater, Less};
use std::ops::{Add, Mul, Sub};
use std::sync::OnceLock;

use num_bigint::BigUint;

use crate::comb::Comb;
use crate::util::test_bit;

const BYTES: usize = 56;
const FIELD_BITS: u32 = 448;
const LIMB_SZ: usize = 8;
const HALF_LIMBS: usize = 4;
const INTERMEDIATE_LIMBS: usize = 11;
const RADIX_POW: u32 = 56;
const LIMIT: u64 = 1 << RADIX_POW;
const MASK: u64 = LIMIT - 1;

#[derive(Clone, Copy)]
struct Half448 {
    x: [u64; HALF_LIMBS],
}

#[derive(Copy, Clone, Debug)]
pub struct Field448 {
    x: [u64; LIMB_SZ],
}

#[derive(Clone, Copy, Debug)]
pub struct Point448 {
    pub x: Field448,
    pub y: Field448,
    pub z: Field448,
}

pub const P: Field448 = Field448 {
    x: [
        0x00ffffffffffffff,
        0x00ffffffffffffff,
        0x00ffffffffffffff,
        0x00ffffffffffffff,
        0x00fffffffffffffe,
        0x00ffffffffffffff,
        0x00ffffffffffffff,
        0x00ffffffffffffff,
    ],
};
const SCALED_P: Field448 = Field448 {
    x: [
        2 * (2u64.pow(RADIX_POW) - 1),
        2 * (2u64.pow(RADIX_POW) - 1),
        2 * (2u64.pow(RADIX_POW) - 1),
        2 * (2u64.pow(RADIX_POW) - 1),
        2 * (2u64.pow(RADIX_POW) - 2),
        2 * (2u64.pow(RADIX_POW) - 1),
        2 * (2u64.pow(RADIX_POW) - 1),
        2 * (2u64.pow(RADIX_POW) - 1),
    ],
};
const D: Field448 = Field448 {
    x: [
        0x00ffffffffff6756,
        0x00ffffffffffffff,
        0x00ffffffffffffff,
        0x00ffffffffffffff,
        0x00fffffffffffffe,
        0x00ffffffffffffff,
        0x00ffffffffffffff,
        0x00ffffffffffffff,
    ],
};
const ZERO: Field448 = Field448 {
    x: [0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
};
const ONE: Field448 = Field448 {
    x: [0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
};

pub const G: Point448 = Point448 {
    x: Field448 {
        x: [
            0x0026a82bc70cc05e,
            0x0080e18b00938e26,
            0x00f72ab66511433b,
            0x00a3d3a46412ae1a,
            0x000f1767ea6de324,
            0x0036da9e14657047,
            0x00ed221d15a622bf,
            0x004f1970c66bed0d,
        ],
    },
    y: Field448 {
        x: [
            0x0008795bf230fa14,
            0x00132c4ed7c8ad98,
            0x001ce67c39c4fdbd,
            0x0005a0c2d73ad3ff,
            0x00a3984087789c1e,
            0x00c7624bea73736c,
            0x00248876203756c9,
            0x00693f46716eb6bc,
        ],
    },
    z: ONE,
};
pub const INFINITY: Point448 = Point448 {
    x: ZERO,
    y: ONE,
    z: ONE,
};

static ED448_COMB: OnceLock<Comb<Point448>> = OnceLock::new();

impl PartialEq for Field448 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
    }
}

impl PartialOrd for Field448 {
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

impl Mul for Half448 {
    type Output = [u128; LIMB_SZ - 1];

    fn mul(self, rhs: Self) -> [u128; LIMB_SZ - 1] {
        let mut result: [u128; LIMB_SZ - 1] = [0; LIMB_SZ - 1];

        for i in 0..HALF_LIMBS {
            for j in 0..HALF_LIMBS {
                result[i + j] += self.x[i] as u128 * rhs.x[j] as u128;
            }
        }

        result
    }
}

impl Add for Half448 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut result = Self { x: [0; HALF_LIMBS] };

        for i in 0..HALF_LIMBS {
            result.x[i] += self.x[i] + rhs.x[i];
        }

        result
    }
}

impl Half448 {
    fn sqr(&self) -> [u128; LIMB_SZ - 1] {
        let mut result: [u128; LIMB_SZ - 1] = [0; LIMB_SZ - 1];

        for i in 0..HALF_LIMBS {
            result[2 * i] += self.x[i] as u128 * self.x[i] as u128;
        }

        for i in 0..HALF_LIMBS {
            for j in i + 1..HALF_LIMBS {
                result[i + j] += (self.x[i] as u128 * self.x[j] as u128) << 1;
            }
        }

        result
    }
}

impl Add for Field448 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut result = Self { x: [0; LIMB_SZ] };

        for j in 0..LIMB_SZ {
            result.x[j] = self.x[j] + rhs.x[j];
        }

        result
    }
}

impl Sub for Field448 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let mut result = Field448 { x: [0; LIMB_SZ] };

        for j in 0..LIMB_SZ {
            result.x[j] = SCALED_P.x[j] + self.x[j] - rhs.x[j];
        }

        result
    }
}

impl Mul for Field448 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let a = Half448 {
            x: [self.x[0], self.x[1], self.x[2], self.x[3]],
        };
        let b = Half448 {
            x: [self.x[4], self.x[5], self.x[6], self.x[7]],
        };
        let c = Half448 {
            x: [rhs.x[0], rhs.x[1], rhs.x[2], rhs.x[3]],
        };
        let d = Half448 {
            x: [rhs.x[4], rhs.x[5], rhs.x[6], rhs.x[7]],
        };

        let p0: [u128; LIMB_SZ - 1] = a * c;
        let p1: [u128; LIMB_SZ - 1] = b * d;
        let p2: [u128; LIMB_SZ - 1] = (a + b) * (c + d);

        let mut intermediate: [i128; INTERMEDIATE_LIMBS] = [0; INTERMEDIATE_LIMBS];
        for j in 0..LIMB_SZ - 1 {
            intermediate[j] += p0[j] as i128 + p1[j] as i128;
            intermediate[j + HALF_LIMBS] += p2[j] as i128 - p0[j] as i128;
        }

        Field448::from(intermediate.to_vec())
    }
}

impl Field448 {
    pub fn reduce(self) -> Self {
        let mut x = self.x;

        for _ in 0..2 {
            let mut k = 0u64;
            for i in 0..LIMB_SZ {
                let s = x[i] + k;
                x[i] = s & MASK;
                k = s >> RADIX_POW;
            }
            x[0] += k;
            x[4] += k;
        }

        let mut t = x;
        t[0] += 1;
        t[4] += 1;
        let mut k = 0u64;
        for i in 0..LIMB_SZ {
            let s = t[i] + k;
            t[i] = s & MASK;
            k = s >> RADIX_POW;
        }

        Field448 { x }.select(&Field448 { x: t }, k.wrapping_neg())
    }

    pub fn neg(&self) -> Self {
        let mut result = Self { x: [0; LIMB_SZ] };

        let mut t: i128;
        let mut k: i128 = 0;

        for j in 0..LIMB_SZ {
            t = P.x[j] as i128 - self.x[j] as i128 + k;
            result.x[j] = (t as u64) & MASK;
            k = t >> RADIX_POW;
        }

        result
    }

    fn sqr(&self) -> Self {
        let a = Half448 {
            x: [self.x[0], self.x[1], self.x[2], self.x[3]],
        };
        let b = Half448 {
            x: [self.x[4], self.x[5], self.x[6], self.x[7]],
        };

        let p0: [u128; LIMB_SZ - 1] = a.sqr();
        let p1: [u128; LIMB_SZ - 1] = b.sqr();
        let p2: [u128; LIMB_SZ - 1] = (a + b).sqr();

        let mut intermediate: [i128; INTERMEDIATE_LIMBS] = [0; INTERMEDIATE_LIMBS];
        for j in 0..LIMB_SZ - 1 {
            intermediate[j] += p0[j] as i128 + p1[j] as i128;
            intermediate[j + HALF_LIMBS] += p2[j] as i128 - p0[j] as i128;
        }

        Field448::from(intermediate.to_vec())
    }

    fn sqr_n_times(self, n: usize) -> Self {
        let mut result = self;

        for _ in 0..n {
            result = result.sqr()
        }

        result
    }

    pub fn select(&self, other: &Self, mask: u64) -> Self {
        let mut result = ZERO;

        let a = self.x.as_ref();
        let b = other.x.as_ref();

        for i in 0..LIMB_SZ {
            result.x[i] = (a[i] & !mask) | (b[i] & mask);
        }

        result
    }
}

impl From<Vec<i128>> for Field448 {
    fn from(value: Vec<i128>) -> Self {
        let mut carry128: i128 = 0;
        let mut unreduced = [0; INTERMEDIATE_LIMBS + 1];

        for j in 0..INTERMEDIATE_LIMBS {
            let sum = carry128 + value[j];
            unreduced[j] = sum as u64 & MASK;
            carry128 = sum >> RADIX_POW;
        }
        unreduced[INTERMEDIATE_LIMBS] = carry128 as u64;

        for j in 8..INTERMEDIATE_LIMBS + 1 {
            unreduced[j - 4] += unreduced[j];
            unreduced[j - 8] += unreduced[j];
            unreduced[j] = 0;
        }

        let mut result = Field448 { x: [0; LIMB_SZ] };
        let mut carry64: u64 = 0;
        for j in 0..LIMB_SZ {
            let sum = carry64 + unreduced[j];
            result.x[j] = sum & MASK;
            carry64 = sum >> RADIX_POW;
        }

        result.x[0] += carry64;
        result.x[4] += carry64;

        carry64 = 0;
        for j in 0..LIMB_SZ {
            let sum = carry64 + result.x[j];
            result.x[j] = sum & MASK;
            carry64 = sum >> RADIX_POW;
        }

        result.x[0] += carry64;
        result.x[4] += carry64;

        result
    }
}

impl From<Vec<u8>> for Field448 {
    fn from(value: Vec<u8>) -> Self {
        let mut result = Self { x: [0; LIMB_SZ] };

        for i in 0..LIMB_SZ {
            for j in 0..7 {
                result.x[i] |= (value[i * 7 + j] as u64) << (j * 8);
            }
        }

        result
    }
}

impl From<Field448> for [u8; BYTES + 1] {
    fn from(value: Field448) -> [u8; BYTES + 1] {
        let mut bytes: [u8; BYTES + 1] = [0; BYTES + 1];

        for i in 0..LIMB_SZ {
            for j in 0..7 {
                bytes[i * 7 + j] = (value.x[i] >> (j * 8)) as u8;
            }
        }

        bytes
    }
}

impl Add for Point448 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let x1 = self.x;
        let y1 = self.y;
        let z1 = self.z;
        let x2 = rhs.x;
        let y2 = rhs.y;
        let z2 = rhs.z;

        let a = z1 * z2;
        let b = a.sqr();
        let c = x1 * x2;
        let d = y1 * y2;
        let e = D * c * d;
        let f = b - e;
        let g = b + e;
        let h = (x1 + y1) * (x2 + y2);

        Self {
            x: a * f * (h - c - d),
            y: a * g * (d - c),
            z: f * g,
        }
    }
}

impl Mul<&[u8]> for Point448 {
    type Output = Self;

    fn mul(self, rhs: &[u8]) -> Self {
        let mut padded = vec![0u8; BYTES];
        padded[..rhs.len()].copy_from_slice(rhs);

        let mut r0: Self = INFINITY;
        let mut r1: Self = self.clone();

        for i in (0..FIELD_BITS as usize).rev() {
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

impl From<Point448> for [u8; BYTES + 1] {
    fn from(value: Point448) -> Self {
        let mut bytes: [u8; BYTES + 1] = value.y.into();
        let sign: u8 = value.x.x[0] as u8 & 1;
        bytes[BYTES] |= sign << 7;

        bytes
    }
}

impl TryFrom<(BigUint, BigUint)> for Point448 {
    type Error = &'static str;

    fn try_from((x, y): (BigUint, BigUint)) -> Result<Self, Self::Error> {
        let xf = Field448::from(x.to_bytes_le());
        let yf = Field448::from(y.to_bytes_le());
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
            })
        }
    }
}

impl TryFrom<Vec<u8>> for Point448 {
    type Error = &'static str;

    fn try_from(mut value: Vec<u8>) -> Result<Self, Self::Error> {
        let x_0 = value[56] >> 7;
        value[56] &= 0b01111111;
        let hi_bits = value[56];

        let y = Field448::from(value);
        if hi_bits != 0 || !(y < P) {
            return Err("y coordinate not a value mod p - invalid encoding");
        }

        let y2 = y.sqr();
        let u = (y2 - ONE).reduce();
        let v = D * y2 - ONE;

        let u2 = u.sqr();
        let u3 = u2 * u;
        let u5 = u3 * u2;
        let v2 = v.sqr();
        let v3 = v2 * v;

        let z = u5 * v3;
        let z2 = z.sqr();
        let z3 = z2 * z;
        let z6 = z3.sqr();
        let z7 = z6 * z;
        let z_6_0 = z7.sqr_n_times(3) * z7;

        let z_12_0 = z_6_0.sqr_n_times(6) * z_6_0;
        let z_24_0 = z_12_0.sqr_n_times(12) * z_12_0;
        let i34 = z_24_0.sqr_n_times(6);
        let z_30_0 = i34 * z_6_0;
        let z_48_0 = i34.sqr_n_times(18) * z_24_0;
        let z_96_0 = z_48_0.sqr_n_times(48) * z_48_0;
        let z_192_0 = z_96_0.sqr_n_times(96) * z_96_0;
        let z_222_0 = z_192_0.sqr_n_times(30) * z_30_0;
        let z_223_0 = z_222_0.sqr() * z;
        let pow_p34 = z_223_0.sqr_n_times(223) * z_222_0;

        let x = u3 * v * pow_p34;
        let vx2 = v * x.sqr();

        if vx2 != u {
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

            Ok(Self { x, y, z: ONE })
        }
    }
}

impl Point448 {
    pub fn double(self) -> Self {
        let x1 = self.x;
        let y1 = self.y;
        let z1 = self.z;

        let b = (x1 + y1).sqr();
        let c = x1.sqr();
        let d = y1.sqr();
        let e = c + d;
        let h = z1.sqr();
        let j = e - (h + h);

        Self {
            x: (b - e) * j,
            y: e * (c - d),
            z: e * j,
        }
    }

    pub fn normalize(self) -> Self {
        let z = self.z;
        let z2 = z.sqr();
        let z3 = z2 * z;
        let z6 = z3.sqr();
        let z7 = z6 * z;
        let z_6_0 = z7.sqr_n_times(3) * z7;

        let z_12_0 = z_6_0.sqr_n_times(6) * z_6_0;
        let z_24_0 = z_12_0.sqr_n_times(12) * z_12_0;
        let i34 = z_24_0.sqr_n_times(6);
        let z_30_0 = i34 * z_6_0;
        let z_48_0 = i34.sqr_n_times(18) * z_24_0;
        let z_96_0 = z_48_0.sqr_n_times(48) * z_48_0;
        let z_192_0 = z_96_0.sqr_n_times(96) * z_96_0;
        let z_222_0 = z_192_0.sqr_n_times(30) * z_30_0;
        let z_223_0 = z_222_0.sqr() * z;

        let zinv = (z_223_0.sqr_n_times(223) * z_222_0).sqr_n_times(2) * z;

        Self {
            x: self.x * zinv,
            y: self.y * zinv,
            z: ONE,
        }
    }

    pub fn is_infinity(&self) -> bool {
        self.x.reduce() == ZERO && self.y.reduce() == self.z.reduce()
    }
}

pub fn edwards448_comb() -> &'static Comb<Point448> {
    ED448_COMB.get_or_init(|| Comb::<Point448>::new(G, 5))
}

#[cfg(test)]
#[path = "unit_tests/edwards448_test.rs"]
mod edwards448_test;
