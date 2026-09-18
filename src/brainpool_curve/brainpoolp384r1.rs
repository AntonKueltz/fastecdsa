use crypto_bigint::{U384, const_monty_params, modular::ConstMontyForm};

use crate::brainpool_curve::{BrainpoolCurve, BrainpoolPoint};
use crate::comb::Comb;

pub struct Brainpoolp384r1;

const_monty_params!(
    Brainpool384P,
    U384,
    "8CB91E82A3386D280F5D6F7E50E641DF152F7109ED5456B412B1DA197FB71123ACD3A729901D1A71874700133107EC53"
);
const_monty_params!(
    Brainpool384Q,
    U384,
    "8CB91E82A3386D280F5D6F7E50E641DF152F7109ED5456B31F166E6CAC0425A7CF3AB6AF6B7FC3103B883202E9046565"
);

static COMB: std::sync::OnceLock<Comb<BrainpoolPoint<Brainpoolp384r1>>> =
    std::sync::OnceLock::new();

impl BrainpoolCurve for Brainpoolp384r1 {
    type CurveField = ConstMontyForm<Brainpool384P, { U384::LIMBS }>;
    type GroupField = ConstMontyForm<Brainpool384Q, { U384::LIMBS }>;

    const BITS: u32 = 384;
    const ONE: Self::CurveField = ConstMontyForm::new(&U384::from_u8(0x1));

    const A: Self::CurveField = ConstMontyForm::new(&U384::from_be_hex(
        "7BC382C63D8C150C3C72080ACE05AFA0C2BEA28E4FB22787139165EFBA91F90F8AA5814A503AD4EB04A8C7DD22CE2826",
    ));
    const A_T: Self::CurveField = ConstMontyForm::new(&U384::from_be_hex(
        "8CB91E82A3386D280F5D6F7E50E641DF152F7109ED5456B412B1DA197FB71123ACD3A729901D1A71874700133107EC50",
    ));
    const B: Self::CurveField = ConstMontyForm::new(&U384::from_be_hex(
        "04A8C7DD22CE28268B39B55416F0447C2FB77DE107DCD2A62E880EA53EEB62D57CB4390295DBC9943AB78696FA504C11",
    ));
    const B_T: Self::CurveField = ConstMontyForm::new(&U384::from_be_hex(
        "7F519EADA7BDA81BD826DBA647910F8C4B9346ED8CCDC64E4B1ABD11756DCE1D2074AA263B88805CED70355A33B471EE",
    ));

    const Z2: Self::CurveField = ConstMontyForm::new(&U384::from_be_hex(
        "7944E97A2D95CE0BCD4E2DF1610D432F1FDCB8E10BE624FBA85B57BC6DA5189EF6EEAB80F4C3BF724DF56D062DB4B3EB",
    ));
    const Z3: Self::CurveField = ConstMontyForm::new(&U384::from_be_hex(
        "8531662625354BE763219530F830CFB71ADEFCDF86EAA140F2A13806E689993631F6CFBD39D3EB3F11A3E43C3BB49CA7",
    ));
    const Z2_INV: Self::CurveField = ConstMontyForm::new(&U384::from_be_hex(
        "1B60D9FEC8FF083C153CC51CCEF3BA2CEA37A66042032CF7F3DFADABF673C0915E3CAB6A67356AE93AB689C5F93B40A8",
    ));
    const Z3_INV: Self::CurveField = ConstMontyForm::new(&U384::from_be_hex(
        "7E38CF40D917C4A68F799D6EBAC699DE459889FACF438C2F877B2750D225DFBB782F532B6FF20E4758DC927CDFD00D26",
    ));

    const G: BrainpoolPoint<Self> = BrainpoolPoint::<Self> {
        x: ConstMontyForm::new(&U384::from_be_hex(
            "1D1C64F068CF45FFA2A63A81B7C13F6B8847A3E77EF14FE3DB7FCAFE0CBD10E8E826E03436D646AAEF87B2E247D4AF1E",
        )),
        y: ConstMontyForm::new(&U384::from_be_hex(
            "8ABE1D7520F9C2A45CB1EB8E95CFD55262B70B29FEEC5864E19C054FF99129280E4646217791811142820341263C5315",
        )),
        z: ConstMontyForm::new(&U384::from_u8(0x1)),
    };
    const G_T: BrainpoolPoint<Self> = BrainpoolPoint::<Self> {
        x: ConstMontyForm::new(&U384::from_be_hex(
            "18DE98B02DB9A306F2AFCD7235F72A819B80AB12EBD653172476FECD462AABFFC4FF191B946A5F54D8D0AA2F418808CC",
        )),
        y: ConstMontyForm::new(&U384::from_be_hex(
            "25AB056962D30651A114AFD2755AD336747F93475B7A1FCA3B88F2B6A208CCFE469408584DC2B2912675BF5B9E582928",
        )),
        z: ConstMontyForm::new(&U384::from_u8(0x1)),
    };
    const INFINITY: BrainpoolPoint<Self> = BrainpoolPoint::<Self> {
        x: ConstMontyForm::new(&U384::from_u8(0x0)),
        y: Self::ONE,
        z: ConstMontyForm::new(&U384::from_u8(0x0)),
    };

    fn comb() -> Option<&'static Comb<BrainpoolPoint<Brainpoolp384r1>>> {
        Some(
            COMB.get_or_init(|| {
                Comb::<BrainpoolPoint<Brainpoolp384r1>>::new(Brainpoolp384r1::G_T, 4)
            }),
        )
    }
}
