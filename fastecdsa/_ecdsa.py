"""Pure-Python ECDSA using gmpy2, replacing the C extension."""

from gmpy2 import invert, mpz

from fastecdsa.curvemath import _point_mul, _shamirs_trick


def sign(msg, d, k, p, a, b, q, gx, gy):
    _p, _a, _q = mpz(p), mpz(a), mpz(q)
    _gx, _gy = mpz(gx), mpz(gy)
    _d, _k = mpz(d), mpz(k)

    # R = k * G, r = R.x mod q
    rx, _ = _point_mul(_gx, _gy, _k, _p, _a)
    r = rx % _q

    # convert hex digest to integer
    e = mpz(msg, 16)
    order_bits = _q.bit_length()
    digest_bits = len(msg) * 4

    if digest_bits > order_bits:
        e >>= digest_bits - order_bits

    # s = k^-1 * (e + d * r) mod q
    kinv = invert(_k, _q)
    s = (kinv * (e + _d * r)) % _q

    return str(r), str(s)


def verify(r, s, msg, qx, qy, p, a, b, q, gx, gy):
    _r, _s = mpz(r), mpz(s)
    _p, _a, _q = mpz(p), mpz(a), mpz(q)
    _gx, _gy = mpz(gx), mpz(gy)
    _qx, _qy = mpz(qx), mpz(qy)

    # convert hex digest to integer
    e = mpz(msg, 16)
    order_bits = _q.bit_length()
    digest_bits = len(msg) * 4

    if digest_bits > order_bits:
        e >>= digest_bits - order_bits

    w = invert(_s, _q)
    u1 = (e * w) % _q
    u2 = (_r * w) % _q

    tx, _ = _shamirs_trick(_gx, _gy, u1, _qx, _qy, u2, _p, _a)
    tx = tx % _q

    return tx == _r
