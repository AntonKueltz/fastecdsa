use crate::curve::{Curve, Field, Point};
use crate::scalar::ScalarField;

pub fn sign<C: Curve>(msg: &[u8], d_bytes: &[u8], k_bytes: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let p = (C::G * k_bytes).normalize();
    let r_bytes: Vec<u8> = Field::<C>::into(p.x);

    let k = C::Order::from_le_bytes(k_bytes);
    let z = C::Order::from_le_bytes(msg);
    let r = C::Order::from_le_bytes(&r_bytes);
    let d = C::Order::from_le_bytes(d_bytes);

    let kinv = k.invert();
    let s = kinv.mul(&z.add(&r.mul(&d)));

    (r.to_le_bytes(), s.to_le_bytes())
}

pub fn verify<C: Curve>(
    r_bytes: &[u8],
    s_bytes: &[u8],
    msg: &[u8],
    qx_bytes: &[u8],
    qy_bytes: &[u8],
) -> bool {
    let (Some(r), Some(s)) = (
        C::Order::from_le_bytes_checked(r_bytes),
        C::Order::from_le_bytes_checked(s_bytes),
    ) else {
        return false;
    };

    let q = Point::<C> {
        x: Field::<C>::from(qx_bytes),
        y: Field::<C>::from(qy_bytes),
        z: C::ONE,
    };
    let z = C::Order::from_le_bytes(msg);

    let sinv = s.invert();
    let u1 = z.mul(&sinv);
    let u2 = r.mul(&sinv);

    let p = Point::<C>::shamir(&C::G, &q, &u1.to_le_bytes(), &u2.to_le_bytes()).normalize();
    let x_bytes: Vec<u8> = Field::<C>::into(p.x);
    let xq = C::Order::from_le_bytes(&x_bytes);

    ScalarField::to_le_bytes(&xq) == ScalarField::to_le_bytes(&r)
}
