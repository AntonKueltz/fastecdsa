use crate::p224::*;

#[test]
fn test_p224_mul_reduce() {
    let expected: P224Element = P224Element {
        x: [
            0x44cad88a, 0x493d8833, 0x5f6d8313, 0x57c80298, 0xb7f8707c, 0x647cd0f2, 0xf49af314,
        ],
    };
    let n: P224MulResult = P224MulResult {
        x: [
            0x0370bd88, 0x5f05da84, 0xe3ab4d61, 0x808ebc77, 0x6c50b25b, 0xb327c79b, 0x100fc8c5,
            0xf48550cf, 0x42df9159, 0x815c55bf, 0xe76c9edd, 0xca20942f, 0xd2e8c0f6, 0x02e1748e,
        ],
    };
    let actual = n.reduce();
    assert_eq!(actual, expected);
}

#[test]
fn test_p224_from_bytes() {
    let expected: P224Element = P224Element {
        x: [
            0x44cad88a, 0x493d8833, 0x5f6d8313, 0x57c80298, 0xb7f8707c, 0x647cd0f2, 0xf49af314,
        ],
    };
    let n: &[u8] = &[
        0x8a, 0xd8, 0xca, 0x44, 0x33, 0x88, 0x3d, 0x49, 0x13, 0x83, 0x6d, 0x5f, 0x98, 0x02, 0xc8,
        0x57, 0x7c, 0x70, 0xf8, 0xb7, 0xf2, 0xd0, 0x7c, 0x64, 0x14, 0xf3, 0x9a, 0xf4,
    ];
    let actual = P224Element::from(n);
    assert_eq!(actual, expected);
}

#[test]
fn test_p224_to_bytes() {
    let expected: Vec<u8> = [
        0x8a, 0xd8, 0xca, 0x44, 0x33, 0x88, 0x3d, 0x49, 0x13, 0x83, 0x6d, 0x5f, 0x98, 0x02, 0xc8,
        0x57, 0x7c, 0x70, 0xf8, 0xb7, 0xf2, 0xd0, 0x7c, 0x64, 0x14, 0xf3, 0x9a, 0xf4,
    ]
    .to_vec();
    let n: P224Element = P224Element {
        x: [
            0x44cad88a, 0x493d8833, 0x5f6d8313, 0x57c80298, 0xb7f8707c, 0x647cd0f2, 0xf49af314,
        ],
    };
    let actual: Vec<u8> = P224Element::into(n);
    assert_eq!(actual, expected);
}

#[test]
fn test_p224_add() {
    let expected: P224Element = P224Element {
        x: [
            0x236627aa, 0x08cbc61e, 0xba81db7b, 0xc6215d38, 0x16a55b7d, 0x2f038d07, 0x7c7aac97,
        ],
    };
    let n: P224Element = P224Element {
        x: [
            0x3de54a8e, 0xfd3b4da7, 0xf57e81d4, 0x3a1c81c8, 0x578abf39, 0xd4ca5f1c, 0x063c9f95,
        ],
    };
    let m: P224Element = P224Element {
        x: [
            0xe580dd1c, 0x0b907876, 0xc50359a6, 0x8c04db6f, 0xbf1a9c44, 0x5a392dea, 0x763e0d01,
        ],
    };
    let actual = n + m;
    assert_eq!(actual, expected);
}

#[test]
fn test_p224_add_overflow() {
    let expected: P224Element = P224Element {
        x: [
            0xfffffffb, 0xffffffff, 0xffffffff, 0xfffffffe, 0xffffffff, 0xffffffff, 0xffffffff,
        ],
    };
    let actual = P224_A + P224_A;
    assert_eq!(actual, expected);
}

