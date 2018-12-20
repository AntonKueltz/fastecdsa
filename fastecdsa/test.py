from hashlib import sha1, sha224, sha256, sha384, sha512
from os import remove
from random import choice, randint
from re import findall, DOTALL
from six.moves.urllib.request import urlopen
import unittest

from .curve import (
    P192, P224, P256, P384, P521, secp192k1, secp224k1, secp256k1, brainpoolP160r1, brainpoolP192r1,
    brainpoolP224r1, brainpoolP256r1, brainpoolP320r1, brainpoolP384r1, brainpoolP512r1
)
from .ecdsa import sign, verify
from .keys import export_key, gen_keypair, get_public_keys_from_sig, import_key
from .point import Point
from .util import RFC6979
from .asn1 import der_encode_signature, InvalidDerSignature, der_decode_signature

CURVES = [
    P192, P224, P256, P384, P521, secp192k1, secp224k1, secp256k1, brainpoolP160r1, brainpoolP192r1,
    brainpoolP224r1, brainpoolP256r1, brainpoolP320r1, brainpoolP384r1, brainpoolP512r1
]


class TestPrimeFieldCurve(unittest.TestCase):
    """ cases taken from https://www.nsa.gov/ia/_files/nist-routines.pdf """

    def test_P192_arith(self):
        S = Point(
            0xd458e7d127ae671b0c330266d246769353a012073e97acf8,
            0x325930500d851f336bddc050cf7fb11b5673a1645086df3b,
            curve=P192
        )
        d = 0xa78a236d60baec0c5dd41b33a542463a8255391af64c74ee
        expected = Point(
            0x1faee4205a4f669d2d0a8f25e3bcec9a62a6952965bf6d31,
            0x5ff2cdfa508a2581892367087c696f179e7a4d7e8260fb06,
            curve=P192
        )
        R = d * S
        self.assertEqual(R, expected)

    def test_P224_arith(self):
        S = Point(
            0x6eca814ba59a930843dc814edd6c97da95518df3c6fdf16e9a10bb5b,
            0xef4b497f0963bc8b6aec0ca0f259b89cd80994147e05dc6b64d7bf22,
            curve=P224
        )
        d = 0xa78ccc30eaca0fcc8e36b2dd6fbb03df06d37f52711e6363aaf1d73b
        expected = Point(
            0x96a7625e92a8d72bff1113abdb95777e736a14c6fdaacc392702bca4,
            0x0f8e5702942a3c5e13cd2fd5801915258b43dfadc70d15dbada3ed10,
            curve=P224
        )
        R = d * S
        self.assertEqual(R, expected)

    def test_P256_arith(self):
        S = Point(
            0xde2444bebc8d36e682edd27e0f271508617519b3221a8fa0b77cab3989da97c9,
            0xc093ae7ff36e5380fc01a5aad1e66659702de80f53cec576b6350b243042a256,
            curve=P256
        )
        d = 0xc51e4753afdec1e6b6c6a5b992f43f8dd0c7a8933072708b6522468b2ffb06fd
        expected = Point(
            0x51d08d5f2d4278882946d88d83c97d11e62becc3cfc18bedacc89ba34eeca03f,
            0x75ee68eb8bf626aa5b673ab51f6e744e06f8fcf8a6c0cf3035beca956a7b41d5,
            curve=P256
        )
        R = d * S
        self.assertEqual(R, expected)

    def test_P384_arith(self):
        S = Point(
            int('fba203b81bbd23f2b3be971cc23997e1ae4d89e69cb6f92385dda82768ada415ebab4167459da98e6'
                '2b1332d1e73cb0e', 16),
            int('5ffedbaefdeba603e7923e06cdb5d0c65b22301429293376d5c6944e3fa6259f162b4788de6987fd5'
                '9aed5e4b5285e45', 16),
            curve=P384
        )
        d = int('a4ebcae5a665983493ab3e626085a24c104311a761b5a8fdac052ed1f111a5c44f76f45659d2d111a'
                '61b5fdd97583480', 16)
        expected = Point(
            int('e4f77e7ffeb7f0958910e3a680d677a477191df166160ff7ef6bb5261f791aa7b45e3e653d151b95d'
                'ad3d93ca0290ef2', 16),
            int('ac7dee41d8c5f4a7d5836960a773cfc1376289d3373f8cf7417b0c6207ac32e913856612fc9ff2e35'
                '7eb2ee05cf9667f', 16),
            curve=P384
        )
        R = d * S
        self.assertEqual(R, expected)

    def test_P521_arith(self):
        S = Point(
            int('000001d5c693f66c08ed03ad0f031f937443458f601fd098d3d0227b4bf62873af50740b0bb84aa15'
                '7fc847bcf8dc16a8b2b8bfd8e2d0a7d39af04b089930ef6dad5c1b4', 16),
            int('00000144b7770963c63a39248865ff36b074151eac33549b224af5c8664c54012b818ed037b2b7c1a'
                '63ac89ebaa11e07db89fcee5b556e49764ee3fa66ea7ae61ac01823', 16),
            curve=P521
        )
        d = int('000001eb7f81785c9629f136a7e8f8c674957109735554111a2a866fa5a166699419bfa9936c78b62'
                '653964df0d6da940a695c7294d41b2d6600de6dfcf0edcfc89fdcb1', 16)
        expected = Point(
            int('00000091b15d09d0ca0353f8f96b93cdb13497b0a4bb582ae9ebefa35eee61bf7b7d041b8ec34c6c0'
                '0c0c0671c4ae063318fb75be87af4fe859608c95f0ab4774f8c95bb', 16),
            int('00000130f8f8b5e1abb4dd94f6baaf654a2d5810411e77b7423965e0c7fd79ec1ae563c207bd255ee'
                '9828eb7a03fed565240d2cc80ddd2cecbb2eb50f0951f75ad87977f', 16),
            curve=P521
        )
        R = d * S
        self.assertEqual(R, expected)

    def test_secp256k1_arith(self):
        # http://crypto.stackexchange.com/a/787/17884
        m = 0xAA5E28D6A97A2479A65527F7290311A3624D4CC0FA1578598EE3C2613BF99522
        expected = Point(
            0x34F9460F0E4F08393D192B3C5133A6BA099AA0AD9FD54EBCCFACDFA239FF49C6,
            0x0B71EA9BD730FD8923F6D25A7A91E7DD7728A960686CB5A901BB419E0F2CA232,
            curve=secp256k1
        )
        R = m * secp256k1.G
        self.assertEqual(R, expected)

        m = 0x7E2B897B8CEBC6361663AD410835639826D590F393D90A9538881735256DFAE3
        expected = Point(
            0xD74BF844B0862475103D96A611CF2D898447E288D34B360BC885CB8CE7C00575,
            0x131C670D414C4546B88AC3FF664611B1C38CEB1C21D76369D7A7A0969D61D97D,
            curve=secp256k1
        )
        R = m * secp256k1.G
        self.assertEqual(R, expected)

        m = 0x6461E6DF0FE7DFD05329F41BF771B86578143D4DD1F7866FB4CA7E97C5FA945D
        expected = Point(
            0xE8AECC370AEDD953483719A116711963CE201AC3EB21D3F3257BB48668C6A72F,
            0xC25CAF2F0EBA1DDB2F0F3F47866299EF907867B7D27E95B3873BF98397B24EE1,
            curve=secp256k1
        )
        R = m * secp256k1.G
        self.assertEqual(R, expected)

        m = 0x376A3A2CDCD12581EFFF13EE4AD44C4044B8A0524C42422A7E1E181E4DEECCEC
        expected = Point(
            0x14890E61FCD4B0BD92E5B36C81372CA6FED471EF3AA60A3E415EE4FE987DABA1,
            0x297B858D9F752AB42D3BCA67EE0EB6DCD1C2B7B0DBE23397E66ADC272263F982,
            curve=secp256k1
        )
        R = m * secp256k1.G
        self.assertEqual(R, expected)

        m = 0x1B22644A7BE026548810C378D0B2994EEFA6D2B9881803CB02CEFF865287D1B9
        expected = Point(
            0xF73C65EAD01C5126F28F442D087689BFA08E12763E0CEC1D35B01751FD735ED3,
            0xF449A8376906482A84ED01479BD18882B919C140D638307F0C0934BA12590BDE,
            curve=secp256k1
        )
        R = m * secp256k1.G
        self.assertEqual(R, expected)

    def test_arbitrary_arithmetic(self):
        for _ in range(100):
            curve = choice(CURVES)
            a, b = randint(0, curve.q), randint(0, curve.q)
            c = (a + b) % curve.q
            P, Q = a * curve.G, b * curve.G
            R = c * curve.G
            pq_sum, qp_sum = P + Q, Q + P
            self.assertTrue(pq_sum == qp_sum)
            self.assertTrue(qp_sum == R)


