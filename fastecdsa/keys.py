from os import urandom
from typing import Any, Callable, Optional, Tuple

from .curve import Curve
from .ecdsa import verify
from .encoding import KeyEncoder
from .point import Point
from .typing import EcdsaSignature, SignableMessage
from .util import mod_sqrt, msg_bytes


def gen_keypair(curve: Curve) -> Tuple[int, Point]:
    """Generate a keypair that consists of a private key and a public key.

    The private key :math:`d` is an integer generated via a cryptographically secure random number
    generator that lies in the range :math:`[1,n)`, where :math:`n` is the curve order. The public
    key :math:`Q` is a point on the curve calculated as :math:`Q = dG`, where :math:`G` is the
    curve's base point.

    Args:
        curve (fastecdsa.curve.Curve): The curve over which the keypair will be calulated.

    Returns:
        (int, fastecdsa.point.Point): Returns a tuple with the private key first and public key
        second.
    """
    private_key = gen_private_key(curve)
    public_key = get_public_key(private_key, curve)
    return private_key, public_key


def gen_private_key(curve: Curve, randfunc: Callable[[Any], bytes] = urandom) -> int:
    """Generate a private key to sign data with.

    The private key :math:`d` is an integer generated via a cryptographically secure random number
    generator that lies in the range :math:`[1,n)`, where :math:`n` is the curve order. The default
    random number generator used is /dev/urandom.

    Args:
        |  curve (fastecdsa.curve.Curve): The curve over which the key will be calulated.
        |  randfunc (function): A function taking one argument 'n' and returning a bytestring
                                of n random bytes suitable for cryptographic use.
                                The default is "os.urandom"
    Returns:
        int: Returns a positive integer smaller than the curve order.
    """
    order_bits = 0
    order = curve.q

    while order > 0:
        order >>= 1
        order_bits += 1

    order_bytes = (order_bits + 7) // 8  # randfunc only takes bytes
    extra_bits = order_bytes * 8 - order_bits  # bits to shave off after getting bytes

    rand = int.from_bytes(randfunc(order_bytes), "big")
    rand >>= extra_bits

    # no modding by group order or we'll introduce biases
    while rand < 1 or rand >= curve.q:
        rand = int.from_bytes(randfunc(order_bytes), "big")
        rand >>= extra_bits

    return rand


def get_public_key(d: int, curve: Curve) -> Point:
    """Generate a public key from a private key.

    The public key :math:`Q` is a point on the curve calculated as :math:`Q = dG`, where :math:`d`
    is the private key and :math:`G` is the curve's base point.

    Args:
        |  d (long): An integer representing the private key.
        |  curve (fastecdsa.curve.Curve): The curve over which the key will be calulated.

    Returns:
        fastecdsa.point.Point: The public key, a point on the given curve.
    """
    return d * curve.G


def get_public_keys_from_sig(
    sig: EcdsaSignature, msg: SignableMessage, curve: Curve, hashfunc: Callable
) -> Tuple[Point, Point]:
    """Recover the public keys that can verify a signature / message pair.

    Args:
        |  sig (int, int): A ECDSA signature.
        |  msg (str|bytes|bytearray): The message corresponding to the signature.
        |  curve (fastecdsa.curve.Curve): The curve used to sign the message.
        |  hashfunc (_hashlib.HASH): The hash function used to compress the message.

    Returns:
        (fastecdsa.point.Point, fastecdsa.point.Point): The public keys that can verify the
                                                        signature for the message.
    """
    r, s = sig
    rinv = pow(r, curve.q - 2, curve.q)

    z = int.from_bytes(hashfunc(msg_bytes(msg)).digest(), "big")
    hash_bit_length = hashfunc().digest_size * 8
    if curve.q.bit_length() < hash_bit_length:
        z >>= hash_bit_length - curve.q.bit_length()

    y_squared = (r * r * r + curve.a * r + curve.b) % curve.p
    y1, y2 = mod_sqrt(y_squared, curve.p)
    R1, R2 = Point(r, y1, curve=curve), Point(r, y2, curve=curve)

    Qs = rinv * (s * R1 - z * curve.G), rinv * (s * R2 - z * curve.G)
    for Q in Qs:
        if not verify(sig, msg, Q, curve=curve, hashfunc=hashfunc):
            raise ValueError(
                f"Could not recover public key, is the signature ({sig}) a valid "
                f"signature for the message ({msg!r}) over the given curve ({curve}) using the "
                f"given hash function ({hashfunc})?"
            )
    return Qs


def export_private_key(
    key: int, curve: Curve, encoder: KeyEncoder, filepath: Optional[str] = None
) -> Optional[bytes]:
    r"""Export a private EC key using the given encoder.

    Args:
        |   key (int): A private EC key.
        |   curve (fastecdsa.curve.Curve): The curve corresponding to the key.
        |   encoder (fastecdsa.encoding.KeyEncoder): An instance of an encoder that can encode a private key.
        |   filepath (str): Where to save the exported key. If :code:`None` the key is simply printed.

    Returns:
        bytes | None: If no filepath is provided the bytes of the encoded key are returned.
    """
    if not isinstance(curve, Curve):
        raise TypeError("curve must be an instance of the Curve type.")
    if not isinstance(encoder, KeyEncoder):
        raise TypeError(
            "encoder must be an instance of a subclass of the KeyEncoder type."
        )

    encoded: bytes = encoder.encode_private_key(key, curve)

    # return binary data or write to file
    if filepath is None:
        return encoded

    # write binary encoded key to disk
    with open(filepath, "wb") as f:
        f.write(encoded)

    return None


def export_public_key(
    key: Point, encoder: KeyEncoder, filepath: Optional[str] = None
) -> Optional[bytes]:
    r"""Export a private EC key using the given encoder.

    Args:
        |   key (fastecdsa.point.Point): A public EC key.
        |   encoder (fastecdsa.encoding.KeyEncoder): An instance of an encoder that can encode a private key.
        |   filepath (str): Where to save the exported key. If :code:`None` the key is simply printed.

    Returns:
        bytes | None: If no filepath is provided the bytes of the encoded key are returned.
    """
    if not isinstance(encoder, KeyEncoder):
        raise TypeError("encoder must be a subclass of KeyEncoder.")

    encoded: bytes = encoder.encode_public_key(key)

    # return binary data or write to file
    if filepath is None:
        return encoded

    # write binary encoded key to disk
    with open(filepath, "wb") as f:
        f.write(encoded)

    return None


def import_private_key(filepath: str, decoder: KeyEncoder) -> int:
    """Import a private EC key.

    Args:
        |  filepath (str): The location of the key file.
        |  decoder (fastecdsa.encoding.KeyEncoder): The decoder used to parse the key.

    Returns:
        (int): A decoded private key.
    """
    if not isinstance(decoder, KeyEncoder):
        raise TypeError("decoder must be a subclass of KeyEncoder.")

    with open(filepath, "rb") as f:
        data = f.read()

    return decoder.decode_private_key(data)


def import_public_key(filepath: str, curve: Curve, decoder: KeyEncoder) -> Point:
    """Import a public EC key.

    Args:
        |  filepath (str): The location of the key file.
        |  decoder (fastecdsa.encoding.KeyEncoder): The decoder used to parse the key.

    Returns:
        (int): A decoded private key.
    """
    if not isinstance(curve, Curve):
        raise TypeError("curve must be an instance of the Curve type.")
    if not isinstance(decoder, KeyEncoder):
        raise TypeError("decoder must be a subclass of KeyEncoder.")

    with open(filepath, "rb") as f:
        data = f.read()

    return decoder.decode_public_key(data, curve)
