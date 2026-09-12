use crate::secp192k1::*;

#[test]
fn test_secp192k1_reduce_mul() {
    let expected: Field<Secp192k1> = Field::<Secp192k1> {
        x: [0x1fdc0bcc5d639ee9, 0x6210fc90b1e33a21, 0x06555f3ed9c20bc1],
    };
    let n: MulResult<Secp192k1> = MulResult::<Secp192k1> {
        x: [
            0x015c8436ef95b0f0,
            0xc4622f8cf0e50ea5,
            0x8ebb76f2c3b820a6,
            0xd607d4ed345638ce,
            0xc369200435046230,
            0x58df53b5dbb21aeb,
        ],
    };
    let actual = n.reduce();
    assert_eq!(actual, expected);
}

#[test]
fn test_secp192k1_normalize() {
    let expected: Point<Secp192k1> = Point::<Secp192k1> {
        x: Field::<Secp192k1> {
            x: [0x5f3745e5a3a01e3c, 0x623ad4562ae7fc11, 0x9e6ffb04eac7f88f],
        },
        y: Field::<Secp192k1> {
            x: [0xc02b8e2d4874b49c, 0x816b21455c94d766, 0x890fba79dec762e2],
        },
        z: Secp192k1::ONE,
    };
    let p: Point<Secp192k1> = Point::<Secp192k1> {
        x: Secp192k1::G.x,
        y: Secp192k1::G.y,
        z: Field::<Secp192k1> { x: [0x3, 0x0, 0x0] },
    };
    let actual = p.normalize();
    assert_eq!(actual, expected);
}
