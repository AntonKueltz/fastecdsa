from sys import argv, exit

from requests import get

try:
    algo = argv[1]
except IndexError:
    exit("Missing arguments [signing algorithm]")

json_uri = f"https://raw.githubusercontent.com/C2SP/wycheproof/refs/heads/main/testvectors_v1/{algo}_test.json"
tests = get(json_uri).json()

print(f"""from unittest import TestCase

from parameterized import parameterized

from fastecdsa.eddsa import verify_{algo}


class TestWhycheproof{algo.capitalize()}(TestCase):
    @parameterized.expand([""")

for group in tests["testGroups"]:
    pk = bytes.fromhex(group["publicKey"]["pk"])

    for test in group["tests"]:
        msg = bytes.fromhex(test["msg"])
        sig = bytes.fromhex(test["sig"])
        expected = test["result"] == "valid"
        print(
            f"        (\n"
            f"            {pk},\n"
            f"            {msg},\n"
            f"            {sig},\n"
            f"            {expected}\n"
            f"        ),"
        )

print(f"""    ])
    def test_{algo}(
        self, pk: bytes, msg: bytes, sig: bytes, expected: bool
    ):

        result = verify_{algo}(sig, msg, pk)
        self.assertEqual(result, expected)
""")