class TestNonceGeneration(unittest.TestCase):
    def test_rfc_6979(self):
        msg = 'sample'
        x = 0x09A4D6792295A7F730FC3F2B49CBC0F62E862272F
        q = 0x4000000000000000000020108A2E0CC0D99F8A5EF

        expected = 0x09744429FA741D12DE2BE8316E35E84DB9E5DF1CD
        nonce = RFC6979(msg, x, q, sha1).gen_nonce()
        self.assertTrue(nonce == expected)

        expected = 0x323E7B28BFD64E6082F5B12110AA87BC0D6A6E159
        nonce = RFC6979(msg, x, q, sha224).gen_nonce()
        self.assertTrue(nonce == expected)

        expected = 0x23AF4074C90A02B3FE61D286D5C87F425E6BDD81B
        nonce = RFC6979(msg, x, q, sha256).gen_nonce()
        self.assertTrue(nonce == expected)

        expected = 0x2132ABE0ED518487D3E4FA7FD24F8BED1F29CCFCE
        nonce = RFC6979(msg, x, q, sha384).gen_nonce()
        self.assertTrue(nonce == expected)

        expected = 0x00BBCC2F39939388FDFE841892537EC7B1FF33AA3
        nonce = RFC6979(msg, x, q, sha512).gen_nonce()
        self.assertTrue(nonce == expected)


