from sys import argv, exit

from requests import get

try:
    curve = argv[1]
    hashfunc = argv[2]
except IndexError:
    exit("Missing arguments [curve] [hash]")

json_uri = f"https://raw.githubusercontent.com/C2SP/wycheproof/refs/heads/main/testvectors_v1/ecdsa_{curve}_{hashfunc}_test.json"
tests = get(json_uri).json()

print(f"""from hashlib import {hashfunc}
from unittest import TestCase

from parameterized import parameterized

from fastecdsa.curve import {curve}
from fastecdsa.ecdsa import verify
from fastecdsa.encoding.der import DEREncoder
from fastecdsa.encoding.sec1 import SEC1Encoder


class TestWhycheproof{curve.capitalize()}{hashfunc.capitalize()}(TestCase):
    @parameterized.expand([""")

for group in tests["testGroups"]:
    pk = bytes.fromhex(group["publicKey"]["uncompressed"])

    for test in group["tests"]:
        tc = f"[{test['tcId']}] {test['comment']}".replace("'", "\\'")
        msg = bytes.fromhex(test["msg"])
        sig = bytes.fromhex(test["sig"])
        expected = test["result"] == "valid"
        print(
            f"        (\n"
            f"            '{tc}',\n"
            f"            {pk},\n"
            f"            {msg},\n"
            f"            {sig},\n"
            f"            {expected}\n"
            f"        ),"
        )

print(f"""    ])
    def test_{curve}_{hashfunc}(
        self, name: str, pk: bytes, msg: bytes, sig: bytes, expected: bool
    ):
        decoder = SEC1Encoder()
        public_key = decoder.decode_public_key(pk, {curve})

        try:
            signature = DEREncoder.decode_signature(sig)
            result = verify(signature, msg, public_key, {curve}, {hashfunc})
            self.assertEqual(result, expected, name)
        except Exception:
            self.assertFalse(expected, name)""")
