from binascii import b2a_uu
from os import urandom
from sys import argv
from timeit import timeit

from .curve import P192, P224, P256, P384, P521, secp256k1
from .ecdsa import sign, verify
from .keys import gen_keypair


def sign_and_verify(d, Q, curve):
    msg = b2a_uu(urandom(32))
    sig = sign(msg, d, curve=curve)
    assert verify(sig, msg, Q, curve=curve)


if __name__ == '__main__':
    iterations = 1000

    for curve in [P192, P224, P256, P384, P521, secp256k1]:
        d, Q = gen_keypair(curve)
        time = timeit(stmt=lambda: sign_and_verify(d, Q, curve), number=iterations)
        print('{} signatures and verifications with curve {} took {:.2f} seconds'.format(
            iterations, curve.name, time))
