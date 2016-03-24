from curve import Curve


class Point:
    def __init__(self, x, y, curve_name):
        self.x = x
        self.y = y
        self.curve = Curve(curve_name)

    def __mul__(self, scalar):
        coords = self.curve.pointMul(self, scalar)
        return Point(coords[0], coords[1], self.curve.name)

    def __rmul__(self, scalar):
        return self * scalar