class TestPrimeFieldECDSA(unittest.TestCase):
    """ case taken from http://tools.ietf.org/html/rfc6979#appendix-A.2.5 """
    def test_ecdsa_P256_SHA1_sign(self):
        d = 0xC9AFA9D845BA75166B5C215767B1D6934E50C3DB36E89B127B8A622B120F6721
        expected = (
            0x61340C88C3AAEBEB4F6D667F672CA9759A6CCAA9FA8811313039EE4A35471D32,
            0x6D7F147DAC089441BB2E2FE8F7A3FA264B9C475098FDCF6E00D7C996E1B8B7EB,
        )
        sig = sign('sample', d, curve=P256, hashfunc=sha1)
        self.assertEqual(sig, expected)

        Q = d * P256.G
        self.assertTrue(verify(sig, 'sample', Q, curve=P256, hashfunc=sha1))

    """ case taken from http://tools.ietf.org/html/rfc6979#appendix-A.2.5 """
    def test_ecdsa_P256_SHA224_sign(self):
        d = 0xC9AFA9D845BA75166B5C215767B1D6934E50C3DB36E89B127B8A622B120F6721
        expected = (
            0x53B2FFF5D1752B2C689DF257C04C40A587FABABB3F6FC2702F1343AF7CA9AA3F,
            0xB9AFB64FDC03DC1A131C7D2386D11E349F070AA432A4ACC918BEA988BF75C74C
        )
        sig = sign('sample', d, curve=P256, hashfunc=sha224)
        self.assertEqual(sig, expected)

        Q = d * P256.G
        self.assertTrue(verify(sig, 'sample', Q, curve=P256, hashfunc=sha224))

    """ case taken from http://tools.ietf.org/html/rfc6979#appendix-A.2.5 """
    def test_ecdsa_P256_SHA2_sign(self):
        d = 0xC9AFA9D845BA75166B5C215767B1D6934E50C3DB36E89B127B8A622B120F6721
        expected = (
            0xEFD48B2AACB6A8FD1140DD9CD45E81D69D2C877B56AAF991C34D0EA84EAF3716,
            0xF7CB1C942D657C41D436C7A1B6E29F65F3E900DBB9AFF4064DC4AB2F843ACDA8
        )
        sig = sign('sample', d, curve=P256, hashfunc=sha256)
        self.assertEqual(sig, expected)

        Q = d * P256.G
        self.assertTrue(verify(sig, 'sample', Q, curve=P256, hashfunc=sha256))

    """ case taken from http://tools.ietf.org/html/rfc6979#appendix-A.2.5 """
    def test_ecdsa_P256_SHA384_sign(self):
        d = 0xC9AFA9D845BA75166B5C215767B1D6934E50C3DB36E89B127B8A622B120F6721
        expected = (
            0x0EAFEA039B20E9B42309FB1D89E213057CBF973DC0CFC8F129EDDDC800EF7719,
            0x4861F0491E6998B9455193E34E7B0D284DDD7149A74B95B9261F13ABDE940954
        )
        sig = sign('sample', d, curve=P256, hashfunc=sha384)
        self.assertEqual(sig, expected)

        Q = d * P256.G
        self.assertTrue(verify(sig, 'sample', Q, curve=P256, hashfunc=sha384))

    """ case taken from http://tools.ietf.org/html/rfc6979#appendix-A.2.5 """
    def test_ecdsa_P256_SHA512_sign(self):
        d = 0xC9AFA9D845BA75166B5C215767B1D6934E50C3DB36E89B127B8A622B120F6721
        expected = (
            0x8496A60B5E9B47C825488827E0495B0E3FA109EC4568FD3F8D1097678EB97F00,
            0x2362AB1ADBE2B8ADF9CB9EDAB740EA6049C028114F2460F96554F61FAE3302FE
        )
        sig = sign('sample', d, curve=P256, hashfunc=sha512)
        self.assertEqual(sig, expected)

        Q = d * P256.G
        self.assertTrue(verify(sig, 'sample', Q, curve=P256, hashfunc=sha512))

    """ case taken from https://www.nsa.gov/ia/_files/ecdsa.pdf """
    def test_ecdsa_P256_verify(self):
        Q = Point(
            0x8101ece47464a6ead70cf69a6e2bd3d88691a3262d22cba4f7635eaff26680a8,
            0xd8a12ba61d599235f67d9cb4d58f1783d3ca43e78f0a5abaa624079936c0c3a9,
            curve=P256
        )
        msg = 'This is only a test message. It is 48 bytes long'
        sig = (
            0x7214bc9647160bbd39ff2f80533f5dc6ddd70ddf86bb815661e805d5d4e6f27c,
            0x7d1ff961980f961bdaa3233b6209f4013317d3e3f9e1493592dbeaa1af2bc367
        )
        self.assertTrue(verify(sig, msg, Q, curve=P256, hashfunc=sha256))

        sig = (
            0x7214bc9647160bbd39ff2f80533f5dc6ddd70ddf86bb815661e805d5d4e6fbad,
            0x7d1ff961980f961bdaa3233b6209f4013317d3e3f9e1493592dbeaa1af2bc367
        )
        self.assertFalse(verify(sig, msg, Q, curve=P256, hashfunc=sha256))


