use std::ops::Add;

use crate::brainpool_curve::{BrainpoolCurve, BrainpoolPoint};
use crate::edwards25519::{INFINITY as ED25519_INFINITY, Point25519};
use crate::scalar::ScalarField;
use crate::sec2_curve::{Point as Sec2Point, Sec2Curve};
use crate::util::{select_mask, test_bit};

pub trait CombPoint: Copy + Add<Output = Self> {
    const BITS: usize;

    fn infinity() -> Self;
    fn double(&self) -> Self;
    fn normalize(&self) -> Self;
    fn select(&self, other: &Self, mask: u64) -> Self;
}

pub struct Comb<P: CombPoint> {
    w: usize,
    d: usize,
    table: Box<[P]>,
}

impl<P: CombPoint> Comb<P> {
    const SCALAR_BYTES: usize = P::BITS.div_ceil(8);

    pub fn new(base: P, w: usize) -> Self {
        assert!((1..=16).contains(&w), "window width out of range");
        let d = P::BITS.div_ceil(w);

        let mut basis = Vec::with_capacity(w);
        let mut current = base;
        basis.push(current);
        for _ in 1..w {
            for _ in 0..d {
                current = current.double();
            }
            basis.push(current);
        }

        let mut table = Vec::with_capacity(1 << w);
        table.push(P::infinity());
        for k in 1..(1usize << w) {
            let low_bit = k.trailing_zeros() as usize;
            table.push(table[k & (k - 1)] + basis[low_bit]);
        }

        let table = table.iter().map(P::normalize).collect();
        Self { w, d, table }
    }

    fn select(&self, index: usize) -> P {
        let mut acc = self.table[0];

        for (i, entry) in self.table.iter().enumerate() {
            acc = acc.select(entry, select_mask(i, index));
        }

        acc
    }

    fn column_bits(&self, scalar: &[u8], col: usize) -> usize {
        let mut index = 0usize;

        for i in 0..self.w {
            let bit_pos = col + i * self.d;
            if bit_pos < scalar.len() * 8 {
                index |= (test_bit(scalar, bit_pos) as usize) << i;
            }
        }

        index
    }

    pub fn mul(&self, scalar: &[u8]) -> P {
        assert!(
            scalar.len() <= Self::SCALAR_BYTES,
            "scalar too large for curve"
        );

        let mut padded = vec![0u8; Self::SCALAR_BYTES];
        padded[..scalar.len()].copy_from_slice(scalar);

        let mut acc = P::infinity();
        for col in (0..self.d).rev() {
            acc = acc.double();
            let index = self.column_bits(&padded, col);
            let entry = self.select(index);
            acc = acc + entry;
        }

        acc
    }
}

impl<C: Sec2Curve> CombPoint for Sec2Point<C> {
    const BITS: usize = C::FIELD_BYTES * 8;

    fn double(&self) -> Self {
        Sec2Point::<C>::double(self)
    }

    fn infinity() -> Self {
        C::INFINITY
    }

    fn normalize(&self) -> Self {
        Sec2Point::<C>::normalize(&self)
    }

    fn select(&self, other: &Self, mask: u64) -> Self {
        let mut result = *self;

        result.x = self.x.select(&other.x, mask);
        result.y = self.y.select(&other.y, mask);
        result.z = self.z.select(&other.z, mask);

        result
    }
}

impl<C: BrainpoolCurve> CombPoint for BrainpoolPoint<C> {
    const BITS: usize = C::BITS as usize;

    fn double(&self) -> Self {
        BrainpoolPoint::<C>::double(self)
    }

    fn infinity() -> Self {
        C::INFINITY
    }

    fn normalize(&self) -> Self {
        BrainpoolPoint::<C>::normalize(self)
    }

    fn select(&self, other: &Self, mask: u64) -> Self {
        let mut result = *self;

        result.x = self.x.select(&other.x, mask);
        result.y = self.y.select(&other.y, mask);
        result.z = self.z.select(&other.z, mask);

        result
    }
}

impl CombPoint for Point25519 {
    const BITS: usize = 253;

    fn double(&self) -> Self {
        Point25519::double(*self)
    }

    fn infinity() -> Self {
        ED25519_INFINITY
    }

    fn normalize(&self) -> Self {
        Point25519::normalize(*self)
    }

    fn select(&self, other: &Self, mask: u64) -> Self {
        let mut result = *self;

        result.x = self.x.select(&other.x, mask);
        result.y = self.y.select(&other.y, mask);
        result.z = self.z.select(&other.z, mask);
        result.t = self.t.select(&other.t, mask);

        result
    }
}
