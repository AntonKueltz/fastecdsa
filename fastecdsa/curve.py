from fastecdsa import curvemath


class Curve:
    def __init__(self, name):
        supported = ['P192', 'P256']

        if name in supported:
            self.name = name
        else:
            raise ValueError('Curve {} is not supported'.format(name))

    def pointMul(self, p, d):
        return map(int, curvemath.mul(str(p.x), str(p.y), str(d), self.name))
