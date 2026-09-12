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