class TestP192ECDSA(unittest.TestCase):
    def test_rfc6979(self):
        text = urlopen('https://tools.ietf.org/rfc/rfc6979.txt').read().decode()
        curve_tests = findall(r'curve: NIST P-192(.*)curve: NIST P-224', text, flags=DOTALL)[0]

        q = int(findall(r'q = ([0-9A-F]*)', curve_tests)[0], 16)
        x = int(findall(r'x = ([0-9A-F]*)', curve_tests)[0], 16)

        test_regex = r'With SHA-(\d+), message = "([a-zA-Z]*)":\n' \
                     r'\s*k = ([0-9A-F]*)\n' \
                     r'\s*r = ([0-9A-F]*)\n' \
                     r'\s*s = ([0-9A-F]*)\n'

        hash_lookup = {
            '1': sha1,
            '224': sha224,
            '256': sha256,
            '384': sha384,
            '512': sha512
        }

        for test in findall(test_regex, curve_tests):
            h = hash_lookup[test[0]]
            msg = test[1]
            k = int(test[2], 16)
            r = int(test[3], 16)
            s = int(test[4], 16)

            self.assertEqual(k, RFC6979(msg, x, q, h).gen_nonce())
            self.assertEqual((r, s), sign(msg, x, curve=P192, hashfunc=h))


