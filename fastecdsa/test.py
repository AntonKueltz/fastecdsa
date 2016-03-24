import unittest

from . import curvemath
from . import Point

class ECDSA(unittest.TestCase):
    def test_sign(self):
        d = 0x70a12c2db16845ed56ff68cfc21a472b3f04d7d6851bf6349f2d7d5b3452b38
        Q = Point.Point(
            0x8101ece47464a6ead70cf69a6e2bd3d88691a3262d22cba4f7635eaff26680a8,
            0xd8a12ba61d599235f67d9cb4d58f1783d3ca43e78f0a5abaa624079936c0c3a9,
            'P256'
        )
