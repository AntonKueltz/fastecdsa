use crate::curve::{BrainpoolCurve, BrainpoolPoint, Curve, Point};
use crate::edwards25519::{G, INFINITY as ED25519_INFINITY, Point25519};
use crate::scalar::ScalarField;

pub struct Comb<C: Curve> {
    w: usize,
    d: usize,
    table: Vec<Point<C>>,
}

pub struct BrainpoolComb<C: BrainpoolCurve> {
    w: usize,
    d: usize,
    table: Vec<BrainpoolPoint<C>>,
}

pub struct Ed25519Comb {
    w: usize,
    d: usize,
    table: Vec<Point25519>,
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

impl<C: BrainpoolCurve> BrainpoolComb<C> {
    pub fn new(base: BrainpoolPoint<C>, w: usize) -> Self {
        assert!(w >= 1 && w <= 16, "window width out of range");

        let n = C::BITS as usize;
        let d = (n + w - 1) / w;

        let mut basis: Vec<BrainpoolPoint<C>> = Vec::with_capacity(w);
        let mut current = base;

        basis.push(current);
        for _ in 1..w {
            for _ in 0..d {
                current = current.double();
            }
            basis.push(current);
        }

        let size = 1usize << w;
        let mut table: Vec<BrainpoolPoint<C>> = Vec::with_capacity(size);
        table.push(C::INFINITY);
        for k in 1..size {
            let low_bit = k.trailing_zeros() as usize;
            let prev = k & (k - 1);
            table.push(table[prev] + basis[low_bit]);
        }

        let table: Vec<BrainpoolPoint<C>> = table.iter().map(|p| p.normalize()).collect();

        Self { w, d, table }
    }

    fn select(&self, index: usize) -> BrainpoolPoint<C> {
        let mut acc = self.table[0];

        for (i, entry) in self.table.iter().enumerate() {
            let mask = crate::comb::select_mask(i, index);

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
            if bit_pos < scalar.len() * 8 && crate::comb::test_bit(scalar, bit_pos) {
                index |= 1 << i;
            }
        }

        index
    }

    pub fn mul(&self, scalar_bytes: &[u8]) -> BrainpoolPoint<C> {
        let field_bytes = (C::BITS as usize + 7) / 8;
        assert!(
            scalar_bytes.len() <= field_bytes,
            "scalar too large for curve"
        );

        let mut padded = vec![0u8; field_bytes];
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

impl Ed25519Comb {
    pub fn new(w: usize) -> Self {
        assert!(w >= 1 && w <= 16, "window width out of range");

        let n = 253;
        let d = (n + w - 1) / w;

        let mut basis: Vec<Point25519> = Vec::with_capacity(w);
        let mut current = G;

        basis.push(current);
        for _ in 1..w {
            for _ in 0..d {
                current = current.double();
            }
            basis.push(current);
        }

        let size = 1usize << w;
        let mut table: Vec<Point25519> = Vec::with_capacity(size);
        table.push(ED25519_INFINITY);
        for k in 1..size {
            let low_bit = k.trailing_zeros() as usize;
            let prev = k & (k - 1);
            table.push(table[prev] + basis[low_bit]);
        }

        let table: Vec<Point25519> = table.iter().map(|p| p.normalize()).collect();

        Self { w, d, table }
    }

    fn select(&self, index: usize) -> Point25519 {
        let mut acc = self.table[0];

        for (i, entry) in self.table.iter().enumerate() {
            let mask = select_mask(i, index);

            acc.x = acc.x.select(&entry.x, mask);
            acc.y = acc.y.select(&entry.y, mask);
            acc.z = acc.z.select(&entry.z, mask);
            acc.t = acc.t.select(&entry.t, mask);
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

    pub fn mul(&self, scalar_bytes: &[u8]) -> Point25519 {
        let mut padded = vec![0u8; 32];
        padded[..scalar_bytes.len()].copy_from_slice(scalar_bytes);

        let mut acc = ED25519_INFINITY;
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
