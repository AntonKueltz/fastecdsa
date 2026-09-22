use crate::sec2_curve::p224::*;

#[test]
fn test_p224_mul_reduce() {
    let expected: Field<P224> = Field::<P224> {
        x: [
            0x493d883344cad88a,
            0x57c802985f6d8313,
            0x647cd0f2b7f8707c,
            0xf49af314,
        ],
    };
    let n: MulResult<P224> = MulResult::<P224> {
        x: [
            0x5f05da840370bd88,
            0x808ebc77e3ab4d61,
            0xb327c79b6c50b25b,
            0xf48550cf100fc8c5,
            0x815c55bf42df9159,
            0xca20942fe76c9edd,
            0x02e1748ed2e8c0f6,
            0x0,
        ],
    };
    let actual = n.reduce();
    assert_eq!(actual, expected);
}

#[test]
fn test_p224_from_bytes() {
    let expected: Field<P224> = Field::<P224> {
        x: [
            0x493d883344cad88a,
            0x57c802985f6d8313,
            0x647cd0f2b7f8707c,
            0xf49af314,
        ],
    };
    let n: &[u8] = &[
        0x8a, 0xd8, 0xca, 0x44, 0x33, 0x88, 0x3d, 0x49, 0x13, 0x83, 0x6d, 0x5f, 0x98, 0x02, 0xc8,
        0x57, 0x7c, 0x70, 0xf8, 0xb7, 0xf2, 0xd0, 0x7c, 0x64, 0x14, 0xf3, 0x9a, 0xf4,
    ];
    let actual = Field::<P224>::from(n);
    assert_eq!(actual, expected);
}

#[test]
fn test_p224_to_bytes() {
    let expected: Vec<u8> = [
        0x8a, 0xd8, 0xca, 0x44, 0x33, 0x88, 0x3d, 0x49, 0x13, 0x83, 0x6d, 0x5f, 0x98, 0x02, 0xc8,
        0x57, 0x7c, 0x70, 0xf8, 0xb7, 0xf2, 0xd0, 0x7c, 0x64, 0x14, 0xf3, 0x9a, 0xf4,
    ]
    .to_vec();
    let n: Field<P224> = Field::<P224> {
        x: [
            0x493d883344cad88a,
            0x57c802985f6d8313,
            0x647cd0f2b7f8707c,
            0xf49af314,
        ],
    };
    let actual: Vec<u8> = Field::<P224>::into(n);
    assert_eq!(actual, expected);
}

#[test]
fn test_p224_add() {
    let expected: Field<P224> = Field::<P224> {
        x: [
            0x08cbc61e236627aa,
            0xc6215d38ba81db7b,
            0x2f038d0716a55b7d,
            0x7c7aac97,
        ],
    };
    let n: Field<P224> = Field::<P224> {
        x: [
            0xfd3b4da73de54a8e,
            0x3a1c81c8f57e81d4,
            0xd4ca5f1c578abf39,
            0x063c9f95,
        ],
    };
    let m: Field<P224> = Field::<P224> {
        x: [
            0x0b907876e580dd1c,
            0x8c04db6fc50359a6,
            0x5a392deabf1a9c44,
            0x763e0d01,
        ],
    };
    let actual = n + m;
    assert_eq!(actual, expected);
}

#[test]
fn test_p224_add_overflow() {
    let expected: Field<P224> = Field::<P224> {
        x: [
            0xfffffffffffffffb,
            0xfffffffeffffffff,
            0xffffffffffffffff,
            0xffffffff,
        ],
    };
    let actual = P224::A + P224::A;
    assert_eq!(actual, expected);
}

