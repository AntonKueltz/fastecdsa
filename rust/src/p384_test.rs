use crate::p384::*;

#[test]
fn test_p384_mul_reduce() {
    let expected: P384Element = P384Element {
        x: [
            0x781aeaba7bc96200,
            0xefa481fda9ee8d39,
            0xb466d1b74e0c9e8f,
            0xc32d66406259e329,
            0x38611010cc57145b,
            0x375ac19f39a9d9b1,
        ],
    };
    let n: P384MulResult = P384MulResult {
        x: [
            0x8a89eb16c9de6ddc,
            0x14ecd97053d488a8,
            0x99838c06ec517664,
            0xd54e9a24772bafd0,
            0xce9ddb19beadf7d3,
            0x8188b282c08e925e,
            0xddcd750e4e3a8738,
            0xaabc1fe040aaf66d,
            0xb48846922eb6ed01,
            0x2c8307288bbdd4d0,
            0x806a09b056ecf01c,
            0x73a68ce141447509,
        ],
    };
    let actual = n.reduce();
    assert_eq!(actual, expected);
}

#[test]
fn test_p384_from_bytes() {
    let expected: P384Element = P384Element {
        x: [
            0xec16fc4c3e7d795f,
            0x555fff21f283837f,
            0x03b0bb7a8fb0db14,
            0x86187e4ed0552fca,
            0x423b701e5ca5ec58,
            0xc4d446e75f9a8ec8,
        ],
    };
    let n: &[u8] = &[
        0x5f, 0x79, 0x7d, 0x3e, 0x4c, 0xfc, 0x16, 0xec, 0x7f, 0x83, 0x83, 0xf2, 0x21, 0xff, 0x5f,
        0x55, 0x14, 0xdb, 0xb0, 0x8f, 0x7a, 0xbb, 0xb0, 0x03, 0xca, 0x2f, 0x55, 0xd0, 0x4e, 0x7e,
        0x18, 0x86, 0x58, 0xec, 0xa5, 0x5c, 0x1e, 0x70, 0x3b, 0x42, 0xc8, 0x8e, 0x9a, 0x5f, 0xe7,
        0x46, 0xd4, 0xc4,
    ];
    let actual = P384Element::from(n);
    assert_eq!(actual, expected);
}

#[test]
fn test_p384_to_bytes() {
    let expected: Vec<u8> = [
        0x5f, 0x79, 0x7d, 0x3e, 0x4c, 0xfc, 0x16, 0xec, 0x7f, 0x83, 0x83, 0xf2, 0x21, 0xff, 0x5f,
        0x55, 0x14, 0xdb, 0xb0, 0x8f, 0x7a, 0xbb, 0xb0, 0x03, 0xca, 0x2f, 0x55, 0xd0, 0x4e, 0x7e,
        0x18, 0x86, 0x58, 0xec, 0xa5, 0x5c, 0x1e, 0x70, 0x3b, 0x42, 0xc8, 0x8e, 0x9a, 0x5f, 0xe7,
        0x46, 0xd4, 0xc4,
    ]
    .to_vec();
    let n: P384Element = P384Element {
        x: [
            0xec16fc4c3e7d795f,
            0x555fff21f283837f,
            0x03b0bb7a8fb0db14,
            0x86187e4ed0552fca,
            0x423b701e5ca5ec58,
            0xc4d446e75f9a8ec8,
        ],
    };
    let actual: Vec<u8> = P384Element::into(n);
    assert_eq!(actual, expected);
}

