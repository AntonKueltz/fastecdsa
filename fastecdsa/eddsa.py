from hashlib import sha512
from os import urandom

from fastecdsa.rust import Ed25519Point

P = 0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFED
L = 0x1000000000000000000000000000000014DEF9DEA2F79CD65812631A5CF5D3ED
G = Ed25519Point.g()


def gen_ed25519_keypair() -> tuple[bytes, bytes]:
    sk = urandom(32)
    hashed = sha512(sk).digest()

    x = bytearray(hashed[:32])
    x[0] &= 0b1111_1000
    x[31] &= 0b0111_1111
    x[31] |= 0b0100_0000

    pk = Ed25519Point.scale_base(int.from_bytes(x, "little")).normalize().encode()
    return sk, pk


def sign_ed25519(sk: bytes, msg: bytes) -> bytes:
    hashed = sha512(sk).digest()

    x = bytearray(hashed[:32])
    x[0] &= 0b1111_1000
    x[31] &= 0b0111_1111
    x[31] |= 0b0100_0000
    a = Ed25519Point.scale_base(int.from_bytes(x, "little")).normalize().encode()

    r = int.from_bytes(sha512(hashed[32:] + msg).digest(), "little") % L
    r_bytes = Ed25519Point.scale_base(r).normalize().encode()

    k = int.from_bytes(sha512(r_bytes + a + msg).digest(), "little") % L
    s = (r + k * int.from_bytes(x, "little")) % L
    s_bytes = s.to_bytes(32, "little")

    return r_bytes + s_bytes


def verify_ed25519(sig: bytes, msg: bytes, a: bytes) -> bool:
    if len(sig) != 64:
        return False

    r_bytes = sig[:32]
    try:
        r = Ed25519Point.decode(r_bytes)
    except ValueError as e:
        print(f"R decode error: {e}")
        return False

    s_bytes = sig[32:]
    s = int.from_bytes(s_bytes, "little")

    if not (0 <= s < L):
        return False

    try:
        a_ = Ed25519Point.decode(a)
    except ValueError as e:
        print(f"A decode error: {e}")
        return False

    k = int.from_bytes(sha512(r_bytes + a + msg).digest(), "little") % L

    return Ed25519Point.scale_base(s) == r + k * a_