#[test]
fn test_p224_sub() {
    let expected1: Field<P224> = Field::<P224> {
        x: [
            0xf1aad53058646d73,
            0xae17a658307b282e,
            0x7a913131987022f4,
            0x8ffe9294,
        ],
    };
    let expected2: Field<P224> = Field::<P224> {
        x: [
            0x0e552acfa79b928e,
            0x51e859a6cf84d7d1,
            0x856ecece678fdd0b,
            0x70016d6b,
        ],
    };
    let n: Field<P224> = Field::<P224> {
        x: [
            0xfd3b4da73de54a8e,
            0x3a1c81c8f57e81d4,
            0xd4ca5f1c578abf39,
            0x063c9f95,
        ],
    };
    let m: Field<P224> = Field::<P224> {
        x: [
            0x0b907876e580dd1c,
            0x8c04db6fc50359a6,
            0x5a392deabf1a9c44,
            0x763e0d01,
        ],
    };
    let actual = n - m;
    assert_eq!(actual, expected1);
    let actual = m - n;
    assert_eq!(actual, expected2);
}

#[test]
fn test_p224_sub_overflow() {
    let expected: Field<P224> = Field::<P224> {
        x: [0x3, 0x0, 0x0, 0x0],
    };
    let n = Field::<P224> {
        x: [0x0, 0x0, 0x0, 0x0],
    };
    let actual = n - P224::A;
    assert_eq!(actual, expected);
}

#[test]
fn test_p224_mul() {
    let expected: Field<P224> = Field::<P224> {
        x: [
            0x493d883344cad88a,
            0x57c802985f6d8313,
            0x647cd0f2b7f8707c,
            0xf49af314,
        ],
    };
    let n: Field<P224> = Field::<P224> {
        x: [
            0xfd3b4da73de54a8e,
            0x3a1c81c8f57e81d4,
            0xd4ca5f1c578abf39,
            0x063c9f95,
        ],
    };
    let m: Field<P224> = Field::<P224> {
        x: [
            0x0b907876e580dd1c,
            0x8c04db6fc50359a6,
            0x5a392deabf1a9c44,
            0x763e0d01,
        ],
    };
    let actual = n * m;
    assert_eq!(actual, expected);
}

#[test]
fn test_p224_mul_overflow() {
    let expected: Field<P224> = Field::<P224> {
        x: [0x2, 0x0, 0x0, 0x0],
    };
    let n: Field<P224> = Field::<P224> {
        x: [0x0, 0xffffffff00000000, 0xffffffffffffffff, 0xffffffff],
    };
    let m: Field<P224> = Field::<P224> {
        x: [
            0xffffffffffffffff,
            0xfffffffeffffffff,
            0xffffffffffffffff,
            0xffffffff,
        ],
    };
    let actual = n * m;
    assert_eq!(actual, expected);
}

#[test]
fn test_p224_mul_const() {
    let expected: Field<P224> = Field::<P224> {
        x: [
            0xe9da6d39ef2a5470,
            0xd0e40e47abf40ea7,
            0xa652f8e2bc55f9c9,
            0x31e4fcae,
        ],
    };
    let n: Field<P224> = Field::<P224> {
        x: [
            0xfd3b4da73de54a8e,
            0x3a1c81c8f57e81d4,
            0xd4ca5f1c578abf39,
            0x063c9f95,
        ],
    };
    let actual = n * 8;
    assert_eq!(actual, expected);
}

#[test]
fn test_p224_mul_const_overflow() {
    let expected: Field<P224> = Field::<P224> {
        x: [
            0xffffffffffffffe9,
            0xfffffffeffffffff,
            0xffffffffffffffff,
            0xffffffff,
        ],
    };
    let actual = P224::A * 8;
    assert_eq!(actual, expected);
}

#[test]
fn test_p224_sqr() {
    let expected: Field<P224> = Field::<P224> {
        x: [
            0xaa26ed998a59a7d1,
            0x024d53cd1ea2eddd,
            0x04da527cfa7f1d9e,
            0xb7713872,
        ],
    };
    let n: Field<P224> = Field::<P224> {
        x: [
            0xfd3b4da73de54a8e,
            0x3a1c81c8f57e81d4,
            0xd4ca5f1c578abf39,
            0x063c9f95,
        ],
    };
    let actual = n.sqr();
    assert_eq!(actual, expected);
}