#[test]
fn test_p384_add() {
    let expected: P384Element = P384Element {
        x: [
            0x91318fcf22e6ed04,
            0x56df65c86b8def39,
            0x26c62d52436194f1,
            0x2b1b40b2789cf845,
            0x8c3a3b77c14f69b8,
            0x5b3f25e168435d94,
        ],
    };
    let n: P384Element = P384Element {
        x: [
            0xec16fc4c3e7d795f,
            0x555fff21f283837f,
            0x03b0bb7a8fb0db14,
            0x86187e4ed0552fca,
            0x423b701e5ca5ec58,
            0xc4d446e75f9a8ec8,
        ],
    };
    let m: P384Element = P384Element {
        x: [
            0xa51a9383e46973a4,
            0x017f66a5790a6bb9,
            0x231571d7b3b0b9dc,
            0xa502c263a847c87b,
            0x49fecb5964a97d5f,
            0x966adefa08a8cecc,
        ],
    };
    let actual = n + m;
    assert_eq!(actual, expected);
}

#[test]
fn test_p384_add_overflow() {
    let expected: P384Element = P384Element {
        x: [
            0x00000000fffffff9,
            0xffffffff00000000,
            0xfffffffffffffffe,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
        ],
    };
    let actual = P384_A + P384_A;
    assert_eq!(actual, expected);
}

#[test]
fn test_p384_sub() {
    let expected1: P384Element = P384Element {
        x: [
            0x46fc68c85a1405bb,
            0x53e0987c797917c6,
            0xe09b49a2dc002138,
            0xe115bbeb280d674e,
            0xf83ca4c4f7fc6ef8,
            0x2e6967ed56f1bffb,
        ],
    };
    let expected2: P384Element = P384Element {
        x: [
            0xb9039738a5ebfa44,
            0xac1f67828686e839,
            0x1f64b65d23ffdec6,
            0x1eea4414d7f298b1,
            0x07c35b3b08039107,
            0xd1969812a90e4004,
        ],
    };
    let n: P384Element = P384Element {
        x: [
            0xec16fc4c3e7d795f,
            0x555fff21f283837f,
            0x03b0bb7a8fb0db14,
            0x86187e4ed0552fca,
            0x423b701e5ca5ec58,
            0xc4d446e75f9a8ec8,
        ],
    };
    let m: P384Element = P384Element {
        x: [
            0xa51a9383e46973a4,
            0x017f66a5790a6bb9,
            0x231571d7b3b0b9dc,
            0xa502c263a847c87b,
            0x49fecb5964a97d5f,
            0x966adefa08a8cecc,
        ],
    };
    let actual = n - m;
    assert_eq!(actual, expected1);
    let actual = m - n;
    assert_eq!(actual, expected2);
}

