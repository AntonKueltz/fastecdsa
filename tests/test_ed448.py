from unittest import TestCase

from parameterized import parameterized

from fastecdsa.eddsa import gen_ed448_keypair, sign_ed448, verify_ed448


class TestEd448(TestCase):
    @parameterized.expand([
        (
            bytes.fromhex("6c82a562cb808d10d632be89c8513ebf6c929f34ddfa8c9f63c9960ef6e348a3528c8a3fcc2f044e39a3fc5b94492f8f032e7549a20098f95b"),
            b"",
            b"",
            bytes.fromhex("533a37f6bbe457251f023c0d88f976ae2dfb504a843e34d2074fd823d41a591f2b233f034f628281f2fd7a22ddd47d7828c59bd0a21bfd3980ff0d2028d4b18a9df63e006c5d1c2d345b925d8dc00b4104852db99ac5c7cdda8530a113a0f4dbb61149f05a7363268c71d95808ff2e652600"),
        ),
        (
            bytes.fromhex("c4eab05d357007c632f3dbb48489924d552b08fe0c353a0d4a1f00acda2c463afbea67c5e8d2877c5e3bc397a659949ef8021e954e0a12274e"),
            b"\x03",
            b"",
            bytes.fromhex("26b8f91727bd62897af15e41eb43c377efb9c610d48f2335cb0bd0087810f4352541b143c4b981b7e18f62de8ccdf633fc1bf037ab7cd779805e0dbcc0aae1cbcee1afb2e027df36bc04dcecbf154336c19f0af7e0a6472905e799f1953d2a0ff3348ab21aa4adafd1d234441cf807c03a00"),
        ),
        (
            bytes.fromhex("c4eab05d357007c632f3dbb48489924d552b08fe0c353a0d4a1f00acda2c463afbea67c5e8d2877c5e3bc397a659949ef8021e954e0a12274e"),
            b"\x03",
            b"\x66\x6f\x6f",
            bytes.fromhex("d4f8f6131770dd46f40867d6fd5d5055de43541f8c5e35abbcd001b32a89f7d2151f7647f11d8ca2ae279fb842d607217fce6e042f6815ea000c85741de5c8da1144a6a1aba7f96de42505d7a7298524fda538fccbbb754f578c1cad10d54d0d5428407e85dcbc98a49155c13764e66c3c00"),
        ),
    ])
    def test_rfc8032_sign(self, sk: bytes, msg: bytes, ctx: bytes, expected: bytes):
       actual = sign_ed448(sk, msg, ctx)
       self.assertEqual(actual, expected)

    @parameterized.expand([
        (
            bytes.fromhex("5fd7449b59b461fd2ce787ec616ad46a1da1342485a70e1f8a0ea75d80e96778edf124769b46c7061bd6783df1e50f6cd1fa1abeafe8256180"),
            b"",
            bytes.fromhex("533a37f6bbe457251f023c0d88f976ae2dfb504a843e34d2074fd823d41a591f2b233f034f628281f2fd7a22ddd47d7828c59bd0a21bfd3980ff0d2028d4b18a9df63e006c5d1c2d345b925d8dc00b4104852db99ac5c7cdda8530a113a0f4dbb61149f05a7363268c71d95808ff2e652600"),
            b"",
        ),
    ])
    def test_rfc8032_verify(self, pk: bytes, msg: bytes, sig: bytes, ctx: bytes):
        self.assertTrue(verify_ed448(sig, msg, pk, ctx))

    def test_keygen(self):
        sk, pk = gen_ed448_keypair()
        sig = sign_ed448(sk, b"")
        self.assertTrue(verify_ed448(sig, b"", pk))
