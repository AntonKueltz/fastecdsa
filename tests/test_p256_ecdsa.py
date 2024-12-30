from hashlib import sha1, sha224, sha256, sha384, sha512
from unittest import TestCase

from fastecdsa.curve import P256
from fastecdsa.ecdsa import EcdsaError, sign, verify
from fastecdsa.point import Point


class TestP256ECDSA(TestCase):
    """case taken from http://tools.ietf.org/html/rfc6979#appendix-A.2.5"""

    def test_ecdsa_P256_SHA1_sign(self):
        d = 0xC9AFA9D845BA75166B5C215767B1D6934E50C3DB36E89B127B8A622B120F6721
        expected = (
            0x61340C88C3AAEBEB4F6D667F672CA9759A6CCAA9FA8811313039EE4A35471D32,
            0x6D7F147DAC089441BB2E2FE8F7A3FA264B9C475098FDCF6E00D7C996E1B8B7EB,
        )
        sig = sign("sample", d, curve=P256, hashfunc=sha1)
        self.assertEqual(sig, expected)

        Q = d * P256.G
        self.assertTrue(verify(sig, "sample", Q, curve=P256, hashfunc=sha1))

    """ case taken from http://tools.ietf.org/html/rfc6979#appendix-A.2.5 """

    def test_ecdsa_P256_SHA224_sign(self):
        d = 0xC9AFA9D845BA75166B5C215767B1D6934E50C3DB36E89B127B8A622B120F6721
        expected = (
            0x53B2FFF5D1752B2C689DF257C04C40A587FABABB3F6FC2702F1343AF7CA9AA3F,
            0xB9AFB64FDC03DC1A131C7D2386D11E349F070AA432A4ACC918BEA988BF75C74C,
        )
        sig = sign("sample", d, curve=P256, hashfunc=sha224)
        self.assertEqual(sig, expected)

        Q = d * P256.G
        self.assertTrue(verify(sig, "sample", Q, curve=P256, hashfunc=sha224))

    """ case taken from http://tools.ietf.org/html/rfc6979#appendix-A.2.5 """

    def test_ecdsa_P256_SHA2_sign(self):
        d = 0xC9AFA9D845BA75166B5C215767B1D6934E50C3DB36E89B127B8A622B120F6721
        expected = (
            0xEFD48B2AACB6A8FD1140DD9CD45E81D69D2C877B56AAF991C34D0EA84EAF3716,
            0xF7CB1C942D657C41D436C7A1B6E29F65F3E900DBB9AFF4064DC4AB2F843ACDA8,
        )
        sig = sign("sample", d, curve=P256, hashfunc=sha256)
        self.assertEqual(sig, expected)

        Q = d * P256.G
        self.assertTrue(verify(sig, "sample", Q, curve=P256, hashfunc=sha256))

    """ case taken from http://tools.ietf.org/html/rfc6979#appendix-A.2.5 """

    def test_ecdsa_P256_SHA384_sign(self):
        d = 0xC9AFA9D845BA75166B5C215767B1D6934E50C3DB36E89B127B8A622B120F6721
        expected = (
            0x0EAFEA039B20E9B42309FB1D89E213057CBF973DC0CFC8F129EDDDC800EF7719,
            0x4861F0491E6998B9455193E34E7B0D284DDD7149A74B95B9261F13ABDE940954,
        )
        sig = sign("sample", d, curve=P256, hashfunc=sha384)
        self.assertEqual(sig, expected)

        Q = d * P256.G
        self.assertTrue(verify(sig, "sample", Q, curve=P256, hashfunc=sha384))

    """ case taken from http://tools.ietf.org/html/rfc6979#appendix-A.2.5 """

    def test_ecdsa_P256_SHA512_sign(self):
        d = 0xC9AFA9D845BA75166B5C215767B1D6934E50C3DB36E89B127B8A622B120F6721
        expected = (
            0x8496A60B5E9B47C825488827E0495B0E3FA109EC4568FD3F8D1097678EB97F00,
            0x2362AB1ADBE2B8ADF9CB9EDAB740EA6049C028114F2460F96554F61FAE3302FE,
        )
        sig = sign("sample", d, curve=P256, hashfunc=sha512)
        self.assertEqual(sig, expected)

        Q = d * P256.G
        self.assertTrue(verify(sig, "sample", Q, curve=P256, hashfunc=sha512))

    """ case taken from https://www.nsa.gov/ia/_files/ecdsa.pdf """

    def test_ecdsa_P256_verify(self):
        Q = Point(
            0x8101ECE47464A6EAD70CF69A6E2BD3D88691A3262D22CBA4F7635EAFF26680A8,
            0xD8A12BA61D599235F67D9CB4D58F1783D3CA43E78F0A5ABAA624079936C0C3A9,
            curve=P256,
        )
        msg = "This is only a test message. It is 48 bytes long"
        sig = (
            0x7214BC9647160BBD39FF2F80533F5DC6DDD70DDF86BB815661E805D5D4E6F27C,
            0x7D1FF961980F961BDAA3233B6209F4013317D3E3F9E1493592DBEAA1AF2BC367,
        )
        self.assertTrue(verify(sig, msg, Q, curve=P256, hashfunc=sha256))

        sig = (
            0x7214BC9647160BBD39FF2F80533F5DC6DDD70DDF86BB815661E805D5D4E6FBAD,
            0x7D1FF961980F961BDAA3233B6209F4013317D3E3F9E1493592DBEAA1AF2BC367,
        )
        self.assertFalse(verify(sig, msg, Q, curve=P256, hashfunc=sha256))

    def test_ecdsa_P256_invalid_Q(self):
        Q = P256.G
        Q.x = 0

        with self.assertRaises(EcdsaError):
            verify((1, 1), "", Q)

    def test_ecdsa_P256_invalid_prehashed_msg_type(self):
        with self.assertRaises(TypeError):
            sign("this is a str type", 0x10001, prehashed=True)