class TestP224ECDSA(unittest.TestCase):
    def test_rfc6979(self):
        text = urlopen('https://tools.ietf.org/rfc/rfc6979.txt').read().decode()
        curve_tests = findall(r'curve: NIST P-224(.*)curve: NIST P-256', text, flags=DOTALL)[0]

        q = int(findall(r'q = ([0-9A-F]*)', curve_tests)[0], 16)
        x = int(findall(r'x = ([0-9A-F]*)', curve_tests)[0], 16)

        test_regex = r'With SHA-(\d+), message = "([a-zA-Z]*)":\n' \
                     r'\s*k = ([0-9A-F]*)\n' \
                     r'\s*r = ([0-9A-F]*)\n' \
                     r'\s*s = ([0-9A-F]*)\n'

        hash_lookup = {
            '1': sha1,
            '224': sha224,
            '256': sha256,
            '384': sha384,
            '512': sha512
        }

        for test in findall(test_regex, curve_tests):
            h = hash_lookup[test[0]]
            msg = test[1]
            k = int(test[2], 16)
            r = int(test[3], 16)
            s = int(test[4], 16)

            self.assertEqual(k, RFC6979(msg, x, q, h).gen_nonce())
            self.assertEqual((r, s), sign(msg, x, curve=P224, hashfunc=h))


class TestP256ECDSA(unittest.TestCase):
    def test_rfc6979(self):
        text = urlopen('https://tools.ietf.org/rfc/rfc6979.txt').read().decode()
        curve_tests = findall(r'curve: NIST P-256(.*)curve: NIST P-384', text, flags=DOTALL)[0]

        q = int(findall(r'q = ([0-9A-F]*)', curve_tests)[0], 16)
        x = int(findall(r'x = ([0-9A-F]*)', curve_tests)[0], 16)

        test_regex = r'With SHA-(\d+), message = "([a-zA-Z]*)":\n' \
                     r'\s*k = ([0-9A-F]*)\n' \
                     r'\s*r = ([0-9A-F]*)\n' \
                     r'\s*s = ([0-9A-F]*)\n'

        hash_lookup = {
            '1': sha1,
            '224': sha224,
            '256': sha256,
            '384': sha384,
            '512': sha512
        }

        for test in findall(test_regex, curve_tests):
            h = hash_lookup[test[0]]
            msg = test[1]
            k = int(test[2], 16)
            r = int(test[3], 16)
            s = int(test[4], 16)

            self.assertEqual(k, RFC6979(msg, x, q, h).gen_nonce())
            self.assertEqual((r, s), sign(msg, x, curve=P256, hashfunc=h))


class TestP384ECDSA(unittest.TestCase):
    def test_rfc6979(self):
        text = urlopen('https://tools.ietf.org/rfc/rfc6979.txt').read().decode()
        curve_tests = findall(r'curve: NIST P-384(.*)curve: NIST P-521', text, flags=DOTALL)[0]

        q_parts = findall(r'q = ([0-9A-F]*)\n\s*([0-9A-F]*)', curve_tests)[0]
        q = int(q_parts[0] + q_parts[1], 16)
        x_parts = findall(r'x = ([0-9A-F]*)\n\s*([0-9A-F]*)', curve_tests)[0]
        x = int(x_parts[0] + x_parts[1], 16)

        test_regex = r'With SHA-(\d+), message = "([a-zA-Z]*)":\n' \
                     r'\s*k = ([0-9A-F]*)\n\s*([0-9A-F]*)\n' \
                     r'\s*r = ([0-9A-F]*)\n\s*([0-9A-F]*)\n' \
                     r'\s*s = ([0-9A-F]*)\n\s*([0-9A-F]*)\n'

        hash_lookup = {
            '1': sha1,
            '224': sha224,
            '256': sha256,
            '384': sha384,
            '512': sha512
        }

        for test in findall(test_regex, curve_tests):
            h = hash_lookup[test[0]]
            msg = test[1]
            k = int(test[2] + test[3], 16)
            r = int(test[4] + test[5], 16)
            s = int(test[6] + test[7], 16)

            self.assertEqual(k, RFC6979(msg, x, q, h).gen_nonce())
            self.assertEqual((r, s), sign(msg, x, curve=P384, hashfunc=h))