#[test]
fn test_p224_sub() {
    let expected1: P224Element = P224Element {
        x: [
            0x58646d73, 0xf1aad530, 0x307b282e, 0xae17a658, 0x987022f4, 0x7a913131, 0x8ffe9294,
        ],
    };
    let expected2: P224Element = P224Element {
        x: [
            0xa79b928e, 0x0e552acf, 0xcf84d7d1, 0x51e859a6, 0x678fdd0b, 0x856ecece, 0x70016d6b,
        ],
    };
    let n: P224Element = P224Element {
        x: [
            0x3de54a8e, 0xfd3b4da7, 0xf57e81d4, 0x3a1c81c8, 0x578abf39, 0xd4ca5f1c, 0x063c9f95,
        ],
    };
    let m: P224Element = P224Element {
        x: [
            0xe580dd1c, 0x0b907876, 0xc50359a6, 0x8c04db6f, 0xbf1a9c44, 0x5a392dea, 0x763e0d01,
        ],
    };
    let actual = n - m;
    assert_eq!(actual, expected1);
    let actual = m - n;
    assert_eq!(actual, expected2);
}

#[test]
fn test_p224_sub_overflow() {
    let expected: P224Element = P224Element {
        x: [0x3, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
    };
    let n = P224Element {
        x: [0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
    };
    let actual = n - P224_A;
    assert_eq!(actual, expected);
}

#[test]
fn test_p224_mul() {
    let expected: P224Element = P224Element {
        x: [
            0x44cad88a, 0x493d8833, 0x5f6d8313, 0x57c80298, 0xb7f8707c, 0x647cd0f2, 0xf49af314,
        ],
    };
    let n: P224Element = P224Element {
        x: [
            0x3de54a8e, 0xfd3b4da7, 0xf57e81d4, 0x3a1c81c8, 0x578abf39, 0xd4ca5f1c, 0x063c9f95,
        ],
    };
    let m: P224Element = P224Element {
        x: [
            0xe580dd1c, 0x0b907876, 0xc50359a6, 0x8c04db6f, 0xbf1a9c44, 0x5a392dea, 0x763e0d01,
        ],
    };
    let actual = n * m;
    assert_eq!(actual, expected);
}

#[test]
fn test_p224_mul_overflow() {
    let expected: P224Element = P224Element {
        x: [0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
    };
    let n: P224Element = P224Element {
        x: [
            0x00000000, 0x00000000, 0x00000000, 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff,
        ],
    };
    let m: P224Element = P224Element {
        x: [
            0xffffffff, 0xffffffff, 0xffffffff, 0xfffffffe, 0xffffffff, 0xffffffff, 0xffffffff,
        ],
    };
    let actual = n * m;
    assert_eq!(actual, expected);
}

#[test]
fn test_p224_mul_const() {
    let expected: P224Element = P224Element {
        x: [
            0xef2a5470, 0xe9da6d39, 0xabf40ea7, 0xd0e40e47, 0xbc55f9c9, 0xa652f8e2, 0x31e4fcae,
        ],
    };
    let n: P224Element = P224Element {
        x: [
            0x3de54a8e, 0xfd3b4da7, 0xf57e81d4, 0x3a1c81c8, 0x578abf39, 0xd4ca5f1c, 0x063c9f95,
        ],
    };
    let actual = n * 8;
    assert_eq!(actual, expected);
}

#[test]
fn test_p224_mul_const_overflow() {
    let expected: P224Element = P224Element {
        x: [
            0xffffffe9, 0xffffffff, 0xffffffff, 0xfffffffe, 0xffffffff, 0xffffffff, 0xffffffff,
        ],
    };
    let actual = P224_A * 8;
    assert_eq!(actual, expected);
}

#[test]
fn test_p224_sqr() {
    let expected: P224Element = P224Element {
        x: [
            0x8a59a7d1, 0xaa26ed99, 0x1ea2eddd, 0x024d53cd, 0xfa7f1d9e, 0x04da527c, 0xb7713872,
        ],
    };
    let n: P224Element = P224Element {
        x: [
            0x3de54a8e, 0xfd3b4da7, 0xf57e81d4, 0x3a1c81c8, 0x578abf39, 0xd4ca5f1c, 0x063c9f95,
        ],
    };
    let actual = n.sqr();
    assert_eq!(actual, expected);
}

#[test]
fn test_p224_sqr_overflow() {
    let expected: P224Element = P224Element {
        x: [0x9, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
    };
    let actual = P224_A.sqr();
    assert_eq!(actual, expected);
}

#[test]
fn test_p224_pt_normalize() {
    let expected: P224Point = P224Point {
        x: P224Element {
            x: [
                0x9026ade8, 0x94059c89, 0x7b6ae575, 0x24ab1588, 0x93c948f8, 0xb6a24e2a, 0xdb7356bf,
            ],
        },
        y: P224Element {
            x: [
                0x0e685087, 0xa3bc0e48, 0x7e97f929, 0x2d873d3e, 0xc07311c6, 0x52976025, 0x5c576280,
            ],
        },
        z: P224_ONE,
    };
    let p: P224Point = P224Point {
        x: P224_G.x,
        y: P224_G.y,
        z: P224Element {
            x: [0x3, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
        },
    };
    let actual = p.normalize();
    assert_eq!(actual, expected);
}

#[test]
fn test_p224_pt_double() {
    let expected: P224Point = P224Point {
        x: P224Element {
            x: [
                0x1a704fa6, 0x32d268fd, 0xd16dc180, 0x89474788, 0x98e60e6d, 0x76dcb767, 0x706a46dc,
            ],
        },
        y: P224Element {
            x: [
                0xd2e4e8bb, 0x7acf3709, 0xfca62948, 0x86892849, 0x2a704fa9, 0xbc25e770, 0x1c2b76a7,
            ],
        },
        z: P224_ONE,
    };
    let actual = P224_G.double().normalize();
    assert_eq!(actual, expected);
}

#[test]
fn test_p224_pt_add() {
    let expected: P224Point = P224Point {
        x: P224Element {
            x: [
                0xfd896d04, 0x79fe0d08, 0x75c21802, 0x58b9d2cc, 0x1eff8225, 0xa551d0d3, 0xdf1b1d66,
            ],
        },
        y: P224Element {
            x: [
                0x1981a925, 0x4e1af359, 0x77d31734, 0x30130ddf, 0x4c0aa568, 0xadd0be44, 0xa3f7f03c,
            ],
        },
        z: P224_ONE,
    };
    let p: P224Point = P224Point {
        x: P224Element {
            x: [
                0x1a704fa6, 0x32d268fd, 0xd16dc180, 0x89474788, 0x98e60e6d, 0x76dcb767, 0x706a46dc,
            ],
        },
        y: P224Element {
            x: [
                0xd2e4e8bb, 0x7acf3709, 0xfca62948, 0x86892849, 0x2a704fa9, 0xbc25e770, 0x1c2b76a7,
            ],
        },
        z: P224_ONE,
    };
    let actual = (p + P224_G).normalize();
    assert_eq!(actual, expected);
}

#[test]
fn test_p224_pt_mul() {
    let expected: P224Point = P224Point {
        x: P224Element {
            x: [
                0xc32b2c04, 0x58cc0de7, 0xd3cba915, 0x0904e80d, 0x9bb446dd, 0x78dc4705, 0xdb938d39,
            ],
        },
        y: P224Element {
            x: [
                0xd272b6e8, 0xee325576, 0x304218f3, 0xed2602c1, 0x21867f93, 0x6a53345e, 0xb279687d,
            ],
        },
        z: P224_ONE,
    };
    let n: [u8; 28] = [
        0xa4, 0xab, 0xb3, 0xdb, 0x51, 0x6d, 0xdb, 0xc6, 0xad, 0x35, 0x8f, 0x2f, 0x28, 0x4c, 0x4f,
        0x1c, 0x85, 0xbf, 0x57, 0x64, 0x6f, 0xd4, 0x09, 0xe8, 0x1b, 0xee, 0xb2, 0x17,
    ];
    let actual = (P224_G * &n).normalize();
    assert_eq!(actual, expected);
}
