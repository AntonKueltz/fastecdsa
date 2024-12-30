from unittest import TestCase

from fastecdsa.curve import Curve


class TestCurve(TestCase):
    def test_repr(self):
        expected = "Test Curve"

        curve = Curve(expected, 0, 0, 0, 0, 0, 0)
        actual = str(curve)

        self.assertEqual(expected, actual)
