from unittest import TestCase

from fastecdsa.curve import Curve
from fastecdsa.encoding.asn1 import asn1_oid


class TestAsn1(TestCase):
    def test_asn1_oid(self):
        expected = b""

        curve = Curve("", 41, 1, 1, 31, 5, 7)
        actual = asn1_oid(curve)

        self.assertEqual(expected, actual)
