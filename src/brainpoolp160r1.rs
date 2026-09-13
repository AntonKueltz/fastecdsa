use crypto_bigint::{U192, const_monty_params, modular::ConstMontyForm};

use crate::comb::BrainpoolComb;
use crate::curve::{BrainpoolCurve, BrainpoolPoint};

pub struct Brainpoolp160r1;

const_monty_params!(
    Brainpool160P,
    U192,
    "00000000E95E4A5F737059DC60DFC7AD95B3D8139515620F"
);
const_monty_params!(
    Brainpool160Q,
    U192,
    "00000000E95E4A5F737059DC60DF5991D45029409E60FC09"
);

static COMB: std::sync::OnceLock<BrainpoolComb<Brainpoolp160r1>> = std::sync::OnceLock::new();

impl BrainpoolCurve for Brainpoolp160r1 {
    type CurveField = ConstMontyForm<Brainpool160P, 3>;
    type GroupField = ConstMontyForm<Brainpool160Q, 3>;

    const BITS: u32 = 160;
    const ONE: Self::CurveField = ConstMontyForm::new(&U192::from_u8(0x1));

    const A: Self::CurveField = ConstMontyForm::new(&U192::from_be_hex(
        "00000000340E7BE2A280EB74E2BE61BADA745D97E8F7C300",
    ));
    const A_T: Self::CurveField = ConstMontyForm::new(&U192::from_be_hex(
        "00000000E95E4A5F737059DC60DFC7AD95B3D8139515620C",
    ));
    const B: Self::CurveField = ConstMontyForm::new(&U192::from_be_hex(
        "000000001E589A8595423412134FAA2DBDEC95C8D8675E58",
    ));
    const B_T: Self::CurveField = ConstMontyForm::new(&U192::from_be_hex(
        "000000007A556B6DAE535B7B51ED2C4D7DAA7A0B5C55F380",
    ));

    const Z2: Self::CurveField = ConstMontyForm::new(&U192::from_be_hex(
        "000000005F645BAF56B70A3B1B8FAD823CC1425AB27D95E3",
    ));
    const Z3: Self::CurveField = ConstMontyForm::new(&U192::from_be_hex(
        "00000000823797219341A6C5AFD9140868CC2F6DAE835455",
    ));
    const Z2_INV: Self::CurveField = ConstMontyForm::new(&U192::from_be_hex(
        "00000000BE6173A19C302EAB6755512B0334CEEC94697F50",
    ));
    const Z3_INV: Self::CurveField = ConstMontyForm::new(&U192::from_be_hex(
        "000000000951B8DAC11F0C4515955521EF84591C00544498",
    ));

    const G: BrainpoolPoint<Self> = BrainpoolPoint::<Self> {
        x: ConstMontyForm::new(&U192::from_be_hex(
            "00000000BED5AF16EA3F6A4F62938C4631EB5AF7BDBCDBC3",
        )),
        y: ConstMontyForm::new(&U192::from_be_hex(
            "000000001667CB477A1A8EC338F94741669C976316DA6321",
        )),
        z: ConstMontyForm::new(&U192::from_u8(0x1)),
    };
    const G_T: BrainpoolPoint<Self> = BrainpoolPoint::<Self> {
        x: ConstMontyForm::new(&U192::from_be_hex(
            "00000000B199B13B9B34EFC1397E64BAEB05ACC265FF2378",
        )),
        y: ConstMontyForm::new(&U192::from_be_hex(
            "00000000ADD6718B7C7C1961F0991B842443772152C9E0AD",
        )),
        z: ConstMontyForm::new(&U192::from_u8(0x1)),
    };
    const INFINITY: BrainpoolPoint<Self> = BrainpoolPoint::<Self> {
        x: ConstMontyForm::new(&U192::from_u8(0x0)),
        y: Self::ONE,
        z: ConstMontyForm::new(&U192::from_u8(0x0)),
    };

    fn comb() -> Option<&'static BrainpoolComb<Brainpoolp160r1>> {
        Some(COMB.get_or_init(|| BrainpoolComb::new(Brainpoolp160r1::G_T, 4)))
    }
}
