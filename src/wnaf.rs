use std::cmp::max;
use std::mem::swap;
use std::str::FromStr;
use std::sync::LazyLock;

use num_bigint::BigInt;

use crate::edwards25519::{G as Ed25519G, INFINITY as Ed25519INFINITY, Point25519};

static N_25519: LazyLock<BigInt> = LazyLock::new(|| {
    BigInt::from_str("7237005577332262213973186563042994240857116359379907606001950938285454250989")
        .unwrap()
});
static N2_25519: LazyLock<BigInt> = LazyLock::new(|| {
    BigInt::from_str("52374249726338269920211035149241586435867815353654972013472729036814800989261858502540231395529115798103940695844492802493701895840516084771691007478121").unwrap()
});

const H_25519: usize = 126;
static G_HI_25519: LazyLock<Point25519> =
    LazyLock::new(|| Ed25519G * &(1u128 << H_25519).to_le_bytes());

static G_TABLE_25519: LazyLock<Vec<Point25519>> = LazyLock::new(|| lookup_table(&Ed25519G, 8));
static G_HI_TABLE_25519: LazyLock<Vec<Point25519>> = LazyLock::new(|| lookup_table(&G_HI_25519, 8));

pub fn edwards25519_faster_verify_trick(
    s: &BigInt,
    k: &BigInt,
    r: &Point25519,
    a: &Point25519,
) -> bool {
    let (n, n2) = (&(*N_25519), &(*N2_25519));
    let (d0, d1) = lagrange(n, n2, k);
    let e = &d1 * s % n;
    let e = if e < BigInt::ZERO { e + n } else { e };
    let (e0, e1) = (&e & ((BigInt::ONE << H_25519) - BigInt::ONE), &e >> H_25519);

    let e0_naf = naf(&e0, 8);
    let e1_naf = naf(&e1, 8);
    let d0_naf = signed_naf(&(-d0), 5);
    let d1_naf = signed_naf(&(-d1), 5);

    let r_table = lookup_table(r, 5);
    let a_table = lookup_table(a, 5);

    let len = max(
        max(e0_naf.len(), e1_naf.len()),
        max(d0_naf.len(), d1_naf.len()),
    );
    let mut acc = Ed25519INFINITY;
    let naf2table = [
        (&e0_naf, &(*G_TABLE_25519)),
        (&e1_naf, &(*G_HI_TABLE_25519)),
        (&d0_naf, &a_table),
        (&d1_naf, &r_table),
    ];

    for i in (0..len).rev() {
        acc = acc.double();

        for (naf, table) in naf2table {
            let d = match naf.get(i) {
                Some(x) => *x,
                None => 0,
            };

            if d > 0 {
                acc = acc + table[(d >> 1) as usize];
            } else if d < 0 {
                acc = acc + table[(-d >> 1) as usize].neg();
            }
        }
    }

    acc.double().double().double().is_infinity()
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

fn naf(x: &BigInt, w: usize) -> Vec<i16> {
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

fn lookup_table(p: &Point25519, w: usize) -> Vec<Point25519> {
    let mut cur = *p;
    let two_p = p.double();
    let limit = 1usize << (w - 2);
    let mut table = Vec::<Point25519>::with_capacity(limit);

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
