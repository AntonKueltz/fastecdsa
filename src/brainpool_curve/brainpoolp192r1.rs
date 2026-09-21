use std::sync::OnceLock;

use crypto_bigint::{U192, const_monty_params, modular::ConstMontyForm};

use crate::brainpool_curve::{BrainpoolCurve, BrainpoolPoint};
use crate::comb::Comb;
use crate::wnaf::lookup_table;

pub struct Brainpoolp192r1;

const_monty_params!(
    Brainpool192P,
    U192,
    "C302F41D932A36CDA7A3463093D18DB78FCE476DE1A86297"
);
const_monty_params!(
    Brainpool192Q,
    U192,
    "C302F41D932A36CDA7A3462F9E9E916B5BE8F1029AC4ACC1"
);

static COMB: OnceLock<Comb<BrainpoolPoint<Brainpoolp192r1>>> = OnceLock::new();
static G_TABLE: OnceLock<Vec<BrainpoolPoint<Brainpoolp192r1>>> = OnceLock::new();

impl BrainpoolCurve for Brainpoolp192r1 {
    type CurveField = ConstMontyForm<Brainpool192P, { U192::LIMBS }>;
    type GroupField = ConstMontyForm<Brainpool192Q, { U192::LIMBS }>;

    const BITS: u32 = 192;
    const ONE: Self::CurveField = ConstMontyForm::new(&U192::from_u8(0x1));

    const A: Self::CurveField = ConstMontyForm::new(&U192::from_be_hex(
        "6A91174076B1E0E19C39C031FE8685C1CAE040E5C69A28EF",
    ));
    const A_T: Self::CurveField = ConstMontyForm::new(&U192::from_be_hex(
        "C302F41D932A36CDA7A3463093D18DB78FCE476DE1A86294",
    ));
    const B: Self::CurveField = ConstMontyForm::new(&U192::from_be_hex(
        "469A28EF7C28CCA3DC721D044F4496BCCA7EF4146FBF25C9",
    ));
    const B_T: Self::CurveField = ConstMontyForm::new(&U192::from_be_hex(
        "13D56FFAEC78681E68F9DEB43B35BEC2FB68542E27897B79",
    ));

    const Z2: Self::CurveField = ConstMontyForm::new(&U192::from_be_hex(
        "9B5453F7B83731569F035FE602706EB6D1A8D560AD98C6BD",
    ));
    const Z3: Self::CurveField = ConstMontyForm::new(&U192::from_be_hex(
        "183C49EEB3A4C86B18830D927586517B20E118BAF9D682A9",
    ));
    const Z2_INV: Self::CurveField = ConstMontyForm::new(&U192::from_be_hex(
        "A1F16DCD748BC551D43EAE8A59E0FB3A9AB6546776DB9FA9",
    ));
    const Z3_INV: Self::CurveField = ConstMontyForm::new(&U192::from_be_hex(
        "B2B68E70B2DDA7B2892289F3C6F59D29B2D805634155CE74",
    ));

    const G: BrainpoolPoint<Self> = BrainpoolPoint::<Self> {
        x: ConstMontyForm::new(&U192::from_be_hex(
            "C0A0647EAAB6A48753B033C56CB0F0900A2F5C4853375FD6",
        )),
        y: ConstMontyForm::new(&U192::from_be_hex(
            "14B690866ABD5BB88B5F4828C1490002E6773FA2FA299B8F",
        )),
        z: ConstMontyForm::new(&U192::from_u8(0x1)),
    };
    const G_T: BrainpoolPoint<Self> = BrainpoolPoint::<Self> {
        x: ConstMontyForm::new(&U192::from_be_hex(
            "3AE9E58C82F63C30282E1FE7BBF43FA72C446AF6F4618129",
        )),
        y: ConstMontyForm::new(&U192::from_be_hex(
            "097E2C5667C2223A902AB5CA449D0084B7E5B3DE7CCC01C9",
        )),
        z: ConstMontyForm::new(&U192::from_u8(0x1)),
    };
    const INFINITY: BrainpoolPoint<Self> = BrainpoolPoint::<Self> {
        x: ConstMontyForm::new(&U192::from_u8(0x0)),
        y: Self::ONE,
        z: ConstMontyForm::new(&U192::from_u8(0x0)),
    };

    fn comb() -> Option<&'static Comb<BrainpoolPoint<Brainpoolp192r1>>> {
        Some(
            COMB.get_or_init(|| {
                Comb::<BrainpoolPoint<Brainpoolp192r1>>::new(Brainpoolp192r1::G_T, 4)
            }),
        )
    }

    fn g_table() -> &'static Vec<BrainpoolPoint<Self>> {
        G_TABLE.get_or_init(|| lookup_table(&Self::G_T, 8))
    }
}
