from hashlib import sha512

from fastecdsa.rust import edwards25519_mul

L = 0x1000000000000000000000000000000014DEF9DEA2F79CD65812631A5CF5D3ED


def sign_ed25519(sk: bytes, msg: bytes):
    hashed = sha512(sk).digest()

    x = bytearray(hashed[:32])
    x[0] &= 0b1111_1000
    x[31] &= 0b0111_1111
    x[31] |= 0b0100_0000
    a = edwards25519_mul(bytes(x))

    r = int.from_bytes(sha512(hashed[32:] + msg).digest(), "little") % L
    r_bytes = edwards25519_mul(r.to_bytes(32, "little"))

    k = int.from_bytes(sha512(r_bytes + a + msg).digest(), "little") % L
    s = (r + k * int.from_bytes(x, "little")) % L
    s_bytes = s.to_bytes(32, "little")

    return r_bytes + s_bytes
