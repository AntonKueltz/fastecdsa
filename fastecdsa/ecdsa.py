from _hashlib import HASH
from collections.abc import Callable
from hashlib import sha256

from fastecdsa import _ecdsa  # type: ignore[attr-defined]
from .curve import Curve, P192, P256
from .point import Point
from .typing import EcdsaSignature, SignableMessage
from .util import RFC6979, msg_bytes

import fastecdsa_rs


class EcdsaError(Exception):
    def __init__(self, msg: str) -> None:
        self.msg = msg


def sign(
    msg: SignableMessage,
    d: int,
    curve: Curve = P256,
    hashfunc: Callable[[], HASH] = sha256,
    prehashed: bool = False,
) -> EcdsaSignature:
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

    # TODO - Fix the bit-length of the random nonce,
    # so that it doesn't leak via timing.
    # This does not change that ks (mod n) = kt (mod n) = k (mod n)
    # ks = k + curve.q
    # kt = ks + curve.q
    # if ks.bit_length() == curve.q.bit_length():
    #     k = kt
    # else:
    #     k = ks

    if curve in {P192}:
        field_size = curve.q_size_bytes
        return rust_sign(
            hashed,
            d.to_bytes(field_size, "little"),
            k.to_bytes(field_size, "little"),
            hash_size,
            field_size,
        )
    else:
        return _c_sign(hashed, d, k, curve)


def rust_sign(
    hashed: bytes, d: bytes, k: bytes, hash_size_bytes: int, field_size: int
) -> EcdsaSignature:
    z = int.from_bytes(hashed, "big")
    z >>= max(hash_size_bytes * 8 - field_size * 8, 0)
    r, s = fastecdsa_rs.p192_sign(z.to_bytes(field_size, "little"), d, k)
    return int.from_bytes(r, "little"), int.from_bytes(s, "little")


def _c_sign(hashed: bytes, d: int, k: int, curve: Curve) -> EcdsaSignature:
    r, s = _ecdsa.sign(
        hashed.hex(),
        str(d),
        str(k),
        str(curve.p),
        str(curve.a),
        str(curve.b),
        str(curve.q),
        str(curve.gx),
        str(curve.gy),
    )
    return int(r), int(s)


def verify(
    sig: EcdsaSignature,
    msg: SignableMessage,
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

    # validate Q, r, s (Q should be validated in constructor of Point already but double check)
    if not curve.is_point_on_curve((Q.x, Q.y)):
        raise EcdsaError(f"Invalid public key, point is not on curve {curve}")
    elif r > curve.q or r < 1:
        raise EcdsaError(
            "Invalid Signature: r is not a positive integer smaller than the curve order"
        )
    elif s > curve.q or s < 1:
        raise EcdsaError(
            "Invalid Signature: s is not a positive integer smaller than the curve order"
        )

    hashed, hash_size = _hash(msg, hashfunc, prehashed)
    if curve in {P192}:
        return _rust_verify(sig, hashed, Q, hash_size, curve.q_size_bytes)
    else:
        return _c_verify(sig, hashed, Q, curve)


def _rust_verify(
    sig: EcdsaSignature, hashed: bytes, Q: Point, hash_size_bytes: int, field_size: int
) -> bool:
    r, s = sig
    z = int.from_bytes(hashed, "big")
    z >>= max(hash_size_bytes * 8 - field_size * 8, 0)
    return fastecdsa_rs.p192_verify(
        r.to_bytes(field_size, "little"),
        s.to_bytes(field_size, "little"),
        z.to_bytes(field_size, "little"),
        Q.x.to_bytes(field_size, "little"),
        Q.y.to_bytes(field_size, "little"),
    )


def _c_verify(sig: EcdsaSignature, hashed: bytes, Q: Point, curve: Curve) -> bool:
    r, s = sig

    return _ecdsa.verify(
        str(r),
        str(s),
        hashed.hex(),
        str(Q.x),
        str(Q.y),
        str(curve.p),
        str(curve.a),
        str(curve.b),
        str(curve.q),
        str(curve.gx),
        str(curve.gy),
    )


def _hash(
    msg: SignableMessage, hashfunc: Callable[[], HASH], prehashed: bool
) -> tuple[bytes, int]:
    if prehashed:
        if not isinstance(msg, (bytes, bytearray)):
            raise TypeError(f"Prehashed message must be bytes, got {type(msg)}")
        bs = msg_bytes(msg)
        return bs, len(bs)
    else:
        h = hashfunc()
        h.update(msg_bytes(msg))
        return h.digest(), h.digest_size
