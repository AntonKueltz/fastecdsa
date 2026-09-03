use crypto_bigint::{const_monty_params, modular::ConstMontyForm, U192};
use pyo3::prelude::*;

const LIMBS: usize = 3;

type P192Element = [u64; LIMBS];
type P192AddResult = [u64; LIMBS + 1];
type P192MulResult = [u64; LIMBS << 1];
type P192Point = (P192Element, P192Element, P192Element);

const P192_ONE: P192Element = [0x1, 0x0, 0x0];
const P192_P: P192Element = [0xffffffffffffffff, 0xfffffffffffffffe, 0xffffffffffffffff];
const P192_A: P192Element = [0xfffffffffffffffc, 0xfffffffffffffffe, 0xffffffffffffffff];
const P192_G: P192Point = (
    [0xf4ff0afd82ff1012, 0x7cbf20eb43a18800, 0x188da80eb03090f6],
    [0x73f977a11e794811, 0x631011ed6b24cdd5, 0x07192b95ffc8da78],
    P192_ONE,
);

const INFINITY: P192Point = ([0, 0, 0], [1, 0, 0], [0, 0, 0]);

const_monty_params!(
    P192Q,
    U192,
    "ffffffffffffffffffffffff99def836146bc9b1b4d22831"
);

#[inline]
fn p192_less_than(x: &P192AddResult, y: &P192AddResult) -> bool {
    for i in (0..LIMBS + 1).rev() {
        if x[i] < y[i] {
            return true;
        } else if x[i] > y[i] {
            return false;
        }
    }
    return false;
}

#[inline]
fn p192_reduce_add(c: &mut P192AddResult) -> P192Element {
    let p: P192AddResult = [P192_P[0], P192_P[1], P192_P[2], 0];
    let mut t: i128;
    let mut k: i128;

    while !p192_less_than(&c, &p) {
        t = c[0] as i128 - p[0] as i128;
        c[0] = t as u64;
        k = t >> 64;

        t = c[1] as i128 - p[1] as i128 + k;
        c[1] = t as u64;
        k = t >> 64;

        t = c[2] as i128 - p[2] as i128 + k;
        c[2] = t as u64;
        k = t >> 64;

        t = c[3] as i128 + k;
        c[3] = t as u64;
    }

    return [c[0], c[1], c[2]];
}

#[inline]
fn p192_reduce_mul(c: &P192MulResult) -> P192Element {
    let mut somewhat_reduced: P192AddResult = [0; LIMBS + 1];

    let mut sum: u128 = c[0] as u128 + c[3] as u128 + c[5] as u128;
    somewhat_reduced[0] = sum as u64;
    let mut carry: u64 = (sum >> 64) as u64;

    sum = c[1] as u128 + c[3] as u128 + c[4] as u128 + c[5] as u128 + carry as u128;
    somewhat_reduced[1] = sum as u64;
    carry = (sum >> 64) as u64;

    sum = c[2] as u128 + c[4] as u128 + c[5] as u128 + carry as u128;
    somewhat_reduced[2] = sum as u64;
    carry = (sum >> 64) as u64;
    somewhat_reduced[3] = carry;

    return p192_reduce_add(&mut somewhat_reduced);
}

fn p192_add(x: &P192Element, y: &P192Element) -> P192Element {
    let mut unreduced: P192AddResult = [0; LIMBS + 1];

    let mut t = x[0] as u128 + y[0] as u128;
    unreduced[0] = t as u64;
    let mut k = t >> 64;

    t = x[1] as u128 + y[1] as u128 + k;
    unreduced[1] = t as u64;
    k = t >> 64;

    t = x[2] as u128 + y[2] as u128 + k;
    unreduced[2] = t as u64;
    k = t >> 64;

    unreduced[3] = k as u64;

    return p192_reduce_add(&mut unreduced);
}

fn p192_sub(x: &P192Element, y: &P192Element) -> P192Element {
    let mut unreduced: P192AddResult = [0; LIMBS + 1];
    let mut t: i128;
    let mut k: i128;

    t = ((P192_P[0] as i128) << 1) - y[0] as i128 + x[0] as i128;
    unreduced[0] = t as u64;
    k = t >> 64;

    t = ((P192_P[1] as i128) << 1) - y[1] as i128 + x[1] as i128 + k;
    unreduced[1] = t as u64;
    k = t >> 64;

    t = ((P192_P[2] as i128) << 1) - y[2] as i128 + x[2] as i128 + k;
    unreduced[2] = t as u64;
    unreduced[3] = (t >> 64) as u64;

    return p192_reduce_add(&mut unreduced);
}

