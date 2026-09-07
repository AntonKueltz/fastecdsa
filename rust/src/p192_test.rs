use crate::p192::*;

#[test]
fn test_p192_reduce_mul() {
    let expected: Field<P192> = Field::<P192> {
        x: [0x1d971573098a6847, 0xa6548a41ca47a71f, 0x8a4ab71f6cc7e431],
    };
    let n: MulResult<P192> = MulResult::<P192> {
        x: [
            0x853922c6d05a85d8,
            0xac9ec312b6d991b2,
            0x056a05340cb1b062,
            0x74d51543b357e19e,
            0x6157d482da3e32fd,
            0x2388dd6885d800d1,
        ],
    };
    let actual = n.reduce();
    assert_eq!(actual, expected);
}

#[test]
fn test_p192_add() {
    let expected: Field<P192> = Field::<P192> {
        x: [0xa089952d255cdd2a, 0xdbba7de21d7656ed, 0xd76729cc37f69111],
    };
    let n: Field<P192> = Field::<P192> {
        x: [0x2d2b1be584243c86, 0x66ce8e3ae5ab02e4, 0x9dba9c2dc998e2b8],
    };
    let m: Field<P192> = Field::<P192> {
        x: [0x735e7947a138a0a4, 0x74ebefa737cb5409, 0x39ac8d9e6e5dae59],
    };
    let actual = n + m;
    assert_eq!(actual, expected);
}

#[test]
fn test_p192_add_overflow() {
    let expected: Field<P192> = Field::<P192> {
        x: [0xfffffffffffffff9, 0xfffffffffffffffe, 0xffffffffffffffff],
    };
    let actual = P192::A + P192::A;
    assert_eq!(actual, expected);
}

#[test]
fn test_p192_sub() {
    let expected1: Field<P192> = Field::<P192> {
        x: [0x7e3c52c04b6f31e2, 0xc266a5b17cba31b0, 0x78606659f6705e81],
    };
    let expected2: Field<P192> = Field::<P192> {
        x: [0x81c3ad3fb490ce1d, 0x3d995a4e8345ce4e, 0x879f99a6098fa17e],
    };
    let n: Field<P192> = Field::<P192> {
        x: [0x8b835b1d28af72bf, 0x07091bce0c8df610, 0x0bd6c66f205f074d],
    };
    let m: Field<P192> = Field::<P192> {
        x: [0x0d47085cdd4040dc, 0x44a2761c8fd3c45f, 0x9376601529eea8cb],
    };
    let actual = n - m;
    assert_eq!(actual, expected1);
    let actual = m - n;
    assert_eq!(actual, expected2);
}

#[test]
fn test_p192_sub_overflow() {
    let expected: Field<P192> = Field::<P192> { x: [0x3, 0x0, 0x0] };
    let n = Field::<P192> { x: [0x0, 0x0, 0x0] };
    let actual = n - P192::A;
    assert_eq!(actual, expected);
}

#[test]
fn test_p192_mul() {
    let expected: Field<P192> = Field::<P192> {
        x: [0x1d971573098a6847, 0xa6548a41ca47a71f, 0x8a4ab71f6cc7e431],
    };
    let n: Field<P192> = Field::<P192> {
        x: [0x2d2b1be584243c86, 0x66ce8e3ae5ab02e4, 0x9dba9c2dc998e2b8],
    };
    let m: Field<P192> = Field::<P192> {
        x: [0x735e7947a138a0a4, 0x74ebefa737cb5409, 0x39ac8d9e6e5dae59],
    };
    let actual = n * m;
    assert_eq!(actual, expected);
}

#[test]
fn test_p192_mul_overflow() {
    let expected: Field<P192> = Field::<P192> { x: [0x2, 0x0, 0x0] };
    let n: Field<P192> = Field::<P192> {
        x: [0xfffffffffffffffe, 0xfffffffffffffffe, 0xffffffffffffffff],
    };
    let m: Field<P192> = Field::<P192> {
        x: [0xfffffffffffffffd, 0xfffffffffffffffe, 0xffffffffffffffff],
    };
    let actual = n * m;
    assert_eq!(actual, expected);
}

#[test]
fn test_p192_mul_const() {
    let expected: Field<P192> = Field::<P192> {
        x: [0x6a3842e6ea0206e4, 0x2513b0e47e9e22fc, 0x9bb300a94f75465a],
    };
    let n: Field<P192> = Field::<P192> {
        x: [0x0d47085cdd4040dc, 0x44a2761c8fd3c45f, 0x9376601529eea8cb],
    };
    let actual = n * 8;
    assert_eq!(actual, expected);
}

