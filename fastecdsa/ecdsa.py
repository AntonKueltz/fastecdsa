from hashlib import sha256  # Python standard lib SHA2 is already in C
from os import urandom

from fastecdsa import _ecdsa
from .curve import P256
from .util import RFC6979


class EcdsaError(Exception):
    def __init__(self, msg):
        self.msg = msg


def gen_keypair(curve=P256):
    qbits = len(bin(curve.q)) - 2  # -2 for the leading 0b
    key_bytes = ((qbits + 7) / 8) * 8
    shift = (key_bytes * 8 - qbits)

    key = int(urandom(key_bytes).encode('hex'), 16)
    key >>= shift

    # we can't just reduce mod q, introduces bias
    while not (key >= 1 and key < curve.q):
        key = int(urandom(key_bytes).encode('hex'), 16)
        key >>= shift

    return key, curve.point_mul(curve.G, key)


def sign(msg, d, curve=P256, hashfunc=sha256):
    # generate a deterministic nonce per RFC6979
    rfc6979 = RFC6979(msg, d, curve.q, hashfunc)
    k = rfc6979.gen_nonce()

    hashed = hashfunc(msg).digest()
    r, s = _ecdsa.sign(hashed.encode('hex'), str(d), str(k), curve.name)
    return (int(r), int(s))


def verify(sig, msg, Q, curve=P256, hashfunc=sha256):
    r, s = sig

    # validate Q, r, s
    if not curve.is_point_on_curve(Q):
        raise EcdsaError('Invalid public key, point is not on curve {}'.format(curve.name))
    elif r > curve.q or r < 1:
        raise EcdsaError('Invalid Signature: r is not a positive integer smaller than the curve order')
    elif s > curve.q or s < 1:
        raise EcdsaError('Invalid Signature: s is not a positive integer smaller than the curve order')

    qx, qy = Q
    hashed = hashfunc(msg).hexdigest()
    return _ecdsa.verify(str(r), str(s), hashed, str(qx), str(qy), curve.name)
