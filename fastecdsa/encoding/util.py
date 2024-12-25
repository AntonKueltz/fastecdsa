from typing import Optional


def int_bytelen(x: int) -> int:
    length = 0

    while x:
        length += 1
        x >>= 8

    return length


def int_to_bytes(x: int, length: Optional[int] = None) -> bytes:
    if length is None:
        length = int_bytelen(x)

    return int.to_bytes(x, length, "big")


def bytes_to_int(bytestr: bytes) -> int:
    return int.from_bytes(bytestr, "big")
