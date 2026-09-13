use crypto_bigint::{U256, const_monty_params, modular::ConstMontyForm};

use crate::comb::BrainpoolComb;
use crate::curve::{BrainpoolCurve, BrainpoolPoint};

pub struct Brainpoolp224r1;

const_monty_params!(
    Brainpool224P,
    U256,
    "00000000D7C134AA264366862A18302575D1D787B09F075797DA89F57EC8C0FF"
);
const_monty_params!(
    Brainpool224Q,
    U256,
    "00000000D7C134AA264366862A18302575D0FB98D116BC4B6DDEBCA3A5A7939F"
);

static COMB: std::sync::OnceLock<BrainpoolComb<Brainpoolp224r1>> = std::sync::OnceLock::new();

impl BrainpoolCurve for Brainpoolp224r1 {
    type CurveField = ConstMontyForm<Brainpool224P, { U256::LIMBS }>;
    type GroupField = ConstMontyForm<Brainpool224Q, { U256::LIMBS }>;

    const BITS: u32 = 224;
    const ONE: Self::CurveField = ConstMontyForm::new(&U256::from_u8(0x1));

    const A: Self::CurveField = ConstMontyForm::new(&U256::from_be_hex(
        "0000000068A5E62CA9CE6C1C299803A6C1530B514E182AD8B0042A59CAD29F43",
    ));
    const A_T: Self::CurveField = ConstMontyForm::new(&U256::from_be_hex(
        "00000000D7C134AA264366862A18302575D1D787B09F075797DA89F57EC8C0FC",
    ));
    const B: Self::CurveField = ConstMontyForm::new(&U256::from_be_hex(
        "000000002580F63CCFE44138870713B1A92369E33E2135D266DBB372386C400B",
    ));
    const B_T: Self::CurveField = ConstMontyForm::new(&U256::from_be_hex(
        "000000004B337D934104CD7BEF271BF60CED1ED20DA14C08B3BB64F18A60888D",
    ));

    const Z2: Self::CurveField = ConstMontyForm::new(&U256::from_be_hex(
        "00000000138C20C685DE0A8B241388E73F69176BF903E14EA91F7DA1663B86F5",
    ));
    const Z3: Self::CurveField = ConstMontyForm::new(&U256::from_be_hex(
        "00000000034488CAB5C245EFC24E3E0EC0676613A54D98F1B766D460B0313DAF",
    ));
    const Z2_INV: Self::CurveField = ConstMontyForm::new(&U256::from_be_hex(
        "000000000C603A056FC7C39B5CB098EC919E8688603BDAA27AE20071AB1C5D9A",
    ));
    const Z3_INV: Self::CurveField = ConstMontyForm::new(&U256::from_be_hex(
        "00000000B418B82969AE7D400D066633BBFE79F43207910FE098CE99A7D224A9",
    ));

    const G: BrainpoolPoint<Self> = BrainpoolPoint::<Self> {
        x: ConstMontyForm::new(&U256::from_be_hex(
            "000000000D9029AD2C7E5CF4340823B2A87DC68C9E4CE3174C1E6EFDEE12C07D",
        )),
        y: ConstMontyForm::new(&U256::from_be_hex(
            "0000000058AA56F772C0726F24C6B89E4ECDAC24354B9E99CAA3F6D3761402CD",
        )),
        z: ConstMontyForm::new(&U256::from_u8(0x1)),
    };
    const G_T: BrainpoolPoint<Self> = BrainpoolPoint::<Self> {
        x: ConstMontyForm::new(&U256::from_be_hex(
            "000000006AB1E344CE25FF3896424E7FFE14762ECB49F8928AC0C76029B4D580",
        )),
        y: ConstMontyForm::new(&U256::from_be_hex(
            "000000000374E9F5143E568CD23F3F4D7C0D4B1E41C8CC0D1C6ABD5F1A46DB4C",
        )),
        z: ConstMontyForm::new(&U256::from_u8(0x1)),
    };
    const INFINITY: BrainpoolPoint<Self> = BrainpoolPoint::<Self> {
        x: ConstMontyForm::new(&U256::from_u8(0x0)),
        y: Self::ONE,
        z: ConstMontyForm::new(&U256::from_u8(0x0)),
    };

    fn comb() -> Option<&'static BrainpoolComb<Brainpoolp224r1>> {
        Some(COMB.get_or_init(|| BrainpoolComb::new(Brainpoolp224r1::G_T, 4)))
    }
}
