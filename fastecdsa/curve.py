from fastecdsa import curvemath
import Point


'''
generators = {
    'P192': Point.Point(
        602046282375688656758213480587526111916698976636884684818,
        174050332293622031404857552280219410364023488927386650641,
        'P192'
    )
    'P256': Point.Point(
        48439561293906451759052585252797914202762949526041747995844080717082404635286,
        36134250956749795798585127919587881956611106672985015071877198253568414405109,
        'P256'
    )
}
'''


class Curve:
    def __init__(self, name):
        supported = ['P192', 'P256']

        if name in supported:
            self.name = name
        else:
            raise ValueError('Curve {} is not supported'.format(name))

    def pointMul(self, p, d):
        return map(int, curvemath.mul(str(p.x), str(p.y), str(d), self.name))
