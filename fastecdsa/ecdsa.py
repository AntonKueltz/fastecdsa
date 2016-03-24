from hashlib import sha256  # Python standard lib SHA2 is already in C

from fastecdsa import _ecdsa


def sign(msg, d):
    hashed = sha256(msg).hexdigest()[-64:]
    r, s = _ecdsa.sign(hashed, str(d), 'P256')
    return int(r), int(s)


def verify(sig, msg, Q):
    hashed = sha256(msg).hexdigest()
