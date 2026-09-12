from _hashlib import HASH
from collections.abc import Callable
from hashlib import sha256

from .curve import Curve, P256
from .point import Point
from .util import RFC6979


class EcdsaError(Exception):
    def __init__(self, msg: str) -> None:
        self.msg = msg


def sign(
    msg: bytes,
    d: int,
    curve: Curve = P256,
    hashfunc: Callable[[], HASH] = sha256,
    prehashed: bool = False,
) -> tuple[int, int]:
    """Sign a message using the elliptic curve digital signature algorithm.

    The elliptic curve signature algorithm is described in full in FIPS 186-4 Section 6. Please
    refer to http://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.186-4.pdf for more information.

    Args:
        |  msg (str|bytes|bytearray): A message to be signed.
        |  d (int): The ECDSA private key of the signer.
        |  curve (fastecdsa.curve.Curve): The curve to be used to sign the message.
        |  hashfunc (Callable): The hash function used to compress the message.
        |  prehashed (bool): The message being passed has already been hashed by :code:`hashfunc`.

    Returns:
        (int, int): The signature (r, s) as a tuple.
    """
    hashed, hash_size = _hash(msg, hashfunc, prehashed)

    # generate a deterministic nonce per RFC6979
    rfc6979 = RFC6979(msg, d, curve.q, hashfunc, prehashed=prehashed)
    k = rfc6979.gen_nonce()

    field_size = (curve.q.bit_length() + 7) >> 3
    db = d.to_bytes(field_size, "little")
    kb = k.to_bytes(field_size, "little")
    z = int.from_bytes(hashed, "big")
    z >>= max(hash_size * 8 - curve.q.bit_length(), 0)

    r, s = curve.sign(z.to_bytes(field_size, "little"), db, kb)
    return int.from_bytes(r, "little"), int.from_bytes(s, "little")


def verify(
    sig: tuple[int, int],
    msg: bytes,
    Q: Point,
    curve: Curve = P256,
    hashfunc: Callable[[], HASH] = sha256,
    prehashed: bool = False,
) -> bool:
    """Verify a message signature using the elliptic curve digital signature algorithm.

    The elliptic curve signature algorithm is described in full in FIPS 186-4 Section 6. Please
    refer to http://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.186-4.pdf for more information.

    Args:
        |  sig (int, int): The signature for the message.
        |  msg (str|bytes|bytearray): A message to be signed.
        |  Q (fastecdsa.point.Point): The ECDSA public key of the signer.
        |  curve (fastecdsa.curve.Curve): The curve to be used to sign the message.
        |  hashfunc (_hashlib.HASH): The hash function used to compress the message.
        |  prehashed (bool): The message being passed has already been hashed by :code:`hashfunc`.

    Returns:
        bool: True if the signature is valid, False otherwise.

    Raises:
        fastecdsa.ecdsa.EcdsaError: If the signature or public key are invalid. Invalid signature
            in this case means that it has values less than 1 or greater than the curve order.
    """
    r, s = sig

    hashed, hash_size = _hash(msg, hashfunc, prehashed)
    field_size = (curve.q.bit_length() + 7) >> 3
    z = int.from_bytes(hashed, "big")
    z >>= max(hash_size * 8 - curve.q.bit_length(), 0)

    return curve.verify(
        r.to_bytes(field_size, "little"),
        s.to_bytes(field_size, "little"),
        z.to_bytes(field_size, "little"),
        Q.x.to_bytes(field_size, "little"),
        Q.y.to_bytes(field_size, "little"),
    )


def _hash(
    msg: bytes, hashfunc: Callable[[], HASH], prehashed: bool
) -> tuple[bytes, int]:
    if prehashed:
        return msg, len(msg)
    else:
        h = hashfunc()
        h.update(msg)
        return h.digest(), h.digest_size
