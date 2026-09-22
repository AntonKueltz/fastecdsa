use std::sync::OnceLock;

use crypto_bigint::{U512, const_monty_params, modular::ConstMontyForm};

use crate::brainpool_curve::{BrainpoolCurve, BrainpoolPoint};
use crate::comb::Comb;
use crate::wnaf::lookup_table;

pub struct Brainpoolp512r1;

const_monty_params!(
    Brainpool512P,
    U512,
    "AADD9DB8DBE9C48B3FD4E6AE33C9FC07CB308DB3B3C9D20ED6639CCA703308717D4D9B009BC66842AECDA12AE6A380E62881FF2F2D82C68528AA6056583A48F3"
);
const_monty_params!(
    Brainpool512Q,
    U512,
    "AADD9DB8DBE9C48B3FD4E6AE33C9FC07CB308DB3B3C9D20ED6639CCA70330870553E5C414CA92619418661197FAC10471DB1D381085DDADDB58796829CA90069"
);

static COMB: OnceLock<Comb<BrainpoolPoint<Brainpoolp512r1>>> = OnceLock::new();
static G_TABLE: OnceLock<Vec<BrainpoolPoint<Brainpoolp512r1>>> = OnceLock::new();

impl BrainpoolCurve for Brainpoolp512r1 {
    type CurveField = ConstMontyForm<Brainpool512P, { U512::LIMBS }>;
    type GroupField = ConstMontyForm<Brainpool512Q, { U512::LIMBS }>;

    const BITS: u32 = 512;
    const ONE: Self::CurveField = ConstMontyForm::new(&U512::from_u8(0x1));

    const A: Self::CurveField = ConstMontyForm::new(&U512::from_be_hex(
        "7830A3318B603B89E2327145AC234CC594CBDD8D3DF91610A83441CAEA9863BC2DED5D5AA8253AA10A2EF1C98B9AC8B57F1117A72BF2C7B9E7C1AC4D77FC94CA",
    ));
    const A_T: Self::CurveField = ConstMontyForm::new(&U512::from_be_hex(
        "AADD9DB8DBE9C48B3FD4E6AE33C9FC07CB308DB3B3C9D20ED6639CCA703308717D4D9B009BC66842AECDA12AE6A380E62881FF2F2D82C68528AA6056583A48F0",
    ));
    const B: Self::CurveField = ConstMontyForm::new(&U512::from_be_hex(
        "3DF91610A83441CAEA9863BC2DED5D5AA8253AA10A2EF1C98B9AC8B57F1117A72BF2C7B9E7C1AC4D77FC94CADC083E67984050B75EBAE5DD2809BD638016F723",
    ));
    const B_T: Self::CurveField = ConstMontyForm::new(&U512::from_be_hex(
        "7CBBBCF9441CFAB76E1890E46884EAE321F70C0BCB4981527897504BEC3E36A62BCDFA2304976540F6450085F2DAE145C22553B465763689180EA2571867423E",
    ));

    const Z2: Self::CurveField = ConstMontyForm::new(&U512::from_be_hex(
        "2730CD020DE1B8EAD89452DDAF5E71F40F60FF88E4DDE904587E383B6A610DFD6A989EE8EF2EDB21446265BA95FC1B93E39B243C0FF2E81CC35906720139D897",
    ));
    const Z3: Self::CurveField = ConstMontyForm::new(&U512::from_be_hex(
        "7D410B34C91776E0A7C0B81D4C3E01DD33AC635C4BF27C622257A6E3EBFD2037B61A185CFF2ED8C994D78F96FE9674D2BD04D0BA3B0CFECF67456EE424E52039",
    ));
    const Z2_INV: Self::CurveField = ConstMontyForm::new(&U512::from_be_hex(
        "323BB83B14B2EFCBA87DD60598A6AFDDC69B89F1820CC174FAEDAEDD5FD1DDCF9679E37AD8AA0C71213EF2A7C5833A4B5091DA99459DC6C1AA5AEAD7C6468574",
    ));
    const Z3_INV: Self::CurveField = ConstMontyForm::new(&U512::from_be_hex(
        "478BE7D4DD2BE98A04A92FD4EA34D816F20B4EAC85917E840D38940C8990E74108B48B1E3B42AEC7B6FD5835E72FFB9489FC0E3001D5026619A08F2D66561510",
    ));

    const G: BrainpoolPoint<Self> = BrainpoolPoint::<Self> {
        x: ConstMontyForm::new(&U512::from_be_hex(
            "81AEE4BDD82ED9645A21322E9C4C6A9385ED9F70B5D916C1B43B62EEF4D0098EFF3B1F78E2D0D48D50D1687B93B97D5F7C6D5047406A5E688B352209BCB9F822",
        )),
        y: ConstMontyForm::new(&U512::from_be_hex(
            "7DDE385D566332ECC0EABFA9CF7822FDF209F70024A57B1AA000C55B881F8111B2DCDE494A5F485E5BCA4BD88A2763AED1CA2B2FA8F0540678CD1E0F3AD80892",
        )),
        z: ConstMontyForm::new(&U512::from_u8(0x1)),
    };
    const G_T: BrainpoolPoint<Self> = BrainpoolPoint::<Self> {
        x: ConstMontyForm::new(&U512::from_be_hex(
            "640ECE5C12788717B9C1BA06CBC2A6FEBA85842458C56DDE9DB1758D39C0313D82BA51735CDB3EA499AA77A7D6943A64F7A3F25FE26F06B51BAA2696FA9035DA",
        )),
        y: ConstMontyForm::new(&U512::from_be_hex(
            "5B534BD595F5AF0FA2C892376C84ACE1BB4E3019B71634C01131159CAE03CEE9D9932184BEEF216BD71DF2DADF86A627306ECFF96DBB8BACE198B61E00F8B332",
        )),
        z: ConstMontyForm::new(&U512::from_u8(0x1)),
    };
    const INFINITY: BrainpoolPoint<Self> = BrainpoolPoint::<Self> {
        x: ConstMontyForm::new(&U512::from_u8(0x0)),
        y: Self::ONE,
        z: ConstMontyForm::new(&U512::from_u8(0x0)),
    };

    fn comb() -> Option<&'static Comb<BrainpoolPoint<Brainpoolp512r1>>> {
        Some(
            COMB.get_or_init(|| {
                Comb::<BrainpoolPoint<Brainpoolp512r1>>::new(Brainpoolp512r1::G_T, 5)
            }),
        )
    }

    fn g_table() -> &'static Vec<BrainpoolPoint<Self>> {
        G_TABLE.get_or_init(|| lookup_table(&Self::G_T, 8))
    }
}
