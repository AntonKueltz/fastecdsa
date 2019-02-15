from binascii import hexlify
from hashlib import sha256  # Python standard lib SHA2 is already in C

from fastecdsa import _ecdsa
from .curve import P256
from .point import Point
from .util import RFC6979, msg_bytes


class EcdsaError(Exception):
    def __init__(self, msg):
        self.msg = msg


def sign(msg, d, curve=P256, hashfunc=sha256):
    """Sign a message using the elliptic curve digital signature algorithm.

    The elliptic curve signature algorithm is described in full in FIPS 186-4 Section 6. Please
    refer to http://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.186-4.pdf for more information.

    Args:
        |  msg (str|bytes|bytearray): A message to be signed.
        |  d (long): The ECDSA private key of the signer.
        |  curve (fastecdsa.curve.Curve): The curve to be used to sign the message.
        |  hashfunc (_hashlib.HASH): The hash function used to compress the message.
    """
    # generate a deterministic nonce per RFC6979
    rfc6979 = RFC6979(msg, d, curve.q, hashfunc)
    k = rfc6979.gen_nonce()

    hashed = hashfunc(msg_bytes(msg)).hexdigest()
    r, s = _ecdsa.sign(
        hashed,
        str(d),
        str(k),
        str(curve.p),
        str(curve.a),
        str(curve.b),
        str(curve.q),
        str(curve.gx),
        str(curve.gy)
    )
    return (int(r), int(s))


def verify(sig, msg, Q, curve=P256, hashfunc=sha256):
    """Verify a message signature using the elliptic curve digital signature algorithm.

    The elliptic curve signature algorithm is described in full in FIPS 186-4 Section 6. Please
    refer to http://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.186-4.pdf for more information.

    Args:
        |  sig (long, long): The signature for the message.
        |  msg (str|bytes|bytearray): A message to be signed.
        |  Q (fastecdsa.point.Point): The ECDSA public key of the signer.
        |  curve (fastecdsa.curve.Curve): The curve to be used to sign the message.
        |  hashfunc (_hashlib.HASH): The hash function used to compress the message.

    Returns:
        bool: True if the signature is valid, False otherwise.

    Raises:
        fastecdsa.ecdsa.EcdsaError: If the signature or public key are invalid. Invalid signature
            in this case means that it has values less than 1 or greater than the curve order.
    """
    if isinstance(Q, tuple):
        Q = Point(Q[0], Q[1], curve)
    r, s = sig

    # validate Q, r, s (Q should be validated in constructor of Point already but double check)
    if not curve.is_point_on_curve((Q.x, Q.y)):
        raise EcdsaError('Invalid public key, point is not on curve {}'.format(curve.name))
    elif r > curve.q or r < 1:
        raise EcdsaError(
            'Invalid Signature: r is not a positive integer smaller than the curve order')
    elif s > curve.q or s < 1:
        raise EcdsaError(
            'Invalid Signature: s is not a positive integer smaller than the curve order')

    hashed = hashfunc(msg_bytes(msg)).hexdigest()
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
        str(curve.gy)
    )