fn p192_mul(x: &P192Element, y: &P192Element) -> P192Element {
    let mut unreduced: P192MulResult = [0; LIMBS << 1];
    let mut k: usize;
    let mut t: u128;

    for i in 0..LIMBS {
        let mut carry: u128 = 0;

        for j in 0..LIMBS {
            k = i + j;
            t = x[i] as u128 * y[j] as u128 + unreduced[k] as u128 + carry;
            unreduced[k] = t as u64;
            carry = t >> 64;
        }

        unreduced[i + LIMBS] = carry as u64;
    }

    return p192_reduce_mul(&unreduced);
}

fn p192_mul_const(x: &P192Element, y: u64) -> P192Element {
    let mut unreduced: P192MulResult = [0; LIMBS << 1];
    let mut t: u128;
    let mut k: u128 = 0;

    for i in 0..LIMBS {
        t = x[i] as u128 * y as u128 + unreduced[i] as u128 + k;
        unreduced[i] = t as u64;
        k = t >> 64;
    }
    unreduced[LIMBS] = k as u64;

    return p192_reduce_mul(&unreduced);
}

fn p192_update_sqr(unreduced: &mut P192MulResult, k: usize, t: u128) {
    let lo = t as u64;
    let hi = (t >> 64) as u64;

    let (sumk, ck) = unreduced[k].overflowing_add(lo);
    unreduced[k] = sumk;

    let (sumk1, ck1a) = unreduced[k + 1].overflowing_add(hi);
    let (sumk1, ck1b) = sumk1.overflowing_add(ck as u64);
    unreduced[k + 1] = sumk1;

    let mut carry = (ck1a || ck1b) as u64;
    let mut l = k + 2;
    while carry != 0 {
        let (s, c) = unreduced[l].overflowing_add(carry);
        unreduced[l] = s;
        carry = c as u64;
        l += 1;
    }
}

fn p192_sqr(x: &P192Element) -> P192Element {
    let mut unreduced: P192MulResult = [0; LIMBS << 1];
    let mut t: u128;
    let mut k: usize;

    for i in 0..LIMBS {
        t = x[i] as u128 * x[i] as u128;
        p192_update_sqr(&mut unreduced, 2 * i, t);

        for j in (i + 1)..LIMBS {
            t = x[i] as u128 * x[j] as u128;
            k = i + j;

            p192_update_sqr(&mut unreduced, k, t);
            p192_update_sqr(&mut unreduced, k, t);
        }
    }

    return p192_reduce_mul(&unreduced);
}

#[inline]
fn p192_sqr_n_times(x: &P192Element, n: usize) -> P192Element {
    let mut result = x.clone();
    for _ in 0..n {
        result = p192_sqr(&result);
    }
    return result;
}

#[inline]
fn p192_pt_eq((px, py, pz): &P192Point, (qx, qy, qz): &P192Point) -> bool {
    return *px == *qx && *py == *qy && *pz == *qz;
}

fn p192_pt_normalize((x, y, z): &P192Point) -> P192Point {
    let z2 = p192_mul(&p192_sqr(&z), &z);
    let z3 = p192_mul(&p192_sqr(&z2), &z);
    let z6 = p192_mul(&p192_sqr_n_times(&z3, 3), &z3);
    let z7 = p192_mul(&p192_sqr(&z6), &z);
    let z14 = p192_mul(&p192_sqr_n_times(&z7, 7), &z7);
    let z15 = p192_mul(&p192_sqr(&z14), &z);
    let z30 = p192_mul(&p192_sqr_n_times(&z15, 15), &z15);
    let z31 = p192_mul(&p192_sqr(&z30), &z);
    let z62 = p192_mul(&p192_sqr_n_times(&z31, 31), &z31);
    let z63 = p192_mul(&p192_sqr(&z62), &z);
    let z126 = p192_mul(&p192_sqr_n_times(&z63, 63), &z63);
    let z127 = p192_mul(&p192_sqr(&z126), &z);

    let mut zinv = p192_sqr(&z127);
    zinv = p192_mul(&p192_sqr_n_times(&zinv, 62), &z62);
    zinv = p192_sqr(&zinv);
    zinv = p192_sqr(&zinv);
    zinv = p192_mul(&zinv, &z);

    let zinv2 = p192_sqr(&zinv);
    let zinv3 = p192_mul(&zinv2, &zinv);

    return (p192_mul(&x, &zinv2), p192_mul(&y, &zinv3), P192_ONE);
}