#[test]
fn test_p192_mul_const_overflow() {
    let expected: Field<P192> = Field::<P192> {
        x: [0xffffffffffffffe7, 0xfffffffffffffffe, 0xffffffffffffffff],
    };
    let actual = P192::A * 8;
    assert_eq!(actual, expected);
}

#[test]
fn test_p192_sqr() {
    let expected: Field<P192> = Field::<P192> {
        x: [0x688af0c9317b0985, 0x6a0a88edeb0b04cd, 0x6a83ae974272591f],
    };
    let n: Field<P192> = Field::<P192> {
        x: [0x2d2b1be584243c86, 0x66ce8e3ae5ab02e4, 0x9dba9c2dc998e2b8],
    };
    let actual = n.sqr();
    assert_eq!(actual, expected);
}

#[test]
fn test_p192_sqr_overflow() {
    let expected: Field<P192> = Field::<P192> { x: [0x9, 0x0, 0x0] };
    let actual = P192::A.sqr();
    assert_eq!(actual, expected);
}

#[test]
fn test_p192_pt_normalize() {
    let expected: Point<P192> = Point::<P192> {
        x: Field::<P192> {
            x: [0x541c568df21c571e, 0x9c153c8beb11f2aa, 0x7481847368e8f3a9],
        },
        y: Field::<P192> {
            x: [0xcb680de985de90e4, 0xddbe398d88b582e1, 0x2fab99516849cf33],
        },
        z: P192::ONE,
    };
    let p: Point<P192> = Point::<P192> {
        x: Field::<P192> {
            x: [0xf4ff0afd82ff1012, 0x7cbf20eb43a18800, 0x188da80eb03090f6],
        },
        y: Field::<P192> {
            x: [0x73f977a11e794811, 0x631011ed6b24cdd5, 0x07192b95ffc8da78],
        },
        z: Field::<P192> { x: [0x3, 0x0, 0x0] },
    };
    let actual = p.normalize();
    assert_eq!(actual, expected);
}

#[test]
fn test_p192_pt_double() {
    let expected: Point<P192> = Point::<P192> {
        x: Field::<P192> {
            x: [0x29a70fb16982a888, 0xd35534631588a3f6, 0xdafebf5828783f2a],
        },
        y: Field::<P192> {
            x: [0x59331afa5c7e93ab, 0x46b27bbc141b868f, 0xdd6bda0d993da0fa],
        },
        z: P192::ONE,
    };
    let actual = P192::G.double().normalize();
    assert_eq!(actual, expected);
}

#[test]
fn test_p192_pt_add() {
    let expected: Point<P192> = Point::<P192> {
        x: Field::<P192> {
            x: [0x0ace8ecb93d23f2a, 0x98bf5bd1aa667832, 0xa37abc6c431f9ac3],
        },
        y: Field::<P192> {
            x: [0x081f7c5710bc68f0, 0xfed7040a1bbda90e, 0x851b3caec99908db],
        },
        z: P192::ONE,
    };
    let p: Point<P192> = Point::<P192> {
        x: Field::<P192> {
            x: [0x590118ebdd7ff590, 0x3e078d9c300e1605, 0x10bb8e9840049b18],
        },
        y: Field::<P192> {
            x: [0x312b72543cceaea1, 0xadc9f836e62762be, 0x31361008476f917b],
        },
        z: P192::ONE,
    };
    let actual = (p + P192::G).normalize();
    assert_eq!(actual, expected);
}

#[test]
fn test_p192_pt_mul() {
    let expected: Point<P192> = Point::<P192> {
        x: Field::<P192> {
            x: [0x68e728029b0fe24d, 0xb4b2b06dd6150b6d, 0x9678db6b4d967d06],
        },
        y: Field::<P192> {
            x: [0x2e117578d179d852, 0x209fe7ced5fea8c8, 0x7c4b9db69f06a18d],
        },
        z: P192::ONE,
    };
    let n: [u8; 24] = [
        0x27, 0xe5, 0x06, 0x66, 0x93, 0xff, 0x12, 0xec, 0x17, 0x0f, 0x29, 0x4a, 0x65, 0xe1, 0x38,
        0x55, 0x74, 0x6d, 0x68, 0x64, 0x72, 0x63, 0xa4, 0xdb,
    ];
    let actual = (P192::G * &n).normalize();
    assert_eq!(actual, expected);
}
