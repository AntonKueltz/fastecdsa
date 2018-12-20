from binascii import a2b_base64, hexlify
from hashlib import sha256
from os import urandom

from .curve import P256
from .ecdsa import verify
from .point import Point
from .util import mod_sqrt, msg_bytes


def gen_keypair(curve):
    """Generate a keypair that consists of a private key and a public key.

    The private key :math:`d` is an integer generated via a cryptographically secure random number
    generator that lies in the range :math:`[1,n)`, where :math:`n` is the curve order. The public
    key :math:`Q` is a point on the curve calculated as :math:`Q = dG`, where :math:`G` is the
    curve's base point.

    Args:
        curve (fastecdsa.curve.Curve): The curve over which the keypair will be calulated.

    Returns:
        long, fastecdsa.point.Point: Returns a tuple with the private key first and public key
        second.
    """
    private_key = gen_private_key(curve)
    public_key = get_public_key(private_key, curve)
    return private_key, public_key


def gen_private_key(curve):
    """Generate a private key to sign data with.

    The private key :math:`d` is an integer generated via a cryptographically secure random number
    generator that lies in the range :math:`[1,n)`, where :math:`n` is the curve order. The specific
    random number generator used is /dev/urandom.

    Args:
        curve (fastecdsa.curve.Curve): The curve over which the key will be calulated.

    Returns:
        long: Returns a positive integer smaller than the curve order.
    """
    order_bits = 0
    order = curve.q

    while order > 0:
        order >>= 1
        order_bits += 1

    order_bytes = (order_bits + 7) // 8  # urandom only takes bytes
    extra_bits = order_bytes * 8 - order_bits  # bits to shave off after getting bytes

    rand = int(hexlify(urandom(order_bytes)), 16)
    rand >>= extra_bits

    # no modding by group order or we'll introduce biases
    while rand >= curve.q:
        rand = int(hexlify(urandom(order_bytes)), 16)
        rand >>= extra_bits

    return rand


def get_public_key(d, curve):
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


def get_public_keys_from_sig(sig, msg, curve=P256, hashfunc=sha256):
    """Recover the public keys that can verify a signature / message pair.

    Args:
        |  sig (long, long): A ECDSA signature.
        |  msg (str|bytes|bytearray): The message corresponding to the signature.
        |  curve (fastecdsa.curve.Curve): The curve used to sign the message.
        |  hashfunc (_hashlib.HASH): The hash function used to compress the message.

    Returns:
        (fastecdsa.point.Point, fastecdsa.point.Point): The public keys that can verify the
                                                        signature for the message.
    """
    r, s = sig
    rinv = pow(r, curve.q - 2, curve.q)

    z = int(hashfunc(msg_bytes(msg)).hexdigest(), 16)
    hash_bit_length = hashfunc().digest_size * 8
    if curve.q.bit_length() < hash_bit_length:
        z >>= (hash_bit_length - curve.q.bit_length())

    y_squared = (r * r * r + curve.a * r + curve.b) % curve.p
    y1, y2 = mod_sqrt(y_squared, curve.p)
    R1, R2 = Point(r, y1, curve=curve), Point(r, y2, curve=curve)

    Qs = rinv * (s * R1 - z * curve.G), rinv * (s * R2 - z * curve.G)
    for Q in Qs:
        if not verify(sig, msg, Q, curve=curve, hashfunc=hashfunc):
            raise ValueError('Could not recover public key, is the signature ({}) a valid '
                             'signature for the message ({}) over the given curve ({}) using the '
                             'given hash function ({})?'.format(sig, msg, curve, hashfunc))
    return Qs


def export_key(key, curve=None, filepath=None):
    """Export a public or private EC key in PEM format.

    Args:
        |   key (fastecdsa.point.Point | long): A public or private EC key
        |   curve (fastecdsa.curve.Curve): The curve corresponding to the key (required if the
            key is a private key)
        |   filepath (str): Where to save the exported key. If None the key is simply printed.
    """
    from .point import Point
    from .asn1 import encode_keypair, encode_public_key

    if isinstance(key, Point):
        encoded = encode_public_key(key)
    elif curve is None:
        raise ValueError('curve paramter cannot be \'None\' when exporting a private key')
    else:
        pubkey = key * curve.G
        encoded = encode_keypair(key, pubkey)

    if filepath is None:
        return encoded
    else:
        f = open(filepath, 'w')
        f.write(encoded)
        f.close()


def import_key(filepath):
    """Import a public or private EC key in PEM format.

    Args:
        filepath (str): The location of the key file

    Returns:
        (long, fastecdsa.point.Point): A (private key, public key) tuple. If a public key was
        imported then the first value will be None.
    """
    from .asn1 import EC_PRIVATE_FOOTER, EC_PUBLIC_FOOTER, decode_key

    with open(filepath, 'r') as f:
        header = f.readline().rstrip()

        base64_data = ''
        line = f.readline().rstrip()

        while line and (line != EC_PRIVATE_FOOTER) and (line != EC_PUBLIC_FOOTER):
            base64_data += line
            line = f.readline().rstrip()

        raw_data = a2b_base64(base64_data)
        return decode_key(raw_data)
