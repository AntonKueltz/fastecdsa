from fastecdsa import curvemath
from .curve import P256


class CurveMismatchError(Exception):
    def __init__(self, curve1, curve2):
        self.msg = 'Tried to add points on two different curves <{}> & <{}>'.format(
            curve1.name, curve2.name)


class Point:
    def __init__(self, x, y, curve=P256):
        if not curve.is_point_on_curve((x, y)):
            raise ValueError('(x, y) coordinates are not on curve <{}>'.format(curve.name))
        else:
            self.x = x
            self.y = y
            self.curve = curve

    def __str__(self):
        return '({:x},\n {:x})\nOn curve <{}>'.format(self.x, self.y, self.curve.name)

    def __unicode__(self):
        return self.__str__()

    def __eq__(self, other):
        return self.x == other.x and self.y == other.y and self.curve is other.curve

    def __add__(self, other):
        if self.curve is not other.curve:
            raise CurveMismatchError(self.curve, other.curve)
        else:
            x, y = curvemath.add(str(self.x), str(self.y), str(other.x), str(other.y),
                                 self.curve.name)
            return Point(int(x), int(y), self.curve)

    def __radd__(self, other):
        return self.__add__(other)

    def __sub__(self, other):
        negative = Point(other.x, -other.y % other.curve.p, other.curve)
        return self.__add__(negative)

    def __rsub__(self, other):
        return self.__sub__(other)

    def __mul__(self, other):
        try:
            d = int(other)
        except ValueError:
            raise TypeError('Curve point multiplication must be by an integer')
        else:
            x, y = curvemath.mul(str(self.x), str(self.y), str(d), self.curve.name)
            return Point(int(x), int(y), self.curve)

    def __rmul__(self, other):
        return self.__mul__(other)
