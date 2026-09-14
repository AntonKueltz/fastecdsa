use crate::secp224k1::*;

#[test]
fn test_secp224k1_reduce_mul() {
    let expected: Field<Secp224k1> = Field::<Secp224k1> {
        x: [
            0x0b2e05dffdae64c9,
            0x8a7a9630aee4dd6d,
            0x2bebca71fd995dc8,
            0x0000000045f3f9a9,
        ],
    };
    let n: MulResult<Secp224k1> = MulResult::<Secp224k1> {
        x: [
            0x2c14b753c7e89ee0,
            0xddcc4d35556ade1f,
            0x94ff53e9f9c67b79,
            0xceb2e89c40a44856,
            0xbe85586267ee403d,
            0xd408d8225ccb4926,
            0x30113179ac69ecf1,
            0x0,
        ],
    };
    let actual = n.reduce();
    assert_eq!(actual, expected);
}

#[test]
fn test_secp224k1_normalize() {
    let expected: Point<Secp224k1> = Point::<Secp224k1> {
        x: Field::<Secp224k1> {
            x: [
                0xafd4cc59e7928343,
                0x788c22a34c257c8d,
                0x6f50334a65a962e0,
                0x000000008b171e66,
            ],
        },
        y: Field::<Secp224k1> {
            x: [
                0xa0ee1948c7246d06,
                0xa7f65dfd403ae9c8,
                0x2a9366c0d643a947,
                0x000000007f58354f,
            ],
        },
        z: Secp224k1::ONE,
    };
    let p: Point<Secp224k1> = Point::<Secp224k1> {
        x: Secp224k1::G.x,
        y: Secp224k1::G.y,
        z: Field::<Secp224k1> {
            x: [0x3, 0x0, 0x0, 0x0],
        },
    };
    let actual = p.normalize();
    assert_eq!(actual, expected);
}

#[test]
fn test_secp224k1_reduce_mul_edge_case() {
    let expected = Field::<Secp224k1> {
        x: [
            0x7f1f51fd165301ef,
            0x2870612b28ae5e6a,
            0xfee47fb49516f189,
            0x000000007f7183bf,
        ],
    };
    let n = MulResult::<Secp224k1> {
        x: [
            0x8ca5388fcb466c04,
            0xce576b3d10ffe49e,
            0x982621fc14a8ca51,
            0xe52e28c35df8e775,
            0xb432bb0c778dcd7d,
            0xeeab57d90d1eaa3f,
            0xfffff2f322d36923,
            0x0,
        ],
    };
    let actual = n.reduce();
    assert_eq!(actual, expected);
}

#[test]
fn test_secp224k1_sqr() {
    let expected = Field::<Secp224k1> {
        x: [
            0x7f1f51fd165301ef,
            0x2870612b28ae5e6a,
            0xfee47fb49516f189,
            0x000000007f7183bf,
        ],
    };
    let n = Field::<Secp224k1> {
        x: [
            0x752424862e3b5b02,
            0x07def52b0b70e8da,
            0x91546aadec19a2d5,
            0x00000000fffff979,
        ],
    };
    let actual = n.sqr();
    assert_eq!(actual, expected);
}

#[test]
fn test_secp224k1_sqr_n_times() {
    let expected = Field::<Secp224k1> {
        x: [
            0x7feb2293d5714b1d,
            0x84918805d546c47e,
            0x3b40f9d5d22c8579,
            0x0000000014bd9f89,
        ],
    };
    let n = Field::<Secp224k1> {
        x: [
            0x7bbeb37e0cd6791e,
            0x7826bd7e2696e76b,
            0x3a13763399dd3a58,
            0x00000000db3e4a73,
        ],
    };
    let actual = n.sqr_n_times(76);
    assert_eq!(actual, expected);
}

#[test]
fn test_secp224k1_pt_mul() {
    let expected = Point::<Secp224k1> {
        x: Field::<Secp224k1> {
            x: [
                0xa8fd05ba92de073a,
                0x7661ef41ec7d1bc9,
                0x8a673ae3d0fe8d24,
                0x000000009966e742,
            ],
        },
        y: Field::<Secp224k1> {
            x: [
                0x61d1087d4a2fcaf0,
                0x8e1bb77c0a8c21e9,
                0xe0b197abc0a68066,
                0x00000000762531d8,
            ],
        },
        z: Secp224k1::ONE,
    };
    let n: [u8; 28] = [
        0xef, 0xc8, 0x36, 0x4a, 0x33, 0x04, 0xb5, 0x7e, 0x9d, 0x52, 0x36, 0x92, 0x1c, 0x28, 0x26,
        0x15, 0x31, 0x61, 0xe0, 0x5d, 0xff, 0x47, 0x23, 0x38, 0xc6, 0xd1, 0x2b, 0xb3,
    ];
    let actual = (Secp224k1::G * &n).normalize();
    assert_eq!(actual, expected);
}
