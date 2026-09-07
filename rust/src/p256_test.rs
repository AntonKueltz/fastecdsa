use crate::p256::*;

#[test]
fn test_p256_mul_reduce() {
    let expected: P256Element = P256Element {
        x: [
            0xae892a213489686c,
            0xb6f1d25e3569310b,
            0xad5553f27083c63e,
            0x3601c5b3196e68e2,
        ],
    };
    let n: P256MulResult = P256MulResult {
        x: [
            0x78af0c46c60d189b,
            0xe4235a4da53be257,
            0x09948537b499f1f4,
            0xd5848dd7a282129a,
            0x0dab0502fcbd9e19,
            0x8374f32c5a457100,
            0x26d9a77ce48942b0,
            0x199ef8090d1475f2,
        ],
    };
    let actual = n.reduce();
    assert_eq!(actual, expected);
}

#[test]
fn test_p256_from_bytes() {
    let expected: P256Element = P256Element {
        x: [
            0x98f30938d11ca647,
            0xd24dea34c2cd3128,
            0x392e25cd56768c23,
            0x77391109555f17c2,
        ],
    };
    let n: &[u8] = &[
        0x47, 0xa6, 0x1c, 0xd1, 0x38, 0x09, 0xf3, 0x98, 0x28, 0x31, 0xcd, 0xc2, 0x34, 0xea, 0x4d,
        0xd2, 0x23, 0x8c, 0x76, 0x56, 0xcd, 0x25, 0x2e, 0x39, 0xc2, 0x17, 0x5f, 0x55, 0x09, 0x11,
        0x39, 0x77,
    ];
    let actual = P256Element::from(n);
    assert_eq!(actual, expected);
}

#[test]
fn test_p256_to_bytes() {
    let expected: Vec<u8> = [
        0x47, 0xa6, 0x1c, 0xd1, 0x38, 0x09, 0xf3, 0x98, 0x28, 0x31, 0xcd, 0xc2, 0x34, 0xea, 0x4d,
        0xd2, 0x23, 0x8c, 0x76, 0x56, 0xcd, 0x25, 0x2e, 0x39, 0xc2, 0x17, 0x5f, 0x55, 0x09, 0x11,
        0x39, 0x77,
    ]
    .to_vec();
    let n: P256Element = P256Element {
        x: [
            0x98f30938d11ca647,
            0xd24dea34c2cd3128,
            0x392e25cd56768c23,
            0x77391109555f17c2,
        ],
    };
    let actual: Vec<u8> = P256Element::into(n);
    assert_eq!(actual, expected);
}

#[test]
fn test_p256_add() {
    let expected: P256Element = P256Element {
        x: [
            0x28a0dc67eae74754,
            0xdd10edbca91db3a5,
            0xe8a975c2e494fdf0,
            0xae3cbc2f6612d12b,
        ],
    };
    let n: P256Element = P256Element {
        x: [
            0x98f30938d11ca647,
            0xd24dea34c2cd3128,
            0x392e25cd56768c23,
            0x77391109555f17c2,
        ],
    };
    let m: P256Element = P256Element {
        x: [
            0x8fadd32f19caa10d,
            0x0ac30387e650827c,
            0xaf7b4ff58e1e71cd,
            0x3703ab2610b3b969,
        ],
    };
    let actual = n + m;
    assert_eq!(actual, expected);
}

#[test]
fn test_p256_add_overflow() {
    let expected: P256Element = P256Element {
        x: [
            0xfffffffffffffff9,
            0x00000000ffffffff,
            0x0000000000000000,
            0xffffffff00000001,
        ],
    };
    let actual = P256_A + P256_A;
    assert_eq!(actual, expected);
}

