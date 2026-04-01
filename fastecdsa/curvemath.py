"""Pure-Python curvemath using gmpy2, replacing the C extension."""

from gmpy2 import bit_test, invert, mpz


def _is_identity(x, y):
    return x == 0 and y == 0


def _point_double(x, y, p, a):
    if _is_identity(x, y):
        return mpz(0), mpz(0)

    denom = (2 * y) % p
    try:
        denom = invert(denom, p)
    except ZeroDivisionError:
        return mpz(0), mpz(0)

    lam = ((3 * x * x + a) * denom) % p

    rx = (lam * lam - x - x) % p
    ry = (lam * (x - rx) - y) % p
    return rx, ry


def _point_add(x1, y1, x2, y2, p, a):
    if _is_identity(x1, y1) and _is_identity(x2, y2):
        return mpz(0), mpz(0)
    if _is_identity(x1, y1):
        return x2, y2
    if _is_identity(x2, y2):
        return x1, y1

    if x1 == x2 and y1 == y2:
        return _point_double(x1, y1, p, a)

    if x1 == x2 and y1 == (p - y2) % p:
        return mpz(0), mpz(0)

    xdiff = invert((x2 - x1) % p, p)
    lam = ((y2 - y1) * xdiff) % p

    rx = (lam * lam - x1 - x2) % p
    ry = (lam * (x1 - rx) - y1) % p
    return rx, ry


def _point_mul(x, y, scalar, p, a):
    if _is_identity(x, y):
        return mpz(0), mpz(0)

    r0x, r0y = x, y
    r1x, r1y = _point_double(x, y, p, a)

    dbits = scalar.bit_length()
    for i in range(dbits - 2, -1, -1):
        if bit_test(scalar, i):
            r0x, r0y = _point_add(r1x, r1y, r0x, r0y, p, a)
            r1x, r1y = _point_double(r1x, r1y, p, a)
        else:
            r1x, r1y = _point_add(r0x, r0y, r1x, r1y, p, a)
            r0x, r0y = _point_double(r0x, r0y, p, a)

    return r0x, r0y


def _shamirs_trick(x1, y1, scalar1, x2, y2, scalar2, p, a):
    sx, sy = _point_add(x1, y1, x2, y2, p, a)

    s1bits = scalar1.bit_length()
    s2bits = scalar2.bit_length()
    l = max(s1bits, s2bits) - 1

    b1 = bit_test(scalar1, l)
    b2 = bit_test(scalar2, l)
    if b1 and b2:
        rx, ry = sx, sy
    elif b1:
        rx, ry = x1, y1
    elif b2:
        rx, ry = x2, y2
    else:
        rx, ry = mpz(0), mpz(0)

    for l in range(l - 1, -1, -1):
        rx, ry = _point_double(rx, ry, p, a)
        b1 = bit_test(scalar1, l)
        b2 = bit_test(scalar2, l)
        if b1 and b2:
            rx, ry = _point_add(rx, ry, sx, sy, p, a)
        elif b1:
            rx, ry = _point_add(rx, ry, x1, y1, p, a)
        elif b2:
            rx, ry = _point_add(rx, ry, x2, y2, p, a)

    return rx, ry


def mul(x, y, d, p, a, b, q, gx, gy):
    _x, _y, _d, _p, _a = mpz(x), mpz(y), mpz(d), mpz(p), mpz(a)
    rx, ry = _point_mul(_x, _y, _d, _p, _a)
    return str(rx), str(ry)


def add(px, py, qx, qy, p, a, b, q, gx, gy):
    _px, _py = mpz(px), mpz(py)
    _qx, _qy = mpz(qx), mpz(qy)
    _p, _a = mpz(p), mpz(a)

    if _px == _qx and _py == _qy:
        rx, ry = _point_double(_px, _py, _p, _a)
    else:
        rx, ry = _point_add(_px, _py, _qx, _qy, _p, _a)

    return str(rx), str(ry)
