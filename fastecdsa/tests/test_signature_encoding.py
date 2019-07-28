from unittest import TestCase

from ..encoding.der import DEREncoder, InvalidDerSignature


class TestSignatureEncoding(TestCase):
    def test_encode_der_signature(self):
        self.assertEqual(DEREncoder.encode_signature(r=1, s=2), b"\x30"  # SEQUENCE
                                                                b"\x06"  # Length of Sequence
                                                                b"\x02"  # INTEGER
                                                                b"\x01"  # Length of r
                                                                b"\x01"  # r
                                                                b"\x02"  # INTEGER
                                                                b"\x01"  # Length of s
                                                                b"\x02")  # s

        # Check that we add a zero byte when the number's highest bit is set
        self.assertEqual(DEREncoder.encode_signature(r=128, s=128),
                         b"0\x08\x02\x02\x00\x80\x02\x02\x00\x80")

    def test_decode_der_signature(self):
        with self.assertRaises(InvalidDerSignature):
            DEREncoder.decode_signature(b"")  # length to shot
        with self.assertRaises(InvalidDerSignature):
            DEREncoder.decode_signature(b"\x31\x06\x02\x01\x01\x02\x01\x02")  # invalid SEQUENCE marker
        with self.assertRaises(InvalidDerSignature):
            DEREncoder.decode_signature(b"\x30\x07\x02\x01\x01\x02\x01\x02")  # invalid length
        with self.assertRaises(InvalidDerSignature):
            DEREncoder.decode_signature(b"\x30\x06\x02\x03\x01\x02\x01\x02")  # invalid length of r
        with self.assertRaises(InvalidDerSignature):
            DEREncoder.decode_signature(b"\x30\x06\x02\x01\x01\x03\x01\x02")  # invalid length of s
        with self.assertRaises(InvalidDerSignature):
            DEREncoder.decode_signature(b"\x30\x06\x03\x01\x01\x02\x01\x02")  # invalid INTEGER marker for r
        with self.assertRaises(InvalidDerSignature):
            DEREncoder.decode_signature(b"\x30\x06\x02\x00\x02\x01\x02")  # length of r is 0
        with self.assertRaises(InvalidDerSignature):
            DEREncoder.decode_signature(b"\x30\x06\x02\x01\x81\x02\x01\x02")  # value of r is negative
        with self.assertRaises(InvalidDerSignature):
            DEREncoder.decode_signature(b"\x30\x07\x02\x02\x00\x01\x02\x01\x02")  # value of r starts with a zero byte
        with self.assertRaises(InvalidDerSignature):
            DEREncoder.decode_signature(b"\x30\x06\x02\x01\x01\x03\x01\x02")  # invalid INTEGER marker for s
        with self.assertRaises(InvalidDerSignature):
            DEREncoder.decode_signature(b"\x30\x06\x02\x01\x01\x02\x00")  # value of s is 0
        with self.assertRaises(InvalidDerSignature):
            DEREncoder.decode_signature(b"\x30\x06\x02\x01\x01\x02\x01\x81")  # value of s is negative
        with self.assertRaises(InvalidDerSignature):
            DEREncoder.decode_signature(b"\x30\x07\x02\x01\x01\x02\x02\x00\x02")  # value of s starts with a zero byte

        self.assertEqual(DEREncoder.decode_signature(b"\x30\x06\x02\x01\x01\x02\x01\x02"), (1, 2))
        self.assertEqual(DEREncoder.decode_signature(b"0\x08\x02\x02\x00\x80\x02\x02\x00\x80"),
                         (128, 128))  # verify zero bytes
        self.assertEqual(DEREncoder.decode_signature(b"0\x08\x02\x02\x03\xE8\x02\x02\x03\xE8"),
                         (1000, 1000))  # verify byte order