use crypto_bigint::{
    modular::{ConstMontyForm, ConstMontyParams},
    Uint,
};

pub trait ScalarField: Copy + Sized {
    fn from_le_bytes(bytes: &[u8]) -> Self;

    fn from_le_bytes_checked(bytes: &[u8]) -> Option<Self>;

    fn to_le_bytes(&self) -> Vec<u8>;

    fn invert(&self) -> Self;

    fn add(&self, other: &Self) -> Self;

    fn mul(&self, other: &Self) -> Self;

    fn is_zero(&self) -> bool;

    fn eq(&self, other: &Self) -> bool;
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

    fn eq(&self, other: &Self) -> bool {
        *self == *other
    }
}
