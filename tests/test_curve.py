from unittest import TestCase

from fastecdsa.curve import Curve


class TestCurve(TestCase):
    def test_repr(self):
        expected = "Test Curve"

        curve = Curve(expected, 1, 1, 1, 1, 1, 1)
        actual = str(curve)

        self.assertEqual(expected, actual)
