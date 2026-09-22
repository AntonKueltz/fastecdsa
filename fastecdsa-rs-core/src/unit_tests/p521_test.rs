use crate::sec2_curve::p521::*;

#[test]
fn test_p521_mul_reduce() {
    let expected = Field::<P521> {
        x: [
            0x705a9fa5e236e828,
            0x53eca8d86a8dc8ed,
            0x2d4bccd8dddb533c,
            0x0067648c56877f03,
            0x663e97ff4d4feb9e,
            0xa5204ff17251c57d,
            0xd190a4bd15e975c1,
            0x24297f2cbf5a86f9,
            0x0000000000000135,
        ],
    };
    let n = MulResult::<P521> {
        x: [
            0xf76b97b01813a6b4,
            0x78fc198441fb7a9d,
            0xfcf7a338ce4f07cc,
            0x25a92b5cf573b8c6,
            0x013ffad2061748c1,
            0xd13bb4e993ac8ca5,
            0x84d5d8c8561f2dbc,
            0xc5cd8a2ee3f220b1,
            0xde0feb944682e905,
            0xe11ea851249c9ef1,
            0xa853401f1896dfb5,
            0x7c725ec2278c7860,
            0xfd3a5a8e7145b9b5,
            0xc9360fbd4a71b0c9,
            0x7597e97f949009a7,
            0xb7e9fbb6d0cc9099,
            0x0000000000005ebc,
            0x0,
        ],
    };
    let actual = n.reduce();
    assert_eq!(actual, expected);
}
