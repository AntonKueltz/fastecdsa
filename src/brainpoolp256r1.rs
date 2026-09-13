use crypto_bigint::{U256, const_monty_params, modular::ConstMontyForm};

use crate::curve::{BrainpoolCurve, BrainpoolPoint};

const_monty_params!(
    Brainpool256P,
    U256,
    "A9FB57DBA1EEA9BC3E660A909D838D726E3BF623D52620282013481D1F6E5377"
);
const_monty_params!(
    Brainpool256Q,
    U256,
    "A9FB57DBA1EEA9BC3E660A909D838D718C397AA3B561A6F7901E0E82974856A7"
);

pub struct Brainpoolp256r1;

impl BrainpoolCurve for Brainpoolp256r1 {
    type CurveField = ConstMontyForm<Brainpool256P, 4>;
    type GroupField = ConstMontyForm<Brainpool256Q, 4>;

    const BITS: u32 = 256;
    const ONE: Self::CurveField = ConstMontyForm::new(&U256::from_u8(0x1));

    const A: Self::CurveField = ConstMontyForm::new(&U256::from_be_hex(
        "7D5A0975FC2C3057EEF67530417AFFE7FB8055C126DC5C6CE94A4B44F330B5D9",
    ));
    const A_T: Self::CurveField = ConstMontyForm::new(&U256::from_be_hex(
        "A9FB57DBA1EEA9BC3E660A909D838D726E3BF623D52620282013481D1F6E5374",
    ));
    const B: Self::CurveField = ConstMontyForm::new(&U256::from_be_hex(
        "26DC5C6CE94A4B44F330B5D9BBD77CBF958416295CF7E1CE6BCCDC18FF8C07B6",
    ));
    const B_T: Self::CurveField = ConstMontyForm::new(&U256::from_be_hex(
        "662C61C430D84EA4FE66A7733D0B76B7BF93EBC4AF2F49256AE58101FEE92B04",
    ));

    const Z2: Self::CurveField = ConstMontyForm::new(&U256::from_be_hex(
        "82BC2EB23A37E62AD4843DF8CFFD588AAD32A61E81A9AD970B3FBD112ADB923F",
    ));
    const Z3: Self::CurveField = ConstMontyForm::new(&U256::from_be_hex(
        "842578589FB6C8E983ECC3D1194894AD7A79F385AFABEE717C45C8259685998B",
    ));
    const Z2_INV: Self::CurveField = ConstMontyForm::new(&U256::from_be_hex(
        "79838C22D2B8DC9AF2E6CF56F8826DC3DFE10FCB17B6AAAF551EE52BEF12F826",
    ));
    const Z3_INV: Self::CurveField = ConstMontyForm::new(&U256::from_be_hex(
        "042CC378EB15E51D6259EBB65740955A838BC0BFDCEC227432A41879492006A2",
    ));

    const G: BrainpoolPoint<Self> = BrainpoolPoint::<Self> {
        x: ConstMontyForm::new(&U256::from_be_hex(
            "8BD2AEB9CB7E57CB2C4B482FFC81B7AFB9DE27E1E3BD23C23A4453BD9ACE3262",
        )),
        y: ConstMontyForm::new(&U256::from_be_hex(
            "547EF835C3DAC4FD97F8461A14611DC9C27745132DED8E545C1D54C72F046997",
        )),
        z: ConstMontyForm::new(&U256::from_u8(0x1)),
    };
    const G_T: BrainpoolPoint<Self> = BrainpoolPoint::<Self> {
        x: ConstMontyForm::new(&U256::from_be_hex(
            "A3E8EB3CC1CFE7B7732213B23A656149AFA142C47AAFBC2B79A191562E1305F4",
        )),
        y: ConstMontyForm::new(&U256::from_be_hex(
            "2D996C823439C56D7F7B22E14644417E69BCB6DE39D027001DABE8F35B25C9BE",
        )),
        z: ConstMontyForm::new(&U256::from_u8(0x1)),
    };
    const INFINITY: BrainpoolPoint<Self> = BrainpoolPoint::<Self> {
        x: ConstMontyForm::new(&U256::from_u8(0x0)),
        y: Self::ONE,
        z: ConstMontyForm::new(&U256::from_u8(0x0)),
    };
}
