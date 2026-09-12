use crate::secp256k1::*;

#[test]
fn test_secp256k1_mul_reduce() {
    let expected: Field<Secp256k1> = Field::<Secp256k1> {
        x: [
            0x41adc5042aa736e7,
            0x8ecfa84c29a9716a,
            0x3ae2d9c5965412e8,
            0x5258ba2490f4f1ed,
        ],
    };
    let n: MulResult<Secp256k1> = MulResult::<Secp256k1> {
        x: [
            0x999d004ec630a5b0,
            0x2f35b24b391a9ff9,
            0x219e2a70414d70e7,
            0x86cf3d06ba44db90,
            0x97215f01e1215166,
            0x58362ea1b8d1fece,
            0xb00521ff55adefdf,
            0x0093dd1e973a9b5e,
        ],
    };
    let actual = n.reduce();
    assert_eq!(actual, expected);
}

#[test]
fn test_secp256k1_add() {
    let expected: Field<Secp256k1> = Field::<Secp256k1> {
        x: [
            0x919ea4d63e2bf7c5,
            0x09ee713ee19da192,
            0xe838811de4079415,
            0xbd8dc78240c5bdee,
        ],
    };
    let n: Field<Secp256k1> = Field::<Secp256k1> {
        x: [
            0x8c65211e8d5790d5,
            0xeb41ad24396677ab,
            0xc3d2be009a6f452f,
            0x00c886470849890b,
        ],
    };
    let m: Field<Secp256k1> = Field::<Secp256k1> {
        x: [
            0x053983b7b0d466f0,
            0x1eacc41aa83729e7,
            0x2465c31d49984ee5,
            0xbcc5413b387c34e3,
        ],
    };
    let actual = n + m;
    assert_eq!(actual, expected);
}

#[test]
fn test_secp256k1_add_overflow() {
    let expected: Field<Secp256k1> = Field::<Secp256k1> {
        x: [
            0xfffffffefffffc2d,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
        ],
    };
    let n: Field<Secp256k1> = Field::<Secp256k1> {
        x: [
            0xfffffffefffffc2e,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
        ],
    };
    let actual = n + n;
    assert_eq!(actual, expected);
}

#[test]
fn test_secp256k1_sub() {
    let expected1: Field<Secp256k1> = Field::<Secp256k1> {
        x: [
            0x872b9d65dc832614,
            0xcc94e909912f4dc4,
            0x9f6cfae350d6f64a,
            0x4403450bcfcd5428,
        ],
    };
    let expected2: Field<Secp256k1> = Field::<Secp256k1> {
        x: [
            0x78d46299237cd61b,
            0x336b16f66ed0b23b,
            0x6093051caf2909b5,
            0xbbfcbaf43032abd7,
        ],
    };
    let n: Field<Secp256k1> = Field::<Secp256k1> {
        x: [
            0x8c65211e8d5790d5,
            0xeb41ad24396677ab,
            0xc3d2be009a6f452f,
            0x00c886470849890b,
        ],
    };
    let m: Field<Secp256k1> = Field::<Secp256k1> {
        x: [
            0x053983b7b0d466f0,
            0x1eacc41aa83729e7,
            0x2465c31d49984ee5,
            0xbcc5413b387c34e3,
        ],
    };
    let actual = n - m;
    assert_eq!(actual, expected1);
    let actual = m - n;
    assert_eq!(actual, expected2);
}

#[test]
fn test_secp256k1_sub_overflow() {
    let expected: Field<Secp256k1> = Field::<Secp256k1> {
        x: [0x1, 0x0, 0x0, 0x0],
    };
    let n: Field<Secp256k1> = Field::<Secp256k1> {
        x: [0x0, 0x0, 0x0, 0x0],
    };
    let m: Field<Secp256k1> = Field::<Secp256k1> {
        x: [
            0xfffffffefffffc2e,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
        ],
    };
    let actual = n - m;
    assert_eq!(actual, expected);
}

#[test]
fn test_secp256k1_mul() {
    let expected: Field<Secp256k1> = Field::<Secp256k1> {
        x: [
            0x41adc5042aa736e7,
            0x8ecfa84c29a9716a,
            0x3ae2d9c5965412e8,
            0x5258ba2490f4f1ed,
        ],
    };
    let n: Field<Secp256k1> = Field::<Secp256k1> {
        x: [
            0x8c65211e8d5790d5,
            0xeb41ad24396677ab,
            0xc3d2be009a6f452f,
            0x00c886470849890b,
        ],
    };
    let m: Field<Secp256k1> = Field::<Secp256k1> {
        x: [
            0x053983b7b0d466f0,
            0x1eacc41aa83729e7,
            0x2465c31d49984ee5,
            0xbcc5413b387c34e3,
        ],
    };
    let actual = n * m;
    assert_eq!(actual, expected);
}