fn p192_pt_double(p @ (x1, y1, z1): &P192Point) -> P192Point {
    if p192_pt_eq(p, &INFINITY) {
        return INFINITY;
    }

    // https://hyperelliptic.org/EFD/g1p/auto-code/shortw/jacobian-3/doubling/dbl-2007-bl.op3
    let xx = p192_sqr(x1);
    let yy = p192_sqr(y1);
    let yyyy = p192_sqr(&yy);
    let zz = p192_sqr(z1);
    let t0 = p192_add(x1, &yy);
    let t1 = p192_sqr(&t0);
    let t2 = p192_sub(&t1, &xx);
    let t3 = p192_sub(&t2, &yyyy);
    let s = p192_mul_const(&t3, 2);
    let t4 = p192_sqr(&zz);
    let t5 = p192_mul(&P192_A, &t4);
    let t6 = p192_mul_const(&xx, 3);
    let m = p192_add(&t6, &t5);
    let t7 = p192_sqr(&m);
    let t8 = p192_mul_const(&s, 2);
    let t = p192_sub(&t7, &t8);
    let x3 = t;
    let t9 = p192_sub(&s, &t);
    let t10 = p192_mul_const(&yyyy, 8);
    let t11 = p192_mul(&m, &t9);
    let y3 = p192_sub(&t11, &t10);
    let t12 = p192_add(y1, z1);
    let t13 = p192_sqr(&t12);
    let t14 = p192_sub(&t13, &yy);
    let z3 = p192_sub(&t14, &zz);

    return (x3, y3, z3);
}

fn p192_pt_add(p @ (x1, y1, z1): &P192Point, q @ (x2, y2, z2): &P192Point) -> P192Point {
    if p192_pt_eq(p, &INFINITY) {
        return *q;
    } else if p192_pt_eq(q, &INFINITY) {
        return *p;
    } else if p192_pt_eq(p, q) {
        return p192_pt_double(p);
    }

    // https://hyperelliptic.org/EFD/g1p/auto-code/shortw/jacobian-3/addition/add-2007-bl.op3
    let z1z1 = p192_sqr(z1);
    let z2z2 = p192_sqr(z2);
    let u1 = p192_mul(x1, &z2z2);
    let u2 = p192_mul(x2, &z1z1);
    let t0 = p192_mul(y1, z2);
    let s1 = p192_mul(&t0, &z2z2);
    let t1 = p192_mul(y2, z1);
    let s2 = p192_mul(&t1, &z1z1);
    let h = p192_sub(&u2, &u1);
    let t2 = p192_mul_const(&h, 2);
    let i = p192_sqr(&t2);
    let j = p192_mul(&h, &i);
    let t3 = p192_sub(&s2, &s1);
    let r = p192_mul_const(&t3, 2);
    let v = p192_mul(&u1, &i);
    let t4 = p192_sqr(&r);
    let t5 = p192_mul_const(&v, 2);
    let t6 = p192_add(&j, &t5);
    let x3 = p192_sub(&t4, &t6);
    let t7 = p192_sub(&v, &x3);
    let t8 = p192_mul(&r, &t7);
    let t9 = p192_mul(&s1, &j);
    let t10 = p192_mul_const(&t9, 2);
    let y3 = p192_sub(&t8, &t10);
    let t11 = p192_add(z1, z2);
    let t12 = p192_sqr(&t11);
    let t13 = p192_sub(&t12, &z1z1);
    let t14 = p192_sub(&t13, &z2z2);
    let z3 = p192_mul(&t14, &h);

    (x3, y3, z3)
}

#[inline]
fn test_bit(n: &[u8], j: usize) -> bool {
    let byte = j >> 3;
    let bit = j & 0b111;
    return ((n[byte] >> bit) & 1) == 1;
}