#[test]
fn test_p256_sub() {
    let expected1: P256Element = P256Element {
        x: [
            0x09453609b752053a,
            0xc78ae6acdc7caeac,
            0x89b2d5d7c8581a56,
            0x403565e344ab5e58,
        ],
    };
    let expected2: P256Element = P256Element {
        x: [
            0xf6bac9f648adfac5,
            0x3875195423835153,
            0x764d2a2837a7e5a9,
            0xbfca9a1bbb54a1a8,
        ],
    };
    let n: P256Element = P256Element {
        x: [
            0x98f30938d11ca647,
            0xd24dea34c2cd3128,
            0x392e25cd56768c23,
            0x77391109555f17c2,
        ],
    };
    let m: P256Element = P256Element {
        x: [
            0x8fadd32f19caa10d,
            0x0ac30387e650827c,
            0xaf7b4ff58e1e71cd,
            0x3703ab2610b3b969,
        ],
    };
    let actual = n - m;
    assert_eq!(actual, expected1);
    let actual = m - n;
    assert_eq!(actual, expected2);
}

#[test]
fn test_p256_sub_overflow() {
    let expected: P256Element = P256Element {
        x: [0x3, 0x0, 0x0, 0x0],
    };
    let n: P256Element = P256Element {
        x: [0x0, 0x0, 0x0, 0x0],
    };
    let actual = n - P256_A;
    assert_eq!(actual, expected);
}

#[test]
fn test_p256_mul() {
    let expected: P256Element = P256Element {
        x: [
            0xae892a213489686c,
            0xb6f1d25e3569310b,
            0xad5553f27083c63e,
            0x3601c5b3196e68e2,
        ],
    };
    let n: P256Element = P256Element {
        x: [
            0x98f30938d11ca647,
            0xd24dea34c2cd3128,
            0x392e25cd56768c23,
            0x77391109555f17c2,
        ],
    };
    let m: P256Element = P256Element {
        x: [
            0x8fadd32f19caa10d,
            0x0ac30387e650827c,
            0xaf7b4ff58e1e71cd,
            0x3703ab2610b3b969,
        ],
    };
    let actual = n * m;
    assert_eq!(actual, expected);
}

#[test]
fn test_p256_mul_overflow() {
    let expected: P256Element = P256Element {
        x: [0x2, 0x0, 0x0, 0x0],
    };
    let n: P256Element = P256Element {
        x: [
            0xfffffffffffffffe,
            0x00000000ffffffff,
            0x0000000000000000,
            0xffffffff00000001,
        ],
    };
    let m: P256Element = P256Element {
        x: [
            0xfffffffffffffffd,
            0x00000000ffffffff,
            0x0000000000000000,
            0xffffffff00000001,
        ],
    };
    let actual = n * m;
    assert_eq!(actual, expected);
}

#[test]
fn test_p256_mul_const() {
    let expected: P256Element = P256Element {
        x: [
            0xc79849c688e5323b,
            0x926f51a316698944,
            0xc9712e6ab3b4611e,
            0xb9c8884daaf8be0e,
        ],
    };
    let n: P256Element = P256Element {
        x: [
            0x98f30938d11ca647,
            0xd24dea34c2cd3128,
            0x392e25cd56768c23,
            0x77391109555f17c2,
        ],
    };
    let actual = n * 8;
    assert_eq!(actual, expected);
}

#[test]
fn test_p256_mul_const_overflow() {
    let expected: P256Element = P256Element {
        x: [
            0xffffffffffffffe7,
            0x00000000ffffffff,
            0x0000000000000000,
            0xffffffff00000001,
        ],
    };
    let actual = P256_A * 8;
    assert_eq!(actual, expected);
}

#[test]
fn test_p256_sqr() {
    let expected: P256Element = P256Element {
        x: [
            0x4d7215408d8ebe64,
            0x0d1006ab74962f42,
            0xcc8fdebe058112ff,
            0x23203afdbb49b5f0,
        ],
    };
    let n: P256Element = P256Element {
        x: [
            0x98f30938d11ca647,
            0xd24dea34c2cd3128,
            0x392e25cd56768c23,
            0x77391109555f17c2,
        ],
    };
    let actual = n.sqr();
    assert_eq!(actual, expected);
}

#[test]
fn test_p256_sqr_overflow() {
    let expected: P256Element = P256Element {
        x: [0x9, 0x0, 0x0, 0x0],
    };
    let actual = P256_A.sqr();
    assert_eq!(actual, expected);
}