#[test]
fn test_p224_sqr_overflow() {
    let expected: Field<P224> = Field::<P224> {
        x: [0x9, 0x0, 0x0, 0x0],
    };
    let actual = P224::A.sqr();
    assert_eq!(actual, expected);
}

#[test]
fn test_p224_pt_normalize() {
    let expected: Point<P224> = Point::<P224> {
        x: Field::<P224> {
            x: [
                0xbc10d59cb07409b6,
                0x6e01409b7240b060,
                0x23e6ea7fbb5bdae8,
                0x925a043f,
            ],
        },
        y: Field::<P224> {
            x: [
                0xc19c808881aad4bc,
                0x99c127357357c276,
                0xe7526153c40b9ff7,
                0x3f127682,
            ],
        },
        z: P224::ONE,
    };
    let p: Point<P224> = Point::<P224> {
        x: P224::G.x,
        y: P224::G.y,
        z: Field::<P224> {
            x: [0x3, 0x0, 0x0, 0x0],
        },
    };
    let actual = p.normalize();
    assert_eq!(actual, expected);
}

#[test]
fn test_p224_pt_double() {
    let expected: Point<P224> = Point::<P224> {
        x: Field::<P224> {
            x: [
                0x32d268fd1a704fa6,
                0x89474788d16dc180,
                0x76dcb76798e60e6d,
                0x706a46dc,
            ],
        },
        y: Field::<P224> {
            x: [
                0x7acf3709d2e4e8bb,
                0x86892849fca62948,
                0xbc25e7702a704fa9,
                0x1c2b76a7,
            ],
        },
        z: P224::ONE,
    };
    let actual = P224::G.double().normalize();
    assert_eq!(actual, expected);
}

#[test]
fn test_p224_pt_add() {
    let expected: Point<P224> = Point::<P224> {
        x: Field::<P224> {
            x: [
                0x79fe0d08fd896d04,
                0x58b9d2cc75c21802,
                0xa551d0d31eff8225,
                0xdf1b1d66,
            ],
        },
        y: Field::<P224> {
            x: [
                0x4e1af3591981a925,
                0x30130ddf77d31734,
                0xadd0be444c0aa568,
                0xa3f7f03c,
            ],
        },
        z: P224::ONE,
    };
    let p: Point<P224> = Point::<P224> {
        x: Field::<P224> {
            x: [
                0x32d268fd1a704fa6,
                0x89474788d16dc180,
                0x76dcb76798e60e6d,
                0x706a46dc,
            ],
        },
        y: Field::<P224> {
            x: [
                0x7acf3709d2e4e8bb,
                0x86892849fca62948,
                0xbc25e7702a704fa9,
                0x1c2b76a7,
            ],
        },
        z: P224::ONE,
    };
    let actual = (p + P224::G).normalize();
    assert_eq!(actual, expected);
}

#[test]
fn test_p224_pt_mul() {
    let expected: Point<P224> = Point::<P224> {
        x: Field::<P224> {
            x: [
                0x58cc0de7c32b2c04,
                0x0904e80dd3cba915,
                0x78dc47059bb446dd,
                0xdb938d39,
            ],
        },
        y: Field::<P224> {
            x: [
                0xee325576d272b6e8,
                0xed2602c1304218f3,
                0x6a53345e21867f93,
                0xb279687d,
            ],
        },
        z: P224::ONE,
    };
    let n: [u8; 28] = [
        0xa4, 0xab, 0xb3, 0xdb, 0x51, 0x6d, 0xdb, 0xc6, 0xad, 0x35, 0x8f, 0x2f, 0x28, 0x4c, 0x4f,
        0x1c, 0x85, 0xbf, 0x57, 0x64, 0x6f, 0xd4, 0x09, 0xe8, 0x1b, 0xee, 0xb2, 0x17,
    ];
    let actual = (P224::G * &n).normalize();
    assert_eq!(actual, expected);
}
