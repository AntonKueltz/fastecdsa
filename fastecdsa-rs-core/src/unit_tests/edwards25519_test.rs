use crate::edwards25519::*;

#[test]
fn test_arithmetic() {
    // 0x4361686ada7d9ecdc95a22bc39448cc3585e383bd00db9e1861ad35f488d603a
    let a = Field25519 {
        x: [
            795356145999930,
            2104472624705731,
            1427385360677088,
            2083457179147804,
            1185370283550681,
        ],
    };
    // 0x24992da16de82c00ec985923fd1b52cc91fb9e26e301713377dddc0e05e3543
    let b = Field25519 {
        x: [
            1651195351283011,
            1211412754097903,
            2048204189499111,
            387059848235295,
            40240314150376,
        ],
    };
    // 0x24992da16de82c00ec985923fd1b52cc91fb9e26e301713377dddc0e05e3543
    let c = Field25519 {
        x: [
            1682518939191880,
            370909906630007,
            256293600291924,
            556803624699819,
            1684721971667814,
        ],
    };
    // c * (a + b) (mod p)
    // 0x3ee1b40d79bc87c9a4c95fc05a1f28b0f36b1f9e7ed07a483772b7aa2e90b105
    let expected = Field25519 {
        x: [
            764891506979077,
            1072960396396270,
            137038511582334,
            1096017213644845,
            1106225749531592,
        ],
    };

    let actual = c * (a + b);
    assert_eq!(actual, expected);
}

#[test]
fn test_bytes_to_field25519() {
    // 0x4361686ada7d9ecdc95a22bc39448cc3585e383bd00db9e1861ad35f488d603a
    let bytes: [u8; BYTES] = [
        0x3a, 0x60, 0x8d, 0x48, 0x5f, 0xd3, 0x1a, 0x86, 0xe1, 0xb9, 0x0d, 0xd0, 0x3b, 0x38, 0x5e,
        0x58, 0xc3, 0x8c, 0x44, 0x39, 0xbc, 0x22, 0x5a, 0xc9, 0xcd, 0x9e, 0x7d, 0xda, 0x6a, 0x68,
        0x61, 0x43,
    ];
    let expected = Field25519 {
        x: [
            795356145999930,
            2104472624705731,
            1427385360677088,
            2083457179147804,
            1185370283550681,
        ],
    };

    let actual = Field25519::from(bytes.to_vec());
    assert_eq!(actual, expected);
}

#[test]
fn test_field25519_to_bytes() {
    let n: Field25519 = Field25519 {
        x: [
            795356145999930,
            2104472624705731,
            1427385360677088,
            2083457179147804,
            1185370283550681,
        ],
    };
    let expected: [u8; BYTES] = [
        0x3a, 0x60, 0x8d, 0x48, 0x5f, 0xd3, 0x1a, 0x86, 0xe1, 0xb9, 0x0d, 0xd0, 0x3b, 0x38, 0x5e,
        0x58, 0xc3, 0x8c, 0x44, 0x39, 0xbc, 0x22, 0x5a, 0xc9, 0xcd, 0x9e, 0x7d, 0xda, 0x6a, 0x68,
        0x61, 0x43,
    ];

    let actual: [u8; BYTES] = n.into();
    assert_eq!(actual, expected);
}

#[test]
fn test_edwards25519_rfc8032_1() {
    let sk: [u8; BYTES] = [
        0x30, 0x7c, 0x83, 0x86, 0x4f, 0x28, 0x33, 0xcb, 0x42, 0x7a, 0x2e, 0xf1, 0xc0, 0x0a, 0x01,
        0x3c, 0xfd, 0xff, 0x27, 0x68, 0xd9, 0x80, 0xc0, 0xa3, 0xa5, 0x20, 0xf0, 0x6, 0x90, 0x4d,
        0xe9, 0x4f,
    ];
    let expected: [u8; BYTES] = [
        0xd7, 0x5a, 0x98, 0x01, 0x82, 0xb1, 0x0a, 0xb7, 0xd5, 0x4b, 0xfe, 0xd3, 0xc9, 0x64, 0x07,
        0x3a, 0x0e, 0xe1, 0x72, 0xf3, 0xda, 0xa6, 0x23, 0x25, 0xaf, 0x02, 0x1a, 0x68, 0xf7, 0x07,
        0x51, 0x1a,
    ];
    let actual: [u8; BYTES] = (G * &sk).normalize().into();
    assert_eq!(actual, expected);
}
