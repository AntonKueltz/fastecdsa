use std::mem::swap;
use std::ops::Add;
use std::str::FromStr;
use std::sync::LazyLock;

use num_bigint::BigInt;

use crate::edwards448::{G as Ed448G, INFINITY as Ed448INFINITY, Point448};
use crate::edwards25519::{G as Ed25519G, INFINITY as Ed25519INFINITY, Point25519};

const W_STATIC: usize = 8;
const W_VAR: usize = 5;

pub static ED25519_INTERLEAVED_WNAF_VERIFY: LazyLock<EdInterleavedWnafVerifier<Point25519>> =
    LazyLock::new(|| {
        let n = (BigInt::ONE << 252)
            + BigInt::from_str("27742317777372353535851937790883648493").unwrap();
        EdInterleavedWnafVerifier::new(n, Ed25519G, 3)
    });
pub static ED448_INTERLEAVED_WNAF_VERIFY: LazyLock<EdInterleavedWnafVerifier<Point448>> =
    LazyLock::new(|| {
        let n = (BigInt::ONE << 446)
            - BigInt::from_str(
                "13818066809895115352007386748515426880336692474882178609894547503885",
            )
            .unwrap();
        EdInterleavedWnafVerifier::new(n, Ed448G, 2)
    });

pub trait WnafVerifyPoint: Copy + Add<Output = Self> {
    fn identity() -> Self;
    fn double(&self) -> Self;
    fn negate(&self) -> Self;
    fn is_identity(&self) -> bool;
}

impl WnafVerifyPoint for Point25519 {
    fn identity() -> Self {
        Ed25519INFINITY
    }
    fn double(&self) -> Self {
        Point25519::double(*self)
    }
    fn negate(&self) -> Self {
        Point25519::neg(self)
    }
    fn is_identity(&self) -> bool {
        self.is_infinity()
    }
}

impl WnafVerifyPoint for Point448 {
    fn identity() -> Self {
        Ed448INFINITY
    }
    fn double(&self) -> Self {
        Point448::double(*self)
    }
    fn negate(&self) -> Self {
        Point448::neg(self)
    }
    fn is_identity(&self) -> bool {
        self.is_infinity()
    }
}

pub struct EdInterleavedWnafVerifier<P: WnafVerifyPoint> {
    n: BigInt,
    n2: BigInt,
    h: usize,
    cofactor_log2: usize,
    g_table: Vec<P>,
    ghi_table: Vec<P>,
}

impl<P: WnafVerifyPoint> EdInterleavedWnafVerifier<P> {
    pub fn new(n: BigInt, g: P, cofactor_log2: usize) -> Self {
        let h = (n.bits() / 2) as usize;
        let n2 = &n * &n;
        let mut g_hi: P = g;

        for _ in 0..h {
            g_hi = g_hi.double();
        }

        Self {
            g_table: lookup_table(&g, W_STATIC),
            ghi_table: lookup_table(&g_hi, W_STATIC),
            n,
            n2,
            h,
            cofactor_log2,
        }
    }

    pub fn verify(&self, s: &BigInt, k: &BigInt, r: &P, a: &P) -> bool {
        let (d0, d1) = lagrange(&self.n, &self.n2, k);
        let e = &d1 * s % &self.n;
        let e = if e < BigInt::ZERO { e + &self.n } else { e };
        let (e0, e1) = (&e & ((BigInt::ONE << self.h) - 1), &e >> self.h);

        let (e0_naf, e1_naf) = (naf(&e0, W_STATIC), naf(&e1, W_STATIC));
        let (d0_naf, d1_naf) = (signed_naf(&-d0, W_VAR), signed_naf(&-d1, W_VAR));
        let a_table = lookup_table(a, W_VAR);
        let r_table = lookup_table(r, W_VAR);

        let mut acc = interleaved(&[
            (&e0_naf, &self.g_table),
            (&e1_naf, &self.ghi_table),
            (&d0_naf, &a_table),
            (&d1_naf, &r_table),
        ]);

        for _ in 0..self.cofactor_log2 {
            acc = acc.double();
        }

        acc.is_identity()
    }
}

