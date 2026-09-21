from unittest import TestCase

from parameterized import parameterized

from fastecdsa.eddsa import sign_ed25519, verify_ed25519
from fastecdsa.keys import gen_ed25519_keypair
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

    @parameterized.expand([
        (
            bytes.fromhex("8c93255d71dcab10e8f379c26200f3c7bd5f09d9bc3068d3ef4edeb4853022b6"),
            bytes.fromhex("c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa"),
            bytes.fromhex("c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a0000000000000000000000000000000000000000000000000000000000000000"),
            True,
        ),
        (
            bytes.fromhex("9bd9f44f4dcc75bd531b56b2cd280b0bb38fc1cd6d1230e14861d861de092e79"),
            bytes.fromhex("c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa"),
            bytes.fromhex("f7badec5b8abeaf699583992219b7b223f1df3fbbea919844e3f7c554a43dd43a5bb704786be79fc476f91d3f3f89b03984d8068dcf1bb7dfc6637b45450ac04"),
            True,
        ),
        (
            bytes.fromhex("aebf3f2601a0c8c5d39cc7d8911642f740b78168218da8471772b35f9d35b9ab"),
            bytes.fromhex("f7badec5b8abeaf699583992219b7b223f1df3fbbea919844e3f7c554a43dd43"),
            bytes.fromhex("c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa8c4bd45aecaca5b24fb97bc10ac27ac8751a7dfe1baff8b953ec9f5833ca260e"),
            True,
        ),
        (
            bytes.fromhex("9bd9f44f4dcc75bd531b56b2cd280b0bb38fc1cd6d1230e14861d861de092e79"),
            bytes.fromhex("cdb267ce40c5cd45306fa5d2f29731459387dbf9eb933b7bd5aed9a765b88d4d"),
            bytes.fromhex("9046a64750444938de19f227bb80485e92b83fdb4b6506c160484c016cc1852f87909e14428a7a1d62e9f22f3d3ad7802db02eb2e688b6c52fcd6648a98bd009"),
             True,
       ),
        (
            bytes.fromhex("e47d62c63f830dc7a6851a0b1f33ae4bb2f507fb6cffec4011eaccd55b53f56c"),
            bytes.fromhex("cdb267ce40c5cd45306fa5d2f29731459387dbf9eb933b7bd5aed9a765b88d4d"),
            bytes.fromhex("160a1cb0dc9c0258cd0a7d23e94d8fa878bcb1925f2c64246b2dee1796bed5125ec6bc982a269b723e0668e540911a9a6a58921d6925e434ab10aa7940551a09"),
            True,
        ),
        (
            bytes.fromhex("e47d62c63f830dc7a6851a0b1f33ae4bb2f507fb6cffec4011eaccd55b53f56c"),
            bytes.fromhex("cdb267ce40c5cd45306fa5d2f29731459387dbf9eb933b7bd5aed9a765b88d4d"),
            bytes.fromhex("21122a84e0b5fca4052f5b1235c80a537878b38f3142356b2c2384ebad4668b7e40bc836dac0f71076f9abe3a53f9c03c1ceeeddb658d0030494ace586687405"),
            True,
        ),
        (
            bytes.fromhex("85e241a07d148b41e47d62c63f830dc7a6851a0b1f33ae4bb2f507fb6cffec40"),
            bytes.fromhex("442aad9f089ad9e14647b1ef9099a1ff4798d78589e66f28eca69c11f582a623"),
            bytes.fromhex("e96f66be976d82e60150baecff9906684aebb1ef181f67a7189ac78ea23b6c0e547f7690a0e2ddcd04d87dbc3490dc19b3b3052f7ff0538cb68afb369ba3a514"),
            False,
        ),
        (
            bytes.fromhex("85e241a07d148b41e47d62c63f830dc7a6851a0b1f33ae4bb2f507fb6cffec40"),
            bytes.fromhex("442aad9f089ad9e14647b1ef9099a1ff4798d78589e66f28eca69c11f582a623"),
            bytes.fromhex("8ce5b96c8f26d0ab6c47958c9e68b937104cd36e13c33566acd2fe8d38aa19427e71f98a473474f2f13f06f97c20d58cc3f54b8bd0d272f42b695dd7e89a8c22"),
            False,
        ),
        (
            bytes.fromhex("9bedc267423725d473888631ebf45988bad3db83851ee85c85e241a07d148b41"),
            bytes.fromhex("f7badec5b8abeaf699583992219b7b223f1df3fbbea919844e3f7c554a43dd43"),
            bytes.fromhex("ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff03be9678ac102edcd92b0210bb34d7428d12ffc5df5f37e359941266a4e35f0f"),
            False,
        ),
        (
            bytes.fromhex("9bedc267423725d473888631ebf45988bad3db83851ee85c85e241a07d148b41"),
            bytes.fromhex("f7badec5b8abeaf699583992219b7b223f1df3fbbea919844e3f7c554a43dd43"),
            bytes.fromhex("ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffca8c5b64cd208982aa38d4936621a4775aa233aa0505711d8fdcfdaa943d4908"),
            False,
        ),
        (
            bytes.fromhex("e96b7021eb39c1a163b6da4e3093dcd3f21387da4cc4572be588fafae23c155b"),
            bytes.fromhex("ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"),
            bytes.fromhex("a9d55260f765261eb9b84e106f665e00b867287a761990d7135963ee0a7d59dca5bb704786be79fc476f91d3f3f89b03984d8068dcf1bb7dfc6637b45450ac04"),
            False,
        ),
        (
            bytes.fromhex("39a591f5321bbe07fd5a23dc2f39d025d74526615746727ceefd6e82ae65c06f"),
            bytes.fromhex("ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"),
            bytes.fromhex("a9d55260f765261eb9b84e106f665e00b867287a761990d7135963ee0a7d59dca5bb704786be79fc476f91d3f3f89b03984d8068dcf1bb7dfc6637b45450ac04"),
            False,
        ),
    ])
    def test_ed25519_speccheck(self, msg: bytes, pk: bytes, sig: bytes, expected: bool):
        actual = verify_ed25519(sig, msg, pk)
        self.assertEqual(actual, expected)