#[test]
fn test_secp256k1_mul_overflow() {
    let expected: Field<Secp256k1> = Field::<Secp256k1> {
        x: [0x2, 0x0, 0x0, 0x0],
    };
    let n: Field<Secp256k1> = Field::<Secp256k1> {
        x: [
            0xfffffffefffffc2e,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
        ],
    };
    let m: Field<Secp256k1> = Field::<Secp256k1> {
        x: [
            0xfffffffefffffc2d,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
        ],
    };
    let actual = n * m;
    assert_eq!(actual, expected);
}

#[test]
fn test_secp256k1_mul_const() {
    let expected: Field<Secp256k1> = Field::<Secp256k1> {
        x: [
            0x632908f46abc86a8,
            0x5a0d6921cb33bd5c,
            0x1e95f004d37a297f,
            0x06443238424c485e,
        ],
    };
    let n: Field<Secp256k1> = Field::<Secp256k1> {
        x: [
            0x8c65211e8d5790d5,
            0xeb41ad24396677ab,
            0xc3d2be009a6f452f,
            0x00c886470849890b,
        ],
    };
    let actual = n * 8;
    assert_eq!(actual, expected);
}

#[test]
fn test_secp256k1_mul_const_overflow() {
    let expected: Field<Secp256k1> = Field::<Secp256k1> {
        x: [
            0xfffffffefffffc27,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
        ],
    };
    let n: Field<Secp256k1> = Field::<Secp256k1> {
        x: [
            0xfffffffefffffc2e,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
        ],
    };
    let actual = n * 8;
    assert_eq!(actual, expected);
}

#[test]
fn test_secp256k1_sqr() {
    let expected: Field<Secp256k1> = Field::<Secp256k1> {
        x: [
            0xe2e34d3c530d6035,
            0x160422654541c0d6,
            0x668169c6bdccc207,
            0xb43ea0eb9b110c16,
        ],
    };
    let n: Field<Secp256k1> = Field::<Secp256k1> {
        x: [
            0x8c65211e8d5790d5,
            0xeb41ad24396677ab,
            0xc3d2be009a6f452f,
            0x00c886470849890b,
        ],
    };
    let actual = n.sqr();
    assert_eq!(actual, expected);
}

#[test]
fn test_secp256k1_sqr_overflow() {
    let expected: Field<Secp256k1> = Field::<Secp256k1> {
        x: [0x1, 0x0, 0x0, 0x0],
    };
    let n: Field<Secp256k1> = Field::<Secp256k1> {
        x: [
            0xfffffffefffffc2e,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
        ],
    };
    let actual = n.sqr();
    assert_eq!(actual, expected);
}

#[test]
fn test_secp256k1_pt_normalize() {
    let expected: Point<Secp256k1> = Point::<Secp256k1> {
        x: Field::<Secp256k1> {
            x: [
                0x1dfb8073b252b288,
                0x5633fef3b9ef62f3,
                0xc7357631ef825902,
                0x2894ccd4fdf43e8e,
            ],
        },
        y: Field::<Secp256k1> {
            x: [
                0x896d4584fe5af04d,
                0xa9b2916d8cd71c08,
                0x1f36fea95a05ad8d,
                0x6d68f37d0ce14177,
            ],
        },
        z: Secp256k1::ONE,
    };
    let p: Point<Secp256k1> = Point::<Secp256k1> {
        x: Secp256k1::G.x,
        y: Secp256k1::G.y,
        z: Field::<Secp256k1> {
            x: [0x3, 0x0, 0x0, 0x0],
        },
    };
    let actual = p.normalize();
    assert_eq!(actual, expected);
}

#[test]
fn test_secp256k1_pt_mul() {
    let expected = Point::<Secp256k1> {
        x: Field::<Secp256k1> {
            x: [
                0x30a6a4c7a9a8de82,
                0x82db22734f854c35,
                0x40085c6d8e5fad42,
                0xd0912e3cdeb62a3b,
            ],
        },
        y: Field::<Secp256k1> {
            x: [
                0x360a05cb12a643de,
                0x3565187af3a74238,
                0x3b979642c43f85af,
                0xcd02e989edda001e,
            ],
        },
        z: Secp256k1::ONE,
    };
    let n: [u8; 32] = [
        0xf8, 0xd6, 0xab, 0xb0, 0x8e, 0x35, 0xee, 0xf5, 0xa4, 0xf9, 0x23, 0xd1, 0x34, 0xd3, 0x4f,
        0x46, 0xbb, 0x6a, 0xd0, 0xb0, 0x9e, 0x85, 0x55, 0x09, 0xa7, 0x7f, 0x95, 0xd0, 0x53, 0x33,
        0x6e, 0x4b,
    ];
    let actual = (Secp256k1::G * &n).normalize();
    assert_eq!(actual, expected);
}

#[test]
fn test_secp256k1_pt_mul_order() {
    let n: [u8; 32] = [
        0x41, 0x41, 0x36, 0xd0, 0x8c, 0x5e, 0xd2, 0xbf, 0x3b, 0xa0, 0x48, 0xaf, 0xe6, 0xdc, 0xae,
        0xba, 0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff,
    ];
    let actual = (Secp256k1::G * &n).normalize();
    assert!(actual.is_point_at_infinity());
}
