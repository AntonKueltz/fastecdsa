from binascii import hexlify
from hashlib import sha256  # Python standard lib SHA2 is already in C

from fastecdsa import _ecdsa
from .curve import P256
from .point import Point
from .util import RFC6979


class EcdsaError(Exception):
    def __init__(self, msg):
        self.msg = msg


def sign(msg, d, curve=P256, hashfunc=sha256):
    # generate a deterministic nonce per RFC6979
    rfc6979 = RFC6979(msg, d, curve.q, hashfunc)
    k = rfc6979.gen_nonce()

    hashed = hashfunc(msg.encode()).hexdigest()
    r, s = _ecdsa.sign(hashed, str(d), str(k), curve.name)
    return (int(r), int(s))


def verify(sig, msg, Q, curve=P256, hashfunc=sha256):
    if isinstance(Q, tuple):
        Q = Point(Q[0], Q[1], curve)
    r, s = sig

    # validate Q, r, s (Q should be validated in constructor of Point already but double check)
    if not curve.is_point_on_curve((Q.x, Q.y)):
        raise EcdsaError('Invalid public key, point is not on curve {}'.format(curve.name))
    elif r > curve.q or r < 1:
        raise EcdsaError(
            'Invalid Signature: r is not a positive integer smaller than the curve order')
    elif s > curve.q or s < 1:
        raise EcdsaError(
            'Invalid Signature: s is not a positive integer smaller than the curve order')

    hashed = hashfunc(msg.encode()).hexdigest()
    return _ecdsa.verify(str(r), str(s), hashed, str(Q.x), str(Q.y), curve.name)
