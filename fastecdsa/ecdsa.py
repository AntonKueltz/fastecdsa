from binascii import hexlify
from hashlib import sha256

from fastecdsa import _ecdsa  # type: ignore
from .curve import Curve, P256
from .point import Point
from .typing import EcdsaSignature, HashFunction, SignableMessage
from .util import RFC6979, msg_bytes


class EcdsaError(Exception):
    def __init__(self, msg: str) -> None:
        self.msg = msg


def sign(
    msg: SignableMessage,
    d: int,
    curve: Curve = P256,
    hashfunc: HashFunction = sha256,
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
    # generate a deterministic nonce per RFC6979
    rfc6979 = RFC6979(msg, d, curve.q, hashfunc, prehashed=prehashed)
    k = rfc6979.gen_nonce()

    # Fix the bit-length of the random nonce,
    # so that it doesn't leak via timing.
    # This does not change that ks (mod n) = kt (mod n) = k (mod n)
    ks = k + curve.q
    kt = ks + curve.q
    if ks.bit_length() == curve.q.bit_length():
        k = kt
    else:
        k = ks

    hashed = _hex_digest(msg, hashfunc, prehashed)

    r, s = _ecdsa.sign(
        hashed,
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
    hashfunc: HashFunction = sha256,
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

    hashed = _hex_digest(msg, hashfunc, prehashed)

    return _ecdsa.verify(
        str(r),
        str(s),
        hashed,
        str(Q.x),
        str(Q.y),
        str(curve.p),
        str(curve.a),
        str(curve.b),
        str(curve.q),
        str(curve.gx),
        str(curve.gy),
    )


def _hex_digest(msg: SignableMessage, hashfunc: HashFunction, prehashed: bool) -> str:
    if prehashed:
        if not isinstance(msg, (bytes, bytearray)):
            raise TypeError(f"Prehashed message must be bytes, got {type(msg)}")
        return hexlify(msg).decode()
    else:
        return hashfunc(msg_bytes(msg)).hexdigest()
