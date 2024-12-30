from unittest import TestCase

from fastecdsa.encoding.pem import PEMEncoder


class TestWhitespaceParsing(TestCase):
    def setUp(self):
        self.encoder = PEMEncoder()

    def test_leading_newline(self):
        expected = 0x3007DF8E8BED6C3592A10F9D0495173DCECBC99A40E8C88B47D7D590F8B608DD
        keypem = (
            b"\n-----BEGIN EC PRIVATE KEY-----\n"
            b"MHQCAQEEIDAH346L7Ww1kqEPnQSVFz3Oy8maQOjIi0fX1ZD4tgjdoAcGBSuBBAAK\n"
            b"oUQDQgAELAcTVHlPOP9Dnv9S+LR1sN7zSiEKo7iKY/KnKV019B+w811PRdlWV/o3\n"
            b"1qVDG+lEGpVjj8cQiE9D//eThCgFbg==\n"
            b"-----END EC PRIVATE KEY-----\n"
        )

        actual = self.encoder.decode_private_key(keypem)

        self.assertEqual(expected, actual)

    def test_leading_trailing_newlines(self):
        expected = 0x3007DF8E8BED6C3592A10F9D0495173DCECBC99A40E8C88B47D7D590F8B608DD
        keypem = (
            b"\n\n\n\n-----BEGIN EC PRIVATE KEY-----\n"
            b"MHQCAQEEIDAH346L7Ww1kqEPnQSVFz3Oy8maQOjIi0fX1ZD4tgjdoAcGBSuBBAAK\n"
            b"oUQDQgAELAcTVHlPOP9Dnv9S+LR1sN7zSiEKo7iKY/KnKV019B+w811PRdlWV/o3\n"
            b"1qVDG+lEGpVjj8cQiE9D//eThCgFbg==\n"
            b"-----END EC PRIVATE KEY-----\n\n\n\n"
        )

        actual = self.encoder.decode_private_key(keypem)

        self.assertEqual(expected, actual)