fn p192_pt_mul(p: &P192Point, n: &[u8]) -> P192Point {
    if n.len() == 0 {
        return INFINITY;
    }

    let mut j = n.len() * 8 - 1;
    while !test_bit(n, j) {
        j -= 1;
    }

    let mut r0: P192Point = INFINITY;
    let mut r1: P192Point = p.clone();

    for i in (0..j + 1).rev() {
        if test_bit(n, i) {
            r0 = p192_pt_add(&r1, &r0);
            r1 = p192_pt_double(&r1);
        } else {
            r1 = p192_pt_add(&r0, &r1);
            r0 = p192_pt_double(&r0);
        }
    }

    return r0;
}

fn p192_shamir(p: &P192Point, q: &P192Point, n: &[u8], m: &[u8]) -> P192Point {
    let mut j = n.len() * 8 - 1;
    while !test_bit(n, j) && !test_bit(m, j) {
        j -= 1;
    }

    let pq = p192_pt_add(&p, &q);
    let mut r = INFINITY;

    for i in (0..j + 1).rev() {
        r = p192_pt_double(&r);

        if test_bit(n, i) && test_bit(m, i) {
            r = p192_pt_add(&r, &pq);
        } else if test_bit(n, i) {
            r = p192_pt_add(&r, &p);
        } else if test_bit(m, i) {
            r = p192_pt_add(&r, &q);
        }
    }

    return r;
}

fn p192_to_bytes(p: &[u64]) -> Vec<u8> {
    let mut result: [u8; 24] = [0; 24];

    for i in 0..3 {
        for j in 0..8 {
            result[i * 8 + j] = (p[i] >> (j * 8)) as u8;
        }
    }

    return result.to_vec();
}

fn bytes_to_p192(x: &[u8]) -> P192Element {
    let mut result: P192Element = [0; LIMBS];

    for i in 0..3 {
        for j in 0..8 {
            result[i] |= (x[i * 8 + j] as u64) << (j * 8);
        }
    }

    return result;
}

#[pyfunction]
pub fn p192_scale_point(n: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let (x, y, _) = p192_pt_normalize(&p192_pt_mul(&P192_G, n));
    return (p192_to_bytes(&x), p192_to_bytes(&y));
}

#[pyfunction]
pub fn p192_sign(msg: &[u8], d_bytes: &[u8], k_bytes: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let (r_p192, _, _) = p192_pt_normalize(&p192_pt_mul(&P192_G, &k_bytes));
    let r_bytes = &p192_to_bytes(&r_p192);

    let k = ConstMontyForm::<P192Q, 3>::new(&U192::from_le_slice(k_bytes));
    let z = ConstMontyForm::<P192Q, 3>::new(&U192::from_le_slice(msg));
    let r = ConstMontyForm::<P192Q, 3>::new(&U192::from_le_slice(r_bytes));
    let d = ConstMontyForm::<P192Q, 3>::new(&U192::from_le_slice(d_bytes));
    let kinv = k.invert().unwrap();
    let s = kinv * (z + r * d);

    return (
        r.retrieve().to_le_bytes().to_vec(),
        s.retrieve().to_le_bytes().to_vec(),
    );
}

#[pyfunction]
pub fn p192_verify(
    r_bytes: &[u8],
    s_bytes: &[u8],
    msg: &[u8],
    qx_bytes: &[u8],
    qy_bytes: &[u8],
) -> bool {
    let q: P192Point = (bytes_to_p192(&qx_bytes), bytes_to_p192(&qy_bytes), P192_ONE);
    let z = ConstMontyForm::<P192Q, 3>::new(&U192::from_le_slice(msg));
    let s = ConstMontyForm::<P192Q, 3>::new(&U192::from_le_slice(s_bytes));
    let r = ConstMontyForm::<P192Q, 3>::new(&U192::from_le_slice(r_bytes));
    let sinv = s.invert().unwrap();
    let u1 = z * sinv;
    let u2 = r * sinv;

    let (x, _, _) = p192_pt_normalize(&p192_shamir(
        &P192_G,
        &q,
        &u1.retrieve().to_le_bytes().to_vec(),
        &u2.retrieve().to_le_bytes().to_vec(),
    ));
    let xq = ConstMontyForm::<P192Q, 3>::new(&U192::from_le_slice(&p192_to_bytes(&x)));
    return xq == r;
}

#[cfg(test)]
#[path = "p192_test.rs"]
mod p192_test;
