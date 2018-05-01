from hashlib import sha256

from .curve import P192, P224, P256, P384, P521, secp256k1
from .ecsda import sign as _sign, verify as _verify
from .keys import gen_private_key, get_public_key

NIST192p = P192
NIST224p = P224
NIST256p = P256
NIST384p = P384
NIST521p = P521
SECP256k1 = secp256k1


class SigningKey:
    def __init__(self, curve):
        self.curve = curve
        self.key = gen_private_key(curve)

    @classmethod
    def generate(cls, curve=None):
        if curve is None:
            curve = P256

        return cls(curve)

    def sign(self, msg, hashfunc=sha256):
        r, s = _sign(msg, self.key, curve=self.curve, hashfunc=hashfunc)

    def get_verifying_key(self):
        return VerifyingKey(self.curve, self.key)


class VerifyingKey:
    def __init_(self, curve, d):
        self.curve = curve
        self.key = get_public_key(d, curve)

    def verify(self, sig, msg, hashfunc=sha256):
        return _verify(sig, msg, self.key, curve=self.curve, hashfunc=hashfunc)
