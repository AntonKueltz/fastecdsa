from unittest import TestCase

from ..curve import secp256k1
from ..encoding.pem import PEMEncoder
from ..point import Point


class TestWhitespaceParsing(TestCase):
    d = 0x3007DF8E8BED6C3592A10F9D0495173DCECBC99A40E8C88B47D7D590F8B608DD
    x = 0x2C071354794F38FF439EFF52F8B475B0DEF34A210AA3B88A63F2A7295D35F41F
    y = 0xB0F35D4F45D95657FA37D6A5431BE9441A95638FC710884F43FFF7938428056E
    Q = Point(x, y, curve=secp256k1)

    def test_leading_newline(self):
        keypem = (
            "\n-----BEGIN EC PRIVATE KEY-----\n"
            "MHQCAQEEIDAH346L7Ww1kqEPnQSVFz3Oy8maQOjIi0fX1ZD4tgjdoAcGBSuBBAAK\n"
            "oUQDQgAELAcTVHlPOP9Dnv9S+LR1sN7zSiEKo7iKY/KnKV019B+w811PRdlWV/o3\n"
            "1qVDG+lEGpVjj8cQiE9D//eThCgFbg==\n"
            "-----END EC PRIVATE KEY-----\n"
        )
        key, pubkey = PEMEncoder.decode_private_key(keypem)
        self.assertEqual(key, self.d)
        self.assertEqual(pubkey, self.Q)

    def test_leading_trailing_newlines(self):
        keypem = (
            "\n\n\n\n-----BEGIN EC PRIVATE KEY-----\n"
            "MHQCAQEEIDAH346L7Ww1kqEPnQSVFz3Oy8maQOjIi0fX1ZD4tgjdoAcGBSuBBAAK\n"
            "oUQDQgAELAcTVHlPOP9Dnv9S+LR1sN7zSiEKo7iKY/KnKV019B+w811PRdlWV/o3\n"
            "1qVDG+lEGpVjj8cQiE9D//eThCgFbg==\n"
            "-----END EC PRIVATE KEY-----\n\n\n\n"
        )
        key, pubkey = PEMEncoder.decode_private_key(keypem)
        self.assertEqual(key, self.d)
        self.assertEqual(pubkey, self.Q)
