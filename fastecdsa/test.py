import unittest

from curve import P256
from ecdsa import KeyPair


class TestCurve(unittest.TestCase):
    def test_P256_arith(self):
        S = (
            0xde2444bebc8d36e682edd27e0f271508617519b3221a8fa0b77cab3989da97c9,
            0xc093ae7ff36e5380fc01a5aad1e66659702de80f53cec576b6350b243042a256
        )
        d = 0xc51e4753afdec1e6b6c6a5b992f43f8dd0c7a8933072708b6522468b2ffb06fd
        expected = (
            0x51d08d5f2d4278882946d88d83c97d11e62becc3cfc18bedacc89ba34eeca03f,
            0x75ee68eb8bf626aa5b673ab51f6e744e06f8fcf8a6c0cf3035beca956a7b41d5
        )
        R = P256.pointMul(S, d)
        self.assertTrue(R[0] == expected[0])
        self.assertTrue(R[1] == expected[1])


class TestECDSA(unittest.TestCase):
    def test_ecdsa_P256_sign(self):
        keys = KeyPair(P256)
        keys.d = 0x70a12c2db16845ed56ff68cfc21a472b3f04d7d6851bf6349f2d7d5b3452b38a
        msg = 'This is only a test message. It is 48 bytes long'
        expected = (
            0x7214bc9647160bbd39ff2f80533f5dc6ddd70ddf86bb815661e805d5d4e6f27c,
            0x7d1ff961980f961bdaa3233b6209f4013317d3e3f9e1493592dbeaa1af2bc367
        )
        got = keys.sign(msg)
        self.assertTrue(got == expected)

    def test_ecdsa_P256_verify(self):
        keys = KeyPair(P256)
        keys.Q = (
            0x8101ece47464a6ead70cf69a6e2bd3d88691a3262d22cba4f7635eaff26680a8,
            0xd8a12ba61d599235f67d9cb4d58f1783d3ca43e78f0a5abaa624079936c0c3a9
        )
        msg = 'This is only a test message. It is 48 bytes long'
        sig = (
            0x7214bc9647160bbd39ff2f80533f5dc6ddd70ddf86bb815661e805d5d4e6f27c,
            0x7d1ff961980f961bdaa3233b6209f4013317d3e3f9e1493592dbeaa1af2bc367
        )
        self.assertTrue(keys.verify(sig, msg))

        sig = (
            0x7214bc9647160bbd39ff2f80533f5dc6ddd70ddf86bb815661e805d5d4e6fbad,
            0x7d1ff961980f961bdaa3233b6209f4013317d3e3f9e1493592dbeaa1af2bc367
        )
        self.assertFalse(keys.verify(sig, msg))

if __name__ == '__main__':
    unittest.main()
