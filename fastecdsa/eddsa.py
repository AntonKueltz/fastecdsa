from hashlib import sha512, shake_256
from os import urandom

from fastecdsa.rust import Ed448Point, Ed25519Point, ed25519_fast_verify

Q25519 = 0x1000000000000000000000000000000014DEF9DEA2F79CD65812631A5CF5D3ED
Q448 = 0x3FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF7CCA23E9C44EDB49AED63690216CC2728DC58F552378C292AB5844F3

H25519 = 126
B_H25519 = Ed25519Point.scale_base(1 << H25519)


def gen_ed25519_keypair() -> tuple[bytes, bytes]:
    sk = urandom(32)
    h = sha512(sk).digest()

    x = bytearray(h[:32])
    x[0] &= 0b1111_1000
    x[31] &= 0b0111_1111
    x[31] |= 0b0100_0000

    pk = Ed25519Point.scale_base(int.from_bytes(x, "little")).normalize().encode()
    return sk, pk


def sign_ed25519(sk: bytes, msg: bytes) -> bytes:
    h = sha512(sk).digest()

    x = bytearray(h[:32])
    x[0] &= 0b1111_1000
    x[31] &= 0b0111_1111
    x[31] |= 0b0100_0000
    b = int.from_bytes(x, "little")
    a = Ed25519Point.scale_base(b).normalize().encode()

    r = int.from_bytes(sha512(h[32:] + msg).digest(), "little") % Q25519
    r_bytes = Ed25519Point.scale_base(r).normalize().encode()

    k = int.from_bytes(sha512(r_bytes + a + msg).digest(), "little") % Q25519
    s = (r + k * b) % Q25519
    s_bytes = s.to_bytes(32, "little")

    return r_bytes + s_bytes


def verify_ed25519(sig: bytes, msg: bytes, a: bytes) -> bool:
    if len(sig) != 64:
        return False

    r_bytes = sig[:32]
    try:
        r = Ed25519Point.decode(r_bytes)
    except ValueError:
        return False

    s_bytes = sig[32:]
    s = int.from_bytes(s_bytes, "little")

    if not (0 <= s < Q25519):
        return False

    try:
        a_ = Ed25519Point.decode(a)
    except ValueError:
        return False

    k = int.from_bytes(sha512(r_bytes + a + msg).digest(), "little") % Q25519

    #  return Ed25519Point.scale_base(s) == r + k * a_
    return ed25519_fast_verify(s, k, r.x, r.y, a_.x, a_.y)


def gen_ed448_keypair() -> tuple[bytes, bytes]:
    sk = urandom(57)
    h = shake_256(sk).digest(114)

    x = bytearray(h[:57])
    x[0] &= 0b1111_1100
    x[56] = 0b0000_0000
    x[55] |= 0b1000_0000

    s = int.from_bytes(bytes(x), "little") % Q448
    pk = Ed448Point.scale_base(s).normalize().encode()
    return sk, pk


def _dom4_ed448(ctx: bytes) -> bytes:
    return b"SigEd448" + bytes([0, len(ctx)]) + ctx


def sign_ed448(sk: bytes, msg: bytes, ctx: bytes = b"") -> bytes:
    if len(ctx) > 255:
        raise ValueError("Length on context must be <= 255")
    dom4 = _dom4_ed448(ctx)

    h = shake_256(sk).digest(114)

    x = bytearray(h[:57])
    x[0] &= 0b1111_1100
    x[56] = 0b0000_0000
    x[55] |= 0b1000_0000
    b = int.from_bytes(bytes(x), "little") % Q448
    a = Ed448Point.scale_base(b).normalize().encode()

    r = int.from_bytes(shake_256(dom4 + h[57:] + msg).digest(114), "little") % Q448
    r_bytes = Ed448Point.scale_base(r).normalize().encode()

    k = int.from_bytes(shake_256(dom4 + r_bytes + a + msg).digest(114), "little") % Q448
    s = (r + k * b) % Q448
    s_bytes = s.to_bytes(57, "little")

    return r_bytes + s_bytes


def verify_ed448(sig: bytes, msg: bytes, a: bytes, ctx: bytes = b"") -> bool:
    if len(sig) != 114:
        return False

    r_bytes = sig[:57]
    try:
        r = Ed448Point.decode(r_bytes)
    except ValueError:
        return False

    s_bytes = sig[57:]
    s = int.from_bytes(s_bytes, "little")

    if not (0 <= s < Q448):
        return False

    try:
        a_ = Ed448Point.decode(a)
    except ValueError:
        return False

    dom4 = _dom4_ed448(ctx)
    k = int.from_bytes(shake_256(dom4 + r_bytes + a + msg).digest(114), "little") % Q448

    return Ed448Point.scale_base(s) == r + k * a_
