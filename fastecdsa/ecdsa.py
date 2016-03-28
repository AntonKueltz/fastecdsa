from hashlib import sha256  # Python standard lib SHA2 is already in C
from os import urandom

from fastecdsa import _ecdsa


class KeyPair:
    def __init__(self, curve):
        d = int(urandom(32).encode('hex'), 16)
        while d >= curve.q:
            d = int(urandom(32).encode('hex'), 16)

        self.d = d
        self.Q = curve.pointMul(curve.G, d)
        self.curve = curve

    def sign(self, msg):
        hashed = sha256(msg).hexdigest()
        r, s = _ecdsa.sign(hashed, str(self.d), self.curve.name)
        return int(r), int(s)

    def verify(self, sig, msg):
        r, s = sig
        qx, qy = self.Q
        hashed = sha256(msg).hexdigest()
        return _ecdsa.verify(str(r), str(s), hashed, str(qx), str(qy), self.curve.name)
