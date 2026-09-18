use crypto_bigint::{U320, const_monty_params, modular::ConstMontyForm};

use crate::brainpool_curve::{BrainpoolCurve, BrainpoolPoint};
use crate::comb::Comb;

pub struct Brainpoolp320r1;

const_monty_params!(
    Brainpool320P,
    U320,
    "D35E472036BC4FB7E13C785ED201E065F98FCFA6F6F40DEF4F92B9EC7893EC28FCD412B1F1B32E27"
);
const_monty_params!(
    Brainpool320Q,
    U320,
    "D35E472036BC4FB7E13C785ED201E065F98FCFA5B68F12A32D482EC7EE8658E98691555B44C59311"
);

static COMB: std::sync::OnceLock<Comb<BrainpoolPoint<Brainpoolp320r1>>> =
    std::sync::OnceLock::new();

impl BrainpoolCurve for Brainpoolp320r1 {
    type CurveField = ConstMontyForm<Brainpool320P, { U320::LIMBS }>;
    type GroupField = ConstMontyForm<Brainpool320Q, { U320::LIMBS }>;

    const BITS: u32 = 320;
    const ONE: Self::CurveField = ConstMontyForm::new(&U320::from_u8(0x1));

    const A: Self::CurveField = ConstMontyForm::new(&U320::from_be_hex(
        "3EE30B568FBAB0F883CCEBD46D3F3BB8A2A73513F5EB79DA66190EB085FFA9F492F375A97D860EB4",
    ));
    const A_T: Self::CurveField = ConstMontyForm::new(&U320::from_be_hex(
        "D35E472036BC4FB7E13C785ED201E065F98FCFA6F6F40DEF4F92B9EC7893EC28FCD412B1F1B32E24",
    ));
    const B: Self::CurveField = ConstMontyForm::new(&U320::from_be_hex(
        "520883949DFDBC42D3AD198640688A6FE13F41349554B49ACC31DCCD884539816F5EB4AC8FB1F1A6",
    ));
    const B_T: Self::CurveField = ConstMontyForm::new(&U320::from_be_hex(
        "A7F561E038EB1ED560B3D147DB782013064C19F27ED27C6780AAF77FB8A547CEB5B4FEF422340353",
    ));

    const Z2: Self::CurveField = ConstMontyForm::new(&U320::from_be_hex(
        "81B063404030CC2A3165EBF290A7E8B5361065D22B699E2D31B27BBBDEEB8A8BFD7979879E605019",
    ));
    const Z3: Self::CurveField = ConstMontyForm::new(&U320::from_be_hex(
        "B58D43456FEE36292986BBD1F780157D41EEBDB3EDA58B8DA5B5F7739FD7AA78CEC24B22AB515E69",
    ));
    const Z2_INV: Self::CurveField = ConstMontyForm::new(&U320::from_be_hex(
        "0F9DA654F862114D19A7D7879F9506D9A3337951C04D80B9D6DF5042E04B14E0878B4C6929E4EA80",
    ));
    const Z3_INV: Self::CurveField = ConstMontyForm::new(&U320::from_be_hex(
        "A1999F91FAA0452D2680ABFEB53808590D839859DD721BA8273CFCA99688508AE968AE331A8C6B9A",
    ));

    const G: BrainpoolPoint<Self> = BrainpoolPoint::<Self> {
        x: ConstMontyForm::new(&U320::from_be_hex(
            "43BD7E9AFB53D8B85289BCC48EE5BFE6F20137D10A087EB6E7871E2A10A599C710AF8D0D39E20611",
        )),
        y: ConstMontyForm::new(&U320::from_be_hex(
            "14FDD05545EC1CC8AB4093247F77275E0743FFED117182EAA9C77877AAAC6AC7D35245D1692E8EE1",
        )),
        z: ConstMontyForm::new(&U320::from_u8(0x1)),
    };
    const G_T: BrainpoolPoint<Self> = BrainpoolPoint::<Self> {
        x: ConstMontyForm::new(&U320::from_be_hex(
            "925BE9FB01AFC6FB4D3E7D4990010F813408AB106C4F09CB7EE07868CC136FFF3357F624A21BED52",
        )),
        y: ConstMontyForm::new(&U320::from_be_hex(
            "63BA3A7A27483EBF6671DBEF7ABB30EBEE084E58A0B077AD42A5A0989D1EE71B1B9BC0455FB0D2C3",
        )),
        z: ConstMontyForm::new(&U320::from_u8(0x1)),
    };
    const INFINITY: BrainpoolPoint<Self> = BrainpoolPoint::<Self> {
        x: ConstMontyForm::new(&U320::from_u8(0x0)),
        y: Self::ONE,
        z: ConstMontyForm::new(&U320::from_u8(0x0)),
    };

    fn comb() -> Option<&'static Comb<BrainpoolPoint<Brainpoolp320r1>>> {
        Some(
            COMB.get_or_init(|| {
                Comb::<BrainpoolPoint<Brainpoolp320r1>>::new(Brainpoolp320r1::G_T, 4)
            }),
        )
    }
}
