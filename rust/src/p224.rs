use std::ops::{Add, Mul, Sub};

use crypto_bigint::{const_monty_params, modular::ConstMontyForm, U256};
use pyo3::prelude::*;

const LIMBS: usize = 7;

struct P224AddResult {
    x: [u32; LIMBS + 1],
}
const P224_P_8: P224AddResult = P224AddResult {
    x: [
        0x00000001, 0x00000000, 0x00000000, 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff, 0x0,
    ],
};

struct P224MulResult {
    x: [u32; LIMBS << 1],
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct P224Element {
    x: [u32; LIMBS],
}
const P224_ONE: P224Element = P224Element {
    x: [0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
};
const P224_P: P224Element = P224Element {
    x: [
        0x00000001, 0x00000000, 0x00000000, 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff,
    ],
};
const P224_A: P224Element = P224Element {
    x: [
        0xfffffffe, 0xffffffff, 0xffffffff, 0xfffffffe, 0xffffffff, 0xffffffff, 0xffffffff,
    ],
};

#[derive(Clone, Copy, Debug, PartialEq)]
struct P224Point {
    x: P224Element,
    y: P224Element,
    z: P224Element,
}
const P224_G: P224Point = P224Point {
    x: P224Element {
        x: [
            0x115c1d21, 0x343280d6, 0x56c21122, 0x4a03c1d3, 0x321390b9, 0x6bb4bf7f, 0xb70e0cbd,
        ],
    },
    y: P224Element {
        x: [
            0x85007e34, 0x44d58199, 0x5a074764, 0xcd4375a0, 0x4c22dfe6, 0xb5f723fb, 0xbd376388,
        ],
    },
    z: P224_ONE,
};
const INFINITY: P224Point = P224Point {
    x: P224Element {
        x: [0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
    },
    y: P224Element {
        x: [0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
    },
    z: P224Element {
        x: [0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
    },
};

impl P224AddResult {
    fn less_than(&self, other: &P224AddResult) -> bool {
        for i in (0..LIMBS + 1).rev() {
            if self.x[i] < other.x[i] {
                return true;
            } else if self.x[i] > other.x[i] {
                return false;
            }
        }

        false
    }

    fn reduce(&mut self) -> P224Element {
        let mut t: i64;
        let mut k: i64;

        while !self.less_than(&P224_P_8) {
            t = self.x[0] as i64 - P224_P_8.x[0] as i64;
            self.x[0] = t as u32;
            k = t >> 32;

            t = self.x[1] as i64 - P224_P_8.x[1] as i64 + k;
            self.x[1] = t as u32;
            k = t >> 32;

            t = self.x[2] as i64 - P224_P_8.x[2] as i64 + k;
            self.x[2] = t as u32;
            k = t >> 32;

            t = self.x[3] as i64 - P224_P_8.x[3] as i64 + k;
            self.x[3] = t as u32;
            k = t >> 32;

            t = self.x[4] as i64 - P224_P_8.x[4] as i64 + k;
            self.x[4] = t as u32;
            k = t >> 32;

            t = self.x[5] as i64 - P224_P_8.x[5] as i64 + k;
            self.x[5] = t as u32;
            k = t >> 32;

            t = self.x[6] as i64 - P224_P_8.x[6] as i64 + k;
            self.x[6] = t as u32;
            k = t >> 32;

            t = self.x[7] as i64 + k;
            self.x[7] = t as u32;
        }

        P224Element {
            x: [
                self.x[0], self.x[1], self.x[2], self.x[3], self.x[4], self.x[5], self.x[6],
            ],
        }
    }
}

impl P224MulResult {
    fn reduce(&self) -> P224Element {
        let mut somewhat_reduced: P224AddResult = P224AddResult { x: [0; LIMBS + 1] };

        let mut sum: i64 = 0x2 + self.x[0] as i64 - self.x[7] as i64 - self.x[11] as i64;
        somewhat_reduced.x[0] = sum as u32;
        let mut carry: i64 = sum >> 32;

        sum = self.x[1] as i64 - self.x[8] as i64 - self.x[12] as i64 + carry;
        somewhat_reduced.x[1] = sum as u32;
        carry = sum >> 32;

        sum = self.x[2] as i64 - self.x[9] as i64 - self.x[13] as i64 + carry;
        somewhat_reduced.x[2] = sum as u32;
        carry = sum >> 32;

        sum = 0x1fffffffe + self.x[3] as i64 + self.x[7] as i64 + self.x[11] as i64
            - self.x[10] as i64
            + carry;
        somewhat_reduced.x[3] = sum as u32;
        carry = sum >> 32;

        sum = 0x1fffffffe + self.x[4] as i64 + self.x[8] as i64 + self.x[12] as i64
            - self.x[11] as i64
            + carry;
        somewhat_reduced.x[4] = sum as u32;
        carry = sum >> 32;

        sum = 0x1fffffffe + self.x[5] as i64 + self.x[9] as i64 + self.x[13] as i64
            - self.x[12] as i64
            + carry;
        somewhat_reduced.x[5] = sum as u32;
        carry = sum >> 32;

        sum = 0x1fffffffe + self.x[6] as i64 + self.x[10] as i64 - self.x[13] as i64 + carry;
        somewhat_reduced.x[6] = sum as u32;
        carry = sum >> 32;

        somewhat_reduced.x[7] = carry as u32;

        return somewhat_reduced.reduce();
    }
}

impl From<&[u8]> for P224Element {
    fn from(x: &[u8]) -> Self {
        let mut result: Self = Self { x: [0; LIMBS] };

        for i in 0..LIMBS {
            for j in 0..4 {
                result.x[i] |= (x[i * 4 + j] as u32) << (j * 8);
            }
        }

        result
    }
}

impl From<P224Element> for Vec<u8> {
    fn from(x: P224Element) -> Vec<u8> {
        let mut result: [u8; 28] = [0; 28];

        for i in 0..LIMBS {
            for j in 0..4 {
                result[i * 4 + j] = (x.x[i] >> (j * 8)) as u8;
            }
        }

        result.to_vec()
    }
}

// impl P224Element {
//     fn sqr(&self) -> Self {
//         let mut unreduced: P224MulResult = P224MulResult { x: [0; LIMBS << 1] };
//         let mut t: u128;
//         let mut k: usize;

//         for i in 0..LIMBS {
//             t = self.x[i] as u128 * self.x[i] as u128;
//             unreduced.sqr_helper(2 * i, t);

//             for j in (i + 1)..LIMBS {
//                 t = self.x[i] as u128 * self.x[j] as u128;
//                 k = i + j;

//                 unreduced.sqr_helper(k, t);
//                 unreduced.sqr_helper(k, t);
//             }
//         }

//         unreduced.reduce()
//     }

//     fn sqr_n_times(&self, n: usize) -> Self {
//         let mut result = self.clone();
//         for _ in 0..n {
//             result = result.sqr();
//         }
//         result
//     }
// }

#[cfg(test)]
#[path = "p224_test.rs"]
mod p224_test;
