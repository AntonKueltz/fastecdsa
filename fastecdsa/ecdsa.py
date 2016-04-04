from hashlib import sha256  # Python standard lib SHA2 is already in C
import hmac
from os import urandom

from fastecdsa import _ecdsa
from util import RFC6979

from Crypto.Random.random import randint  # Use python secure random for now


class KeyPair:
    def __init__(self, curve, hashfunc=sha256):
        self.d = randint(2, curve.q - 1)
        self.Q = curve.point_mul(curve.G, self.d)
        self.curve = curve
        self.hashfunc = hashfunc

    def sign(self, msg):
        hashed = self.hashfunc(msg).digest()
        rfc6979 = RFC6979(msg, self.d, self.curve.q, self.hashfunc)
        k = rfc6979.gen_nonce()
        r, s = _ecdsa.sign(hashed.encode('hex'), str(self.d), str(k), self.curve.name)
        return int(r), int(s)

    def verify(self, sig, msg):
        r, s = sig

        if r > self.curve.q or r < 1:
            raise EcdsaError('Invalid Signature: r is not a positive integer smaller than the curve order')
        elif s > self.curve.q or s < 1:
            raise EcdsaError('Invalid Signature: r is not a positive integer smaller than the curve order')

        qx, qy = self.Q
        hashed = self.hashfunc(msg).hexdigest()
        return _ecdsa.verify(str(r), str(s), hashed, str(qx), str(qy), self.curve.name)


class EcdsaError(Exception):
    def __init__(self, msg):
        self.msg = msg