#[test]
fn test_p384_sub_overflow() {
    let expected: P384Element = P384Element {
        x: [0x3, 0x0, 0x0, 0x0, 0x0, 0x0],
    };
    let n: P384Element = P384Element {
        x: [0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
    };
    let actual = n - P384_A;
    assert_eq!(actual, expected);
}

#[test]
fn test_p384_mul() {
    let expected: P384Element = P384Element {
        x: [
            0x781aeaba7bc96200,
            0xefa481fda9ee8d39,
            0xb466d1b74e0c9e8f,
            0xc32d66406259e329,
            0x38611010cc57145b,
            0x375ac19f39a9d9b1,
        ],
    };
    let n: P384Element = P384Element {
        x: [
            0xec16fc4c3e7d795f,
            0x555fff21f283837f,
            0x03b0bb7a8fb0db14,
            0x86187e4ed0552fca,
            0x423b701e5ca5ec58,
            0xc4d446e75f9a8ec8,
        ],
    };
    let m: P384Element = P384Element {
        x: [
            0xa51a9383e46973a4,
            0x017f66a5790a6bb9,
            0x231571d7b3b0b9dc,
            0xa502c263a847c87b,
            0x49fecb5964a97d5f,
            0x966adefa08a8cecc,
        ],
    };
    let actual = n * m;
    assert_eq!(actual, expected);
}

#[test]
fn test_p384_mul_overflow() {
    let expected: P384Element = P384Element {
        x: [0x2, 0x0, 0x0, 0x0, 0x0, 0x0],
    };
    let n: P384Element = P384Element {
        x: [
            0x00000000fffffffe,
            0xffffffff00000000,
            0xfffffffffffffffe,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
        ],
    };
    let m: P384Element = P384Element {
        x: [
            0x00000000fffffffd,
            0xffffffff00000000,
            0xfffffffffffffffe,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
        ],
    };
    let actual = n * m;
    assert_eq!(actual, expected);
}

#[test]
fn test_p384_mul_const() {
    let expected: P384Element = P384Element {
        x: [
            0x60b7e25bf3ebcafe,
            0xaafff915941c1bff,
            0x1d85dbd47d86d8a8,
            0x30c3f27682a97e50,
            0x11db80f2e52f62c4,
            0x26a2373afcd47642,
        ],
    };
    let n: P384Element = P384Element {
        x: [
            0xec16fc4c3e7d795f,
            0x555fff21f283837f,
            0x03b0bb7a8fb0db14,
            0x86187e4ed0552fca,
            0x423b701e5ca5ec58,
            0xc4d446e75f9a8ec8,
        ],
    };
    let actual = n * 8;
    assert_eq!(actual, expected);
}

#[test]
fn test_p384_mul_const_overflow() {
    let expected: P384Element = P384Element {
        x: [
            0x00000000ffffffe7,
            0xffffffff00000000,
            0xfffffffffffffffe,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
        ],
    };
    let actual = P384_A * 8;
    assert_eq!(actual, expected);
}

#[test]
fn test_p384_sqr() {
    let expected: P384Element = P384Element {
        x: [
            0xf6aff7f3676e0760,
            0xee9eff734d3f9583,
            0x32aa587fed4a4128,
            0x5ea0dddf8460c5aa,
            0xb6fa36967ac7191f,
            0xdb4f1dcd6dc0ecb2,
        ],
    };
    let n: P384Element = P384Element {
        x: [
            0xec16fc4c3e7d795f,
            0x555fff21f283837f,
            0x03b0bb7a8fb0db14,
            0x86187e4ed0552fca,
            0x423b701e5ca5ec58,
            0xc4d446e75f9a8ec8,
        ],
    };
    let actual = n.sqr();
    assert_eq!(actual, expected);
}

#[test]
fn test_p384_sqr_overflow() {
    let expected: P384Element = P384Element {
        x: [0x9, 0x0, 0x0, 0x0, 0x0, 0x0],
    };
    let actual = P384_A.sqr();
    assert_eq!(actual, expected);
}

#[test]
fn test_p384_pt_normalize() {
    let expected: P384Point = P384Point {
        x: P384Element {
            x: [
                0x5bd07c3f459b5686,
                0xb41cc59887097661,
                0x09ff0751d59792e9,
                0x619178604867f4d8,
                0x2c4ca458c5ae4c29,
                0x2f646bcaf8ba1d06,
            ],
        },
        y: P384Element {
            x: [
                0x177dbeb9b008ab32,
                0x55b7babadb2a9c81,
                0x1b9fc8edc45e3fba,
                0x4b974378c89d690e,
                0xe705dfbb3e514a86,
                0xef0a5d90fc146075,
            ],
        },
        z: P384_ONE,
    };
    let p: P384Point = P384Point {
        x: P384_G.x,
        y: P384_G.y,
        z: P384Element {
            x: [0x3, 0x0, 0x0, 0x0, 0x0, 0x0],
        },
    };
    let actual = p.normalize();
    assert_eq!(actual, expected);
}

#[test]
fn test_p384_pt_double() {
    let expected: P384Point = P384Point {
        x: P384Element {
            x: [
                0x5b96a9c75295df61,
                0x4fe0e86ebe0e64f8,
                0x51d207d19fb96e9e,
                0x89025959a6f434d6,
                0x69260045c55b97f0,
                0x08d999057ba3d2d9,
            ],
        },
        y: P384Element {
            x: [
                0x61501e700a940e80,
                0x5ffd43e94d39e22d,
                0x904e505f256ab425,
                0xb275d875bc6cc43e,
                0xb7bfe8dffd6dba74,
                0x8e80f1fa5b1b3ced,
            ],
        },
        z: P384_ONE,
    };
    let actual = P384_G.double().normalize();
    assert_eq!(actual, expected);
}

#[test]
fn test_p384_pt_add() {
    let expected: P384Point = P384Point {
        x: P384Element {
            x: [
                0x02d7e5c70500c831,
                0xb408bbae5026580d,
                0xbea4f240d3566da6,
                0xcb9d3910202dcd06,
                0x64793c7e5fdc7d98,
                0x077a41d4606ffa14,
            ],
        },
        y: P384Element {
            x: [
                0xb65f28600a2f1df1,
                0xc24abd6be4b5d298,
                0xf7684c0edc111eac,
                0x8520b41c85115aa5,
                0x7d0bbe9602a9fc99,
                0xc995f7ca0b0c4283,
            ],
        },
        z: P384_ONE,
    };
    let n: P384Point = P384Point {
        x: P384Element {
            x: [
                0x5b96a9c75295df61,
                0x4fe0e86ebe0e64f8,
                0x51d207d19fb96e9e,
                0x89025959a6f434d6,
                0x69260045c55b97f0,
                0x08d999057ba3d2d9,
            ],
        },
        y: P384Element {
            x: [
                0x61501e700a940e80,
                0x5ffd43e94d39e22d,
                0x904e505f256ab425,
                0xb275d875bc6cc43e,
                0xb7bfe8dffd6dba74,
                0x8e80f1fa5b1b3ced,
            ],
        },
        z: P384_ONE,
    };
    let actual = (n + P384_G).normalize();
    assert_eq!(actual, expected);
}

#[test]
fn test_p384_pt_mul() {
    let expected: P384Point = P384Point {
        x: P384Element {
            x: [
                0xfe712caf245e39f4,
                0x4fd25d4784ba695b,
                0xd03097903cf5ccfb,
                0x1efc3df190ecfba3,
                0x9429474358b3b53d,
                0x354429756fd492a2,
            ],
        },
        y: P384Element {
            x: [
                0x2f285d4a305d37e1,
                0x3ceb2e5d4eb3051c,
                0x6e407edf72a3fd35,
                0x789bb2a6ce589cfd,
                0xe84eb6afb0b45236,
                0xd0f89ce5ada72abf,
            ],
        },
        z: P384_ONE,
    };
    let n: [u8; 48] = [
        0x50, 0x8b, 0x23, 0x44, 0x0b, 0x21, 0xf7, 0xb3, 0xe6, 0xf8, 0xfc, 0x5b, 0xbb, 0x31, 0x48,
        0xc9, 0x2d, 0x63, 0xf9, 0xeb, 0x54, 0x0a, 0x87, 0x39, 0x71, 0x87, 0x02, 0x62, 0x77, 0x7d,
        0x63, 0xd5, 0xbd, 0x11, 0xdf, 0x66, 0x87, 0xc1, 0xd4, 0xdc, 0xe0, 0xf8, 0x9b, 0x9e, 0x69,
        0x1f, 0x9f, 0x76,
    ];
    let actual = (P384_G * &n).normalize();
    assert_eq!(actual, expected);
}

#[test]
fn test_p384_pt_mul_order() {
    let n: [u8; 48] = [
        0x73, 0x29, 0xc5, 0xcc, 0x6a, 0x19, 0xec, 0xec, 0x7a, 0xa7, 0xb0, 0x48, 0xb2, 0x0d, 0x1a,
        0x58, 0xdf, 0x2d, 0x37, 0xf4, 0x81, 0x4d, 0x63, 0xc7, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff,
    ];
    let actual = (P384_G * &n).normalize();
    assert!(actual.is_point_at_infinity());
}
