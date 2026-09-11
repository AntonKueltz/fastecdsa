from timeit import timeit

from fastecdsa.curve import (
    Curve,
    P192,
    P224,
    P256,
    P384,
    P521,
    W25519,
    W448,
    secp192k1,
    secp224k1,
    secp256k1,
    brainpoolP160r1,
    brainpoolP192r1,
    brainpoolP224r1,
    brainpoolP256r1,
    brainpoolP320r1,
    brainpoolP384r1,
    brainpoolP512r1,
)
from fastecdsa.point import Point
from .ecdsa import sign, verify

msg = bytes(32)


def sign_and_verify(d: int, Q: Point, curve: Curve) -> None:
    sig = sign(msg, d, curve=curve)
    assert verify(sig, msg, Q, curve=curve)


def run() -> None:
    iterations = 1000
    curves = (
        P192,
        P224,
        P256,
        P384,
        P521,
        W25519,
        W448,
        secp192k1,
        secp224k1,
        secp256k1,
        brainpoolP160r1,
        brainpoolP192r1,
        brainpoolP224r1,
        brainpoolP256r1,
        brainpoolP320r1,
        brainpoolP384r1,
        brainpoolP512r1,
    )

    for curve in curves:
        d = curve.q - 0xDEAD
        Q = curve.G * d
        time = timeit(stmt=lambda: sign_and_verify(d, Q, curve), number=iterations)
        print(
            f"{iterations} signatures and verifications with curve {curve} took {time:.2f} seconds"
        )


if __name__ == "__main__":
    run()
