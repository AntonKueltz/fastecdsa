from fastecdsa import curvemath


class Curve:
    def __init__(self, name, p, a, b, gx, gy, q):
        self.name = name
        self.p = p
        self.a = a
        self.b = b
        self.G = (gx, gy)
        self.q = q

    def isPointOnCurve(self, P):
        ''' check if a point P (int, int) is on the curve '''
        x, y, = P[0], P[1]
        left = y * y
        right = (x * x * x) + (self.a * x) + self.b
        return (left - right) % self.p == 0

    def pointMul(self, P, d):
        ''' Multiply a point P (int, int) with a scalar (d) '''
        if self.isPointOnCurve(P):
            return map(int, curvemath.mul(str(P[0]), str(P[1]), str(d), self.name))
        else:
            return (0, 0)


# see https://www.nsa.gov/ia/_files/nist-routines.pdf for params
P192 = Curve(
    'P192',
    0xfffffffffffffffffffffffffffffffeffffffffffffffff,
    -3,
    0x64210519e59c80e70fa7e9ab72243049feb8deecc146b9b1,
    0x188da80eb03090f67cbf20eb43a18800f4ff0afd82ff1012,
    0x07192b95ffc8da78631011ed6b24cdd573f977a11e794811,
    0xffffffffffffffffffffffff99def836146bc9b1b4d22831
)
P256 = Curve(
    'P256',
    0xffffffff00000001000000000000000000000000ffffffffffffffffffffffff,
    -3,
    0x5ac635d8aa3a93e7b3ebbd55769886bc651d06b0cc53b0f63bce3c3e27d2604b,
    0x6b17d1f2e12c4247f8bce6e563a440f277037d812deb33a0f4a13945d898c296,
    0x4fe342e2fe1a7f9b8ee7eb4a7c0f9e162bce33576b315ececbb6406837bf51f5,
    0xffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632551
)
