from dataclasses import dataclass
from hashlib import sha1, sha224, sha256, sha384, sha512
from json import load
from typing import List
from unittest import TestCase

from fastecdsa.curve import P192, P224, P256, P384, P521
from fastecdsa.ecdsa import sign
from fastecdsa.typing import HashFunction
from fastecdsa.util import RFC6979

hash_lookup = {"1": sha1, "224": sha224, "256": sha256, "384": sha384, "512": sha512}


@dataclass
class Vector:
    h: HashFunction
    m: str
    k: int
    r: int
    s: int

    def __init__(self, data: dict):
        self.h = hash_lookup[data["h"]]
        self.m = data["m"]
        self.k = data["k"]
        self.r = data["r"]
        self.s = data["s"]


@dataclass
class Data:
    q: int
    x: int
    vectors: List[Vector]

    def __init__(self, data: dict):
        self.q = data["q"]
        self.x = data["x"]
        self.vectors = [Vector(d) for d in data["test_case"]]


class TestRFC6979ECDSA(TestCase):
    def _run_test(self, filename, curve):
        with open(filename) as f:
            data = load(f)

        test_data = Data(data)

        for test in test_data.vectors:
            self.assertEqual(
                test.k, RFC6979(test.m, test_data.x, test_data.q, test.h).gen_nonce()
            )
            self.assertEqual(
                (test.r, test.s),
                sign(test.m, test_data.x, curve=curve, hashfunc=test.h),
            )

    def test_p192_rfc6979_ecdsa(self):
        self._run_test("tests/vectors/rfc6979/P192.json", P192)

    def test_p224_rfc6979_ecdsa(self):
        self._run_test("tests/vectors/rfc6979/P224.json", P224)

    def test_p256_rfc6979_ecdsa(self):
        self._run_test("tests/vectors/rfc6979/P256.json", P256)

    def test_p384_rfc6979_ecdsa(self):
        self._run_test("tests/vectors/rfc6979/P384.json", P384)

    def test_p521_rfc6979_ecdsa(self):
        self._run_test("tests/vectors/rfc6979/P521.json", P521)
