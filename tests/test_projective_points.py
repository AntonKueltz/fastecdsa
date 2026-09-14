from random import choice, randint
from unittest import TestCase

from fastecdsa.curve import CURVES
from fastecdsa.point import Point


class TestProjectivePoints(TestCase):
    def test_arithmetic_parity(self):
        for i in range(100):
            c = choice(CURVES)
            g = c.G
            g_ = Point(g.x, g.y, c, projective=True)

            d, e = randint(1, c.q-1), randint(1, c.q-1)

            expected = d*g + e*g
            actual = (d*g_ + e*g_).normalize()

            self.assertEqual(actual, expected, f"i={i}, curve={c}, d={d:x}, e={e:x}")
