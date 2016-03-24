from Curve import Curve


class Point:
    def __init__(self, x, y, curve="P256"):
        self.x = x
        self.y = y

        if curve == "P256":
            self.curve = Curve()

    def __mul__(self, scalar):
        coords = self.curve.pointMul(self, scalar)
        return Point(coords[0], coords[1], self.curve)

    def __rmul__(self, scalar):
        return self * other
