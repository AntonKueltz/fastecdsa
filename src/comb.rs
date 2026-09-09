use std::sync::OnceLock;

use crate::curve::{Curve, Point};
use crate::p192::P192;
use crate::p224::P224;
use crate::p256::P256;
use crate::p384::P384;
use crate::p521::P521;

pub struct Comb<C: Curve> {
    w: usize,
    d: usize,
    table: Vec<Point<C>>,
}

impl<C: Curve> Comb<C> {
    pub fn new(base: Point<C>, w: usize) -> Self {
        assert!(w >= 1 && w <= 16, "window width out of range");

        let n = C::FIELD_BYTES * 8;
        let d = (n + w - 1) / w;

        let mut basis: Vec<Point<C>> = Vec::with_capacity(w);
        let mut current = base;

        basis.push(current);
        for _ in 1..w {
            for _ in 0..d {
                current = current.double();
            }
            basis.push(current);
        }

        let size = 1usize << w;
        let mut table: Vec<Point<C>> = Vec::with_capacity(size);
        table.push(C::INFINITY);
        for k in 1..size {
            let low_bit = k.trailing_zeros() as usize;
            let prev = k & (k - 1);
            table.push(table[prev] + basis[low_bit]);
        }

        let table: Vec<Point<C>> = table.iter().map(|p| p.normalize()).collect();

        Self { w, d, table }
    }

    fn select(&self, index: usize) -> Point<C> {
        let mut acc = self.table[0];

        for (i, entry) in self.table.iter().enumerate() {
            let mask = select_mask(i, index);

            acc.x = acc.x.select(&entry.x, mask);
            acc.y = acc.y.select(&entry.y, mask);
            acc.z = acc.z.select(&entry.z, mask);
        }

        acc
    }

    fn column_bits(&self, scalar: &[u8], col: usize) -> usize {
        let mut index = 0usize;

        for i in 0..self.w {
            let bit_pos = col + i * self.d;
            if bit_pos < scalar.len() * 8 && test_bit(scalar, bit_pos) {
                index |= 1 << i;
            }
        }

        index
    }

    pub fn mul(&self, scalar_bytes: &[u8]) -> Point<C> {
        let mut padded = vec![0u8; C::FIELD_BYTES];
        padded[..scalar_bytes.len()].copy_from_slice(scalar_bytes);

        let mut acc = C::INFINITY;
        for col in (0..self.d).rev() {
            acc = acc.double();
            let index = self.column_bits(&padded, col);
            let entry = self.select(index);
            acc = acc + entry;
        }

        acc
    }
}

#[inline]
fn select_mask(a: usize, b: usize) -> u64 {
    let diff = (a as i64) ^ (b as i64);
    let is_nonzero = (diff | diff.wrapping_neg()) >> 63;

    !(is_nonzero as u64)
}

#[inline]
fn test_bit(n: &[u8], j: usize) -> bool {
    let byte = j >> 3;
    let bit = j & 0b111;

    ((n[byte] >> bit) & 1) == 1
}

static P192_COMB: OnceLock<Comb<P192>> = OnceLock::new();
pub fn p192_comb() -> &'static Comb<P192> {
    P192_COMB.get_or_init(|| Comb::new(P192::G, 4))
}

static P224_COMB: OnceLock<Comb<P224>> = OnceLock::new();
pub fn p224_comb() -> &'static Comb<P224> {
    P224_COMB.get_or_init(|| Comb::new(P224::G, 4))
}

static P256_COMB: OnceLock<Comb<P256>> = OnceLock::new();
pub fn p256_comb() -> &'static Comb<P256> {
    P256_COMB.get_or_init(|| Comb::new(P256::G, 4))
}

static P384_COMB: OnceLock<Comb<P384>> = OnceLock::new();
pub fn p384_comb() -> &'static Comb<P384> {
    P384_COMB.get_or_init(|| Comb::new(P384::G, 4))
}

static P521_COMB: OnceLock<Comb<P521>> = OnceLock::new();
pub fn p521_comb() -> &'static Comb<P521> {
    P521_COMB.get_or_init(|| Comb::new(P521::G, 5))
}