pub fn interleaved<P: WnafVerifyPoint>(terms: &[(&Vec<i16>, &Vec<P>)]) -> P {
    let len = terms.iter().map(|(n, _)| n.len()).max().unwrap_or(0);
    let mut acc = P::identity();

    for i in (0..len).rev() {
        acc = acc.double();

        for (naf, table) in terms {
            match naf.get(i).copied().unwrap_or(0) {
                0 => {}
                d if d > 0 => acc = acc + table[(d >> 1) as usize],
                d => acc = acc + table[(-d >> 1) as usize].negate(),
            }
        }
    }

    acc
}

fn lagrange(n: &BigInt, n2: &BigInt, k: &BigInt) -> (BigInt, BigInt) {
    let (mut nu, mut nv, mut p) = (n2.clone(), k * k + 1, n * k);
    let nbits = n.bits();
    let (r, t) = ((nbits + 4) >> 1, nbits + 1);
    let mod2r = &((BigInt::ONE << r) - 1);

    let (mut u0, mut u1) = (n & mod2r, BigInt::ZERO);
    let (mut v0, mut v1) = (k & mod2r, BigInt::ONE);

    loop {
        if nu < nv {
            swap(&mut u0, &mut v0);
            swap(&mut u1, &mut v1);
            swap(&mut nu, &mut nv);
        }

        let nvbits = nv.bits();
        if nvbits <= t {
            v0 = if &v0 >> (r - 1) > BigInt::ZERO {
                v0 - (BigInt::ONE << r)
            } else {
                v0
            };
            v1 = if &v1 >> (r - 1) > BigInt::ZERO {
                v1 - (BigInt::ONE << r)
            } else {
                v1
            };
            return (v0, v1);
        }

        let s = p.bits().saturating_sub(nvbits);
        if p > BigInt::ZERO {
            u0 = (u0 - (&v0 << s)) & mod2r;
            u1 = (u1 - (&v1 << s)) & mod2r;
            nu = nu + (&nv << (s << 1)) - (&p << (s + 1));
            p = p - (&nv << s);
        } else {
            u0 = (u0 + (&v0 << s)) & mod2r;
            u1 = (u1 + (&v1 << s)) & mod2r;
            nu = nu + (&nv << (s << 1)) + (&p << (s + 1));
            p = p + (&nv << s);
        }
    }
}

pub fn naf(x: &BigInt, w: usize) -> Vec<i16> {
    let mut result: Vec<i16> = Vec::<i16>::with_capacity(x.bits() as usize);
    let mut k = x.clone();

    let correction: i16 = 1 << w;
    let mask = BigInt::from(correction - 1);
    let limit: i16 = correction >> 1;

    while k >= BigInt::ONE {
        if k.bit(0) {
            let n: i16 = (&k & &mask).try_into().unwrap();
            let ki = if n < limit { n } else { n - correction };
            result.push(ki);
            k = k - BigInt::from(ki);
        } else {
            result.push(0);
        }

        k = k >> 1;
    }

    result
}

fn signed_naf(x: &BigInt, w: usize) -> Vec<i16> {
    if *x < BigInt::ZERO {
        let mut result = naf(&(-x), w);

        for xi in result.iter_mut() {
            *xi = -*xi;
        }

        result
    } else {
        naf(x, w)
    }
}

pub fn lookup_table<P: WnafVerifyPoint>(p: &P, w: usize) -> Vec<P> {
    let mut cur = *p;
    let two_p = p.double();
    let limit = 1usize << (w - 2);
    let mut table = Vec::<P>::with_capacity(limit);

    for i in 0..limit {
        table.push(cur);

        if i + 1 < limit {
            cur = cur + two_p;
        }
    }

    table
}

#[cfg(test)]
#[path = "unit_tests/wnaf_test.rs"]
mod wnaf_test;
