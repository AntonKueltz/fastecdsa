use std::str::FromStr;

use num_bigint::BigInt;

use crate::edwards_wnaf::*;

#[test]
fn test_lagrange() {
    let n = BigInt::from_str(
        "7237005577332262213973186563042994240857116359379907606001950938285454250989",
    )
    .unwrap();
    let k = BigInt::from_str(
        "6682786107049885749401341300633611636291510781895386611944124961911771452520",
    )
    .unwrap();

    let expected_d0 = BigInt::from_str("-8445764170423175551664033641582400043").unwrap();
    let expected_d1 = BigInt::from_str("31676400192244112621556654502937037988").unwrap();
    let (d0, d1) = lagrange(&n, &(&n * &n), &k);

    assert_eq!(d0, expected_d0);
    assert_eq!(d1, expected_d1);
    assert_eq!((d1 * k - d0) % n, BigInt::ZERO);
}

#[test]
fn test_naf1() {
    let x = BigInt::from(101);
    let expected: Vec<i16> = vec![1, 0, 1, 0, 0, -1, 0, 1];
    let actual = naf(&x, 2);
    assert_eq!(actual, expected);
}

#[test]
fn test_naf2() {
    let x = BigInt::from_str("328647527836432718").unwrap();
    let expected: Vec<i16> = vec![
        0, -1, 0, 0, 1, 0, 1, 0, 1, 0, 0, -1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, -1,
        0, 0, -1, 0, 0, 0, 0, -1, 0, 1, 0, -1, 0, 0, -1, 0, 1, 0, -1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0,
        1,
    ];
    let actual = naf(&x, 2);
    assert_eq!(actual, expected);
}

#[test]
fn test_naf3() {
    let x = BigInt::from_str("1122334455").unwrap();
    let expected: Vec<i16> = vec![
        -9, 0, 0, 0, 0, 0, 0, 0, -9, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0,
        0, 1,
    ];
    let actual = naf(&x, 6);
    assert_eq!(actual, expected);
}
