from unittest import TestCase

from parameterized import parameterized

from fastecdsa.eddsa import gen_ed25519_keypair, sign_ed25519, verify_ed25519
from fastecdsa.rust import Ed25519Point


class TestEd25519(TestCase):
    @parameterized.expand([
        (
            bytes.fromhex("9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"),
            b"",
            bytes.fromhex("e5564300c360ac729086e2cc806e828a84877f1eb8e5d974d873e065224901555fb8821590a33bacc61e39701cf9b46bd25bf5f0595bbe24655141438e7a100b"),
        ),
        (
            bytes.fromhex("4ccd089b28ff96da9db6c346ec114e0f5b8a319f35aba624da8cf6ed4fb8a6fb"),
            b"\x72",
            bytes.fromhex("92a009a9f0d4cab8720e820b5f642540a2b27b5416503f8fb3762223ebdb69da085ac1e43e15996e458f3613d0f11d8c387b2eaeb4302aeeb00d291612bb0c00"),
        ),
        (
            bytes.fromhex("c5aa8df43f9f837bedb7442f31dcb7b166d38535076f094b85ce3a2e0b4458f7"),
            b"\xaf\x82",
            bytes.fromhex("6291d657deec24024827e69c3abe01a30ce548a284743a445e3680d7db5ac3ac18ff9b538d16f290ae67f760984dc6594a7c15e9716ed28dc027beceea1ec40a"),
        ),
    ])
    def test_rfc8032_sign(self, sk: bytes, msg: bytes, expected: bytes):
       actual = sign_ed25519(sk, msg)
       self.assertEqual(actual, expected)

    @parameterized.expand([
        (
            bytes.fromhex("d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a"),
            b"",
            bytes.fromhex("e5564300c360ac729086e2cc806e828a84877f1eb8e5d974d873e065224901555fb8821590a33bacc61e39701cf9b46bd25bf5f0595bbe24655141438e7a100b"),
        ),
        (
            bytes.fromhex("3d4017c3e843895a92b70aa74d1b7ebc9c982ccf2ec4968cc0cd55f12af4660c"),
            b"\x72",
            bytes.fromhex("92a009a9f0d4cab8720e820b5f642540a2b27b5416503f8fb3762223ebdb69da085ac1e43e15996e458f3613d0f11d8c387b2eaeb4302aeeb00d291612bb0c00"),
        ),
        (
            bytes.fromhex("fc51cd8e6218a1a38da47ed00230f0580816ed13ba3303ac5deb911548908025"),
            b"\xaf\x82",
            bytes.fromhex("6291d657deec24024827e69c3abe01a30ce548a284743a445e3680d7db5ac3ac18ff9b538d16f290ae67f760984dc6594a7c15e9716ed28dc027beceea1ec40a"),
        ),
    ])
    def test_rfc8032_verify(self, pk: bytes, msg: bytes, sig: bytes):
        self.assertTrue(verify_ed25519(sig, msg, pk))

    def test_wrong_bit_x0(self):
        encoded = b"\xec\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff"

        with self.assertRaises(ValueError):
            Ed25519Point.decode(encoded)

    def test_keygen(self):
        sk, pk = gen_ed25519_keypair()
        sig = sign_ed25519(sk, b"")
        self.assertTrue(verify_ed25519(sig, b"", pk))