#[test]
fn test_p256_pt_normalize() {
    let expected: P256Point = P256Point {
        x: P256Element {
            x: [
                0x8cf578243482c049,
                0xf0c77fb9935305bc,
                0xff316efd0b12401a,
                0x9a1f175351e87924,
            ],
        },
        y: P256Element {
            x: [
                0xfe103b463af41f7a,
                0x3a82e575de0b4f5c,
                0xd5e2a9e650725b30,
                0xdd086ac52f565094,
            ],
        },
        z: P256_ONE,
    };
    let p: P256Point = P256Point {
        x: P256_G.x,
        y: P256_G.y,
        z: P256Element {
            x: [0x3, 0x0, 0x0, 0x0],
        },
    };
    let actual = p.normalize();
    assert_eq!(actual, expected);
}

#[test]
fn test_p256_pt_double() {
    let expected: P256Point = P256Point {
        x: P256Element {
            x: [
                0xa60b48fc47669978,
                0xc08969e277f21b35,
                0x8a52380304b51ac3,
                0x7cf27b188d034f7e,
            ],
        },
        y: P256Element {
            x: [
                0x9e04b79d227873d1,
                0xba7dade63ce98229,
                0x293d9ac69f7430db,
                0x07775510db8ed040,
            ],
        },
        z: P256_ONE,
    };
    let actual = P256_G.double().normalize();
    assert_eq!(actual, expected);
}

#[test]
fn test_p256_pt_add() {
    let expected: P256Point = P256Point {
        x: P256Element {
            x: [
                0xfb41661bc6e7fd6c,
                0xe6c6b721efada985,
                0xc8f7ef951d4bf165,
                0x5ecbe4d1a6330a44,
            ],
        },
        y: P256Element {
            x: [
                0x9a79b127a27d5032,
                0xd82ab036384fb83d,
                0x374b06ce1a64a2ec,
                0x8734640c4998ff7e,
            ],
        },
        z: P256_ONE,
    };
    let n: P256Point = P256Point {
        x: P256Element {
            x: [
                0xa60b48fc47669978,
                0xc08969e277f21b35,
                0x8a52380304b51ac3,
                0x7cf27b188d034f7e,
            ],
        },
        y: P256Element {
            x: [
                0x9e04b79d227873d1,
                0xba7dade63ce98229,
                0x293d9ac69f7430db,
                0x07775510db8ed040,
            ],
        },
        z: P256_ONE,
    };
    let actual = (n + P256_G).normalize();
    assert_eq!(actual, expected);
}

#[test]
fn test_p256_pt_mul() {
    let expected: P256Point = P256Point {
        x: P256Element {
            x: [
                0x31aaee5563e0e8b3,
                0x1a46bea87c4323a6,
                0x7aa96e981a21dc85,
                0x4577970b464a4920,
            ],
        },
        y: P256Element {
            x: [
                0x11782ec0c78d507c,
                0xef9cc511aa9f24a2,
                0x79adf83eb3dde528,
                0x736d4e4e6584fab6,
            ],
        },
        z: P256_ONE,
    };
    let n: [u8; 32] = [
        0x5e, 0xda, 0x47, 0x80, 0x6a, 0x6d, 0xff, 0xbf, 0x44, 0xdf, 0x82, 0x61, 0xac, 0x74, 0x29,
        0xf6, 0x5d, 0x18, 0xed, 0xc2, 0x32, 0x4a, 0xc4, 0x84, 0xa3, 0x69, 0x61, 0xc3, 0xdd, 0x23,
        0x14, 0xf4,
    ];
    let actual = (P256_G * &n).normalize();
    assert_eq!(actual, expected);
}

#[test]
fn test_p256_pt_mul_order() {
    let n: [u8; 32] = [
        0x51, 0x25, 0x63, 0xfc, 0xc2, 0xca, 0xb9, 0xf3, 0x84, 0x9e, 0x17, 0xa7, 0xad, 0xfa, 0xe6,
        0xbc, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff,
        0xff, 0xff,
    ];
    let actual = (P256_G * &n).normalize();
    assert!(actual.is_point_at_infinity());
}
