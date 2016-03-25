import unittest

from ecdsa import sign, verify
from point import Point


class ECDSA(unittest.TestCase):
    # https://www.nsa.gov/ia/_files/ecdsa.pdf
    def test_ecdsa_P256_sign(self):
        d = 0x70a12c2db16845ed56ff68cfc21a472b3f04d7d6851bf6349f2d7d5b3452b38a
        msg = 'This is only a test message. It is 48 bytes long'
        expected = (
            0x7214bc9647160bbd39ff2f80533f5dc6ddd70ddf86bb815661e805d5d4e6f27c,
            0x7d1ff961980f961bdaa3233b6209f4013317d3e3f9e1493592dbeaa1af2bc367
        )
        got = sign(msg, d)
        self.assertTrue(got == expected)

    def test_ecdsa_P256_verify(self):
        qx = 0x8101ece47464a6ead70cf69a6e2bd3d88691a3262d22cba4f7635eaff26680a8
        qy = 0xd8a12ba61d599235f67d9cb4d58f1783d3ca43e78f0a5abaa624079936c0c3a9
        Q = Point(qx, qy, 'P256')
        msg = 'This is only a test message. It is 48 bytes long'
        sig = (
            0x7214bc9647160bbd39ff2f80533f5dc6ddd70ddf86bb815661e805d5d4e6f27c,
            0x7d1ff961980f961bdaa3233b6209f4013317d3e3f9e1493592dbeaa1af2bc367
        )
        self.assertTrue(verify(sig, msg, Q))

        sig = (
            0x7214bc9647160bbd39ff2f80533f5dc6ddd70ddf86bb815661e805d5d4e6fbad,
            0x7d1ff961980f961bdaa3233b6209f4013317d3e3f9e1493592dbeaa1af2bc367
        )
        self.assertFalse(verify(sig, msg, Q))

if __name__ == '__main__':
    unittest.main()
