from fastecdsa import curvemath
import Point


class Curve:
    def pointMul(self, p, d):
        args = map(str, (p.x, p.y, d))
        return map(int, curvemath.mul(*args))