class TestP521ECDSA(unittest.TestCase):
    def test_rfc6979(self):
        text = urlopen('https://tools.ietf.org/rfc/rfc6979.txt').read().decode()
        curve_tests = findall(r'curve: NIST P-521(.*)curve: NIST K-163', text, flags=DOTALL)[0]

        q_parts = findall(r'q = ([0-9A-F]*)\n\s*([0-9A-F]*)\n\s*([0-9A-F]*)', curve_tests)[0]
        q = int(q_parts[0] + q_parts[1] + q_parts[2], 16)
        x_parts = findall(r'x = ([0-9A-F]*)\n\s*([0-9A-F]*)\n\s*([0-9A-F]*)', curve_tests)[0]
        x = int(x_parts[0] + x_parts[1] + x_parts[2], 16)

        test_regex = r'With SHA-(\d+), message = "([a-zA-Z]*)":\n' \
                     r'\s*k = ([0-9A-F]*)\n\s*([0-9A-F]*)\n\s*([0-9A-F]*)\n' \
                     r'\s*r = ([0-9A-F]*)\n\s*([0-9A-F]*)\n\s*([0-9A-F]*)\n' \
                     r'\s*s = ([0-9A-F]*)\n\s*([0-9A-F]*)\n\s*([0-9A-F]*)\n'

        hash_lookup = {
            '1': sha1,
            '224': sha224,
            '256': sha256,
            '384': sha384,
            '512': sha512
        }

        for test in findall(test_regex, curve_tests):
            h = hash_lookup[test[0]]
            msg = test[1]
            k = int(test[2] + test[3] + test[4], 16)
            r = int(test[5] + test[6] + test[7], 16)
            s = int(test[8] + test[9] + test[10], 16)

            self.assertEqual(k, RFC6979(msg, x, q, h).gen_nonce())
            self.assertEqual((r, s), sign(msg, x, curve=P521, hashfunc=h))


