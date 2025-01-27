from unittest import TestCase

from fastecdsa.curve import Curve
from fastecdsa.keys import gen_private_key


class TestKeygen(TestCase):
    def test_gen_private_key(self) -> None:
        class FakeCurve(Curve):
            def __init__(self, q: int) -> None:
                super().__init__("FakeCurve", 0, 0, 0, q, 0, 0)

        class FakeRandom:
            def __init__(self, values: bytes = b"\x01") -> None:
                self.values = values
                self.pos = 0

            def __call__(self, nb: int) -> bytes:
                result = self.values[self.pos : self.pos + nb]
                self.pos += nb
                return result

        # 1 byte / 6 bits shaved off + the first try is lower than the order
        self.assertEqual(gen_private_key(FakeCurve(2), randfunc=FakeRandom(b"\x40")), 1)

        # 1 byte / 6 bits shaved off + the first try is higher than the order
        self.assertEqual(
            gen_private_key(FakeCurve(2), randfunc=FakeRandom(b"\xc0\x40")), 1
        )

        # 2 byte / 3 are shaved off, the first try is lower than the order.
        self.assertEqual(
            gen_private_key(FakeCurve(8191), randfunc=FakeRandom(b"\xff\xf0")), 8190
        )

        # 2 byte  / 3 are shaved off
        # first try : _bytes_to_int("\xff\xf8") >> 3 == 8191 (too high for order 8191)
        # second try : _bytes_to_int("\xff\xf0") >> 3 == 8190 (ok for order 8191)
        self.assertEqual(
            gen_private_key(FakeCurve(8191), randfunc=FakeRandom(b"\xff\xf8\xff\xf0")),
            8190,
        )

        # Same but with a different second try value
        self.assertEqual(
            gen_private_key(FakeCurve(8191), randfunc=FakeRandom(b"\xff\xf8\xff\xef")),
            8189,
        )
