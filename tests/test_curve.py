from unittest import TestCase

from fastecdsa.curve import Curve


class TestCurve(TestCase):
    def test_repr(self):
        expected = "Test Curve"

        curve = Curve(expected, 41, 1, 1, 31, 5, 7)
        actual = str(curve)

        self.assertEqual(expected, actual)