class TestBrainpoolECDH(unittest.TestCase):
    def test_256bit(self):
        # https://tools.ietf.org/html/rfc7027#appendix-A.1
        dA = 0x81DB1EE100150FF2EA338D708271BE38300CB54241D79950F77B063039804F1D
        qA = dA * brainpoolP256r1.G

        self.assertEqual(qA.x, 0x44106E913F92BC02A1705D9953A8414DB95E1AAA49E81D9E85F929A8E3100BE5)
        self.assertEqual(qA.y, 0x8AB4846F11CACCB73CE49CBDD120F5A900A69FD32C272223F789EF10EB089BDC)

        dB = 0x55E40BC41E37E3E2AD25C3C6654511FFA8474A91A0032087593852D3E7D76BD3
        qB = dB * brainpoolP256r1.G

        self.assertEqual(qB.x, 0x8D2D688C6CF93E1160AD04CC4429117DC2C41825E1E9FCA0ADDD34E6F1B39F7B)
        self.assertEqual(qB.y, 0x990C57520812BE512641E47034832106BC7D3E8DD0E4C7F1136D7006547CEC6A)

        self.assertEqual((dA * qB).x, (dB * qA).x)
        self.assertEqual((dA * qB).y, (dB * qA).y)

        Z = dA * qB
        self.assertEqual(Z.x, 0x89AFC39D41D3B327814B80940B042590F96556EC91E6AE7939BCE31F3A18BF2B)
        self.assertEqual(Z.y, 0x49C27868F4ECA2179BFD7D59B1E3BF34C1DBDE61AE12931648F43E59632504DE)

    def test_384bit(self):
        # https://tools.ietf.org/html/rfc7027#appendix-A.2
        dA = int('1E20F5E048A5886F1F157C74E91BDE2B98C8B52D58E5003D57053FC4B0BD6'
                 '5D6F15EB5D1EE1610DF870795143627D042', 16)
        qA = dA * brainpoolP384r1.G

        self.assertEqual(
            qA.x,
            int('68B665DD91C195800650CDD363C625F4E742E8134667B767B1B47679358'
                '8F885AB698C852D4A6E77A252D6380FCAF068', 16)
        )
        self.assertEqual(
            qA.y,
            int('55BC91A39C9EC01DEE36017B7D673A931236D2F1F5C83942D049E3FA206'
                '07493E0D038FF2FD30C2AB67D15C85F7FAA59', 16)
        )

        dB = int('032640BC6003C59260F7250C3DB58CE647F98E1260ACCE4ACDA3DD869F74E'
                 '01F8BA5E0324309DB6A9831497ABAC96670', 16)
        qB = dB * brainpoolP384r1.G

        self.assertEqual(
            qB.x,
            int('4D44326F269A597A5B58BBA565DA5556ED7FD9A8A9EB76C25F46DB69D19'
                'DC8CE6AD18E404B15738B2086DF37E71D1EB4', 16)
        )
        self.assertEqual(
            qB.y,
            int('62D692136DE56CBE93BF5FA3188EF58BC8A3A0EC6C1E151A21038A42E91'
                '85329B5B275903D192F8D4E1F32FE9CC78C48', 16)
        )

        self.assertEqual((dA * qB).x, (dB * qA).x)
        self.assertEqual((dA * qB).y, (dB * qA).y)

        Z = dA * qB
        self.assertEqual(
            Z.x,
            int('0BD9D3A7EA0B3D519D09D8E48D0785FB744A6B355E6304BC51C229FBBCE2'
                '39BBADF6403715C35D4FB2A5444F575D4F42', 16)
        )
        self.assertEqual(
            Z.y,
            int('0DF213417EBE4D8E40A5F76F66C56470C489A3478D146DECF6DF0D94BAE9'
                'E598157290F8756066975F1DB34B2324B7BD', 16)
        )

    def test_512bit(self):
        # https://tools.ietf.org/html/rfc7027#appendix-A.2
        dA = int('16302FF0DBBB5A8D733DAB7141C1B45ACBC8715939677F6A56850A38BD87B'
                 'D59B09E80279609FF333EB9D4C061231FB26F92EEB04982A5F1D1764CAD57665422', 16)
        qA = dA * brainpoolP512r1.G

        self.assertEqual(
            qA.x,
            int('0A420517E406AAC0ACDCE90FCD71487718D3B953EFD7FBEC5F7F27E28C6'
                '149999397E91E029E06457DB2D3E640668B392C2A7E737A7F0BF04436D11640FD09FD', 16)
        )
        self.assertEqual(
            qA.y,
            int('72E6882E8DB28AAD36237CD25D580DB23783961C8DC52DFA2EC138AD472'
                'A0FCEF3887CF62B623B2A87DE5C588301EA3E5FC269B373B60724F5E82A6AD147FDE7', 16)
        )

        dB = int('230E18E1BCC88A362FA54E4EA3902009292F7F8033624FD471B5D8ACE49D1'
                 '2CFABBC19963DAB8E2F1EBA00BFFB29E4D72D13F2224562F405CB80503666B25429', 16)
        qB = dB * brainpoolP512r1.G

        self.assertEqual(
            qB.x,
            int('9D45F66DE5D67E2E6DB6E93A59CE0BB48106097FF78A081DE781CDB31FC'
                'E8CCBAAEA8DD4320C4119F1E9CD437A2EAB3731FA9668AB268D871DEDA55A5473199F', 16)
        )
        self.assertEqual(
            qB.y,
            int('2FDC313095BCDD5FB3A91636F07A959C8E86B5636A1E930E8396049CB48'
                '1961D365CC11453A06C719835475B12CB52FC3C383BCE35E27EF194512B71876285FA', 16)
        )

        self.assertEqual((dA * qB).x, (dB * qA).x)
        self.assertEqual((dA * qB).y, (dB * qA).y)

        Z = dA * qB
        self.assertEqual(
            Z.x,
            int('A7927098655F1F9976FA50A9D566865DC530331846381C87256BAF322624'
                '4B76D36403C024D7BBF0AA0803EAFF405D3D24F11A9B5C0BEF679FE1454B21C4CD1F', 16)
        )
        self.assertEqual(
            Z.y,
            int('7DB71C3DEF63212841C463E881BDCF055523BD368240E6C3143BD8DEF8B3'
                'B3223B95E0F53082FF5E412F4222537A43DF1C6D25729DDB51620A832BE6A26680A2', 16)
        )


