from hashlib import sha512
from timeit import timeit

from fastecdsa.curve import (
    P192,
    P224,
    P256,
    P384,
    P521,
    Curve,
    brainpoolP160r1,
    brainpoolP192r1,
    brainpoolP224r1,
    brainpoolP256r1,
    brainpoolP320r1,
    brainpoolP384r1,
    brainpoolP512r1,
    secp192k1,
    secp224k1,
    secp256k1,
)
from fastecdsa.ecdsa import sign, verify
from fastecdsa.eddsa import (
    gen_ed25519_keypair,
    gen_ed448_keypair,
    sign_ed25519,
    sign_ed448,
    verify_ed25519,
    verify_ed448,
)
from fastecdsa.point import Point

msg = bytes(32)


def sign_and_verify(d: int, Q: Point, curve: Curve) -> None:
    sig = sign(msg, d, curve=curve)
    assert verify(sig, msg, Q, curve=curve)


def sign_and_verify_ed25519(sk: bytes, pk: bytes, msg: bytes) -> None:
    sig = sign_ed25519(sk, msg)
    assert verify_ed25519(sig, msg, pk)

def sign_and_verify_ed448(sk: bytes, pk: bytes, msg: bytes) -> None:
    sig = sign_ed448(sk, msg)
    assert verify_ed448(sig, msg, pk)


def run() -> None:
    iterations = 1000
    curves = (
        P192,
        P224,
        P256,
        P384,
        P521,
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

    sk, pk = gen_ed25519_keypair()
    time = timeit(stmt=lambda: sign_and_verify_ed25519(sk, pk, msg), number=iterations)
    print(
        f"{iterations} signatures and verifications with ed25519 took {time:.2f} seconds"
    )

    sk, pk = gen_ed448_keypair()
    time = timeit(stmt=lambda: sign_and_verify_ed448(sk, pk, msg), number=iterations)
    print(
        f"{iterations} signatures and verifications with ed448 took {time:.2f} seconds"
    )


if __name__ == "__main__":
    run()
