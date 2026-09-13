use std::ops::{Add, Mul, Sub};

use crypto_bigint::{
    Choice, CtSelect, Uint,
    modular::{ConstMontyForm, ConstMontyParams},
};

pub trait ScalarField:
    Copy + PartialEq + Sized + Add<Output = Self> + Mul<Output = Self> + Sub<Output = Self>
{
    fn from_le_bytes(bytes: &[u8]) -> Self;

    fn from_le_bytes_checked(bytes: &[u8]) -> Option<Self>;

    fn to_le_bytes(&self) -> Vec<u8>;

    fn invert(&self) -> Self;

    fn add(&self, other: &Self) -> Self;

    fn mul(&self, other: &Self) -> Self;

    fn is_zero(&self) -> bool;

    fn test_bit(&self, i: u32) -> bool;

    fn sqr(&self) -> Self;

    fn select(&self, other: &Self, mask: u64) -> Self;
}

impl<MOD, const LIMBS: usize> ScalarField for ConstMontyForm<MOD, LIMBS>
where
    MOD: ConstMontyParams<LIMBS>,
{
    fn from_le_bytes(bytes: &[u8]) -> Self {
        let mut padded = vec![0u8; Uint::<LIMBS>::BYTES];
        padded[..bytes.len()].copy_from_slice(bytes);
        Self::new(&Uint::<LIMBS>::from_le_slice(&padded))
    }

    fn from_le_bytes_checked(bytes: &[u8]) -> Option<Self> {
        let mut padded = vec![0u8; Uint::<LIMBS>::BYTES];
        padded[..bytes.len()].copy_from_slice(bytes);
        let raw = Uint::<LIMBS>::from_le_slice(&padded);

        let modulus = MOD::PARAMS.modulus().as_ref();
        if raw.is_zero().into() || &raw >= modulus {
            return None;
        }
        Some(Self::new(&raw))
    }

    fn to_le_bytes(&self) -> Vec<u8> {
        self.retrieve().to_le_bytes().to_vec()
    }

    fn invert(&self) -> Self {
        ConstMontyForm::invert(self).unwrap()
    }

    fn add(&self, other: &Self) -> Self {
        *self + *other
    }

    fn mul(&self, other: &Self) -> Self {
        *self * *other
    }

    fn is_zero(&self) -> bool {
        bool::from(self.retrieve().is_zero())
    }

    fn test_bit(&self, i: u32) -> bool {
        self.retrieve().bit(i).into()
    }

    fn sqr(&self) -> Self {
        self.square()
    }

    fn select(&self, other: &Self, mask: u64) -> Self {
        let choice = Choice::from((mask & 1) as u8);
        let selected = Uint::ct_select(self.as_montgomery(), other.as_montgomery(), choice);

        Self::from_montgomery(selected)
    }
}
