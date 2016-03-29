import unittest

from curve import P192, P224, P256, P384, P521
from ecdsa import KeyPair


class TestCurve(unittest.TestCase):
    ''' cases taken from https://www.nsa.gov/ia/_files/nist-routines.pdf '''

    def test_P192_arith(self):
        S = (
            0xd458e7d127ae671b0c330266d246769353a012073e97acf8,
            0x325930500d851f336bddc050cf7fb11b5673a1645086df3b
        )
        d = 0xa78a236d60baec0c5dd41b33a542463a8255391af64c74ee
        expected = (
            0x1faee4205a4f669d2d0a8f25e3bcec9a62a6952965bf6d31,
            0x5ff2cdfa508a2581892367087c696f179e7a4d7e8260fb06
        )
        R = P192.pointMul(S, d)
        self.assertTrue(R[0] == expected[0])
        self.assertTrue(R[1] == expected[1])

    def test_P224_arith(self):
        S = (
            0x6eca814ba59a930843dc814edd6c97da95518df3c6fdf16e9a10bb5b,
            0xef4b497f0963bc8b6aec0ca0f259b89cd80994147e05dc6b64d7bf22
        )
        d = 0xa78ccc30eaca0fcc8e36b2dd6fbb03df06d37f52711e6363aaf1d73b
        expected = (
            0x96a7625e92a8d72bff1113abdb95777e736a14c6fdaacc392702bca4,
            0x0f8e5702942a3c5e13cd2fd5801915258b43dfadc70d15dbada3ed10
        )
        R = P224.pointMul(S, d)
        self.assertTrue(R[0] == expected[0])
        self.assertTrue(R[1] == expected[1])

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

    def test_P384_arith(self):
        S = (
            0xfba203b81bbd23f2b3be971cc23997e1ae4d89e69cb6f92385dda82768ada415ebab4167459da98e62b1332d1e73cb0e,
            0x5ffedbaefdeba603e7923e06cdb5d0c65b22301429293376d5c6944e3fa6259f162b4788de6987fd59aed5e4b5285e45
        )
        d = 0xa4ebcae5a665983493ab3e626085a24c104311a761b5a8fdac052ed1f111a5c44f76f45659d2d111a61b5fdd97583480
        expected = (
            0xe4f77e7ffeb7f0958910e3a680d677a477191df166160ff7ef6bb5261f791aa7b45e3e653d151b95dad3d93ca0290ef2,
            0xac7dee41d8c5f4a7d5836960a773cfc1376289d3373f8cf7417b0c6207ac32e913856612fc9ff2e357eb2ee05cf9667f
        )
        R = P384.pointMul(S, d)
        self.assertTrue(R[0] == expected[0])
        self.assertTrue(R[1] == expected[1])

    def test_P521_arith(self):
        S = (
            0x000001d5c693f66c08ed03ad0f031f937443458f601fd098d3d0227b4bf62873af50740b0bb84aa157fc847bcf8dc16a8b2b8bfd8e2d0a7d39af04b089930ef6dad5c1b4,
            0x00000144b7770963c63a39248865ff36b074151eac33549b224af5c8664c54012b818ed037b2b7c1a63ac89ebaa11e07db89fcee5b556e49764ee3fa66ea7ae61ac01823
        )
        d = 0x000001eb7f81785c9629f136a7e8f8c674957109735554111a2a866fa5a166699419bfa9936c78b62653964df0d6da940a695c7294d41b2d6600de6dfcf0edcfc89fdcb1
        expected = (
            0x00000091b15d09d0ca0353f8f96b93cdb13497b0a4bb582ae9ebefa35eee61bf7b7d041b8ec34c6c00c0c0671c4ae063318fb75be87af4fe859608c95f0ab4774f8c95bb,
            0x00000130f8f8b5e1abb4dd94f6baaf654a2d5810411e77b7423965e0c7fd79ec1ae563c207bd255ee9828eb7a03fed565240d2cc80ddd2cecbb2eb50f0951f75ad87977f
        )
        R = P521.pointMul(S, d)
        self.assertTrue(R[0] == expected[0])
        self.assertTrue(R[1] == expected[1])


class TestECDSA(unittest.TestCase):
    def test_ecdsa_P256_sign(self):
        keys = KeyPair(P256)
        msg = 'Some test message to be signed via ECDSA'
        sig = keys.sign(msg)
        self.assertTrue(keys.verify(sig, msg))

    ''' case taken from https://www.nsa.gov/ia/_files/ecdsa.pdf '''
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