class TestAsn1(unittest.TestCase):
    def test_generate_and_parse_pem(self):
        d, Q = gen_keypair(P256)
        export_key(d, curve=P256, filepath='p256.key')
        export_key(Q, curve=P256, filepath='p256.pub')

        parsed_d, parsed_Q = import_key('p256.key')
        self.assertEqual(parsed_d, d)
        self.assertEqual(parsed_Q, Q)

        parsed_d, parsed_Q = import_key('p256.pub')
        self.assertTrue(parsed_d is None)
        self.assertEqual(parsed_Q, Q)

        remove('p256.key')
        remove('p256.pub')


class TestSignatureDERencoding(unittest.TestCase):
    def test_encode_der_signature(self):
        self.assertEqual(der_encode_signature(r=1, s=2), b"\x30"  # SEQUENCE
                                                         b"\x06"  # Length of Sequence
                                                         b"\x02"  # INTEGER
                                                         b"\x01"  # Length of r
                                                         b"\x01"  # r
                                                         b"\x02"  # INTEGER
                                                         b"\x01"  # Length of s
                                                         b"\x02")  # s

        # Check that we add a zero byte when the number's highest bit is set
        self.assertEqual(der_encode_signature(r=128, s=128),
                         b"0\x08\x02\x02\x00\x80\x02\x02\x00\x80")

    def test_decode_der_signature(self):
        with self.assertRaises(InvalidDerSignature):
            der_decode_signature(b"")  # length to shot
        with self.assertRaises(InvalidDerSignature):
            der_decode_signature(b"\x31\x06\x02\x01\x01\x02\x01\x02")  # invalid SEQUENCE marker
        with self.assertRaises(InvalidDerSignature):
            der_decode_signature(b"\x30\x07\x02\x01\x01\x02\x01\x02")  # invalid length
        with self.assertRaises(InvalidDerSignature):
            der_decode_signature(b"\x30\x06\x02\x03\x01\x02\x01\x02")  # invalid length of r
        with self.assertRaises(InvalidDerSignature):
            der_decode_signature(b"\x30\x06\x02\x01\x01\x03\x01\x02")  # invalid length of s
        with self.assertRaises(InvalidDerSignature):
            der_decode_signature(b"\x30\x06\x03\x01\x01\x02\x01\x02")  # invalid INTEGER marker for r
        with self.assertRaises(InvalidDerSignature):
            der_decode_signature(b"\x30\x06\x02\x00\x02\x01\x02")  # length of r is 0
        with self.assertRaises(InvalidDerSignature):
            der_decode_signature(b"\x30\x06\x02\x01\x81\x02\x01\x02")  # value of r is negative
        with self.assertRaises(InvalidDerSignature):
            der_decode_signature(b"\x30\x07\x02\x02\x00\x01\x02\x01\x02")  # value of r starts with a zero byte
        with self.assertRaises(InvalidDerSignature):
            der_decode_signature(b"\x30\x06\x02\x01\x01\x03\x01\x02")  # invalid INTEGER marker for s
        with self.assertRaises(InvalidDerSignature):
            der_decode_signature(b"\x30\x06\x02\x01\x01\x02\x00")  # value of s is 0
        with self.assertRaises(InvalidDerSignature):
            der_decode_signature(b"\x30\x06\x02\x01\x01\x02\x01\x81")  # value of s is negative
        with self.assertRaises(InvalidDerSignature):
            der_decode_signature(b"\x30\x07\x02\x01\x01\x02\x02\x00\x02")  # value of s starts with a zero byte

        self.assertEqual(der_decode_signature(b"\x30\x06\x02\x01\x01\x02\x01\x02"), (1, 2))
        self.assertEqual(der_decode_signature(b"0\x08\x02\x02\x00\x80\x02\x02\x00\x80"), (128, 128))  # verify zero bytes
        self.assertEqual(der_decode_signature(b"0\x08\x02\x02\x03\xE8\x02\x02\x03\xE8"), (1000, 1000))  # verify byte order


class TestKeyRecovery(unittest.TestCase):
    def test_key_recovery(self):
        for curve in CURVES:
            d, Q = gen_keypair(curve)
            msg = 'https://crypto.stackexchange.com/questions/18105/how-does-recovering-the-' \
                  'public-key-from-an-ecdsa-signature-work'
            sig = sign(msg, d, curve=curve)

            Qs = get_public_keys_from_sig(sig, msg, curve=curve, hashfunc=sha256)
            self.assertTrue(Q in Qs)


if __name__ == '__main__':

    unittest.main()
