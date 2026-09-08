from sys import argv, exit

from requests import get

try:
    json_uri = argv[1]
except IndexError:
    exit("Missing argument for JSON URI")

tests = get(json_uri).json()

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
