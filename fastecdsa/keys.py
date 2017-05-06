from binascii import hexlify, unhexlify, a2b_base64, b2a_base64
from os import urandom
from struct import pack, unpack
from textwrap import wrap


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
        fastecdsa.point.Point: The public key, a point on the given curve.``
    """
    return d * curve.G


def _int_to_bytes(x):
    bs = ''

    while x:
        bs = pack('=B', (0xff & x)) + bs
        x >>= 8

    return bs


def encode_keypair(d, Q):
    header = '-----BEGIN EC PRIVATE KEY-----\n'
    footer = '\n-----END EC PRIVATE KEY-----'

    d_bytes = _int_to_bytes(d)
    x_bytes = _int_to_bytes(Q.x)
    y_bytes = _int_to_bytes(Q.y)

    d_len = pack('=B', len(d_bytes))
    param_len = pack('=B', len(Q.curve.oid) + 2)
    oid_len = pack('=B', len(Q.curve.oid))
    key_data_len = pack('=B', len(x_bytes + y_bytes) + 4)
    xy_len = pack('=B', len(x_bytes + y_bytes) + 2)

    bit_string = '\x02\x01\x01\x04{}{}\xa0{}\x06{}{}\xa1{}\x03{}\x00\x04{}{}'.format(
        d_len, d_bytes, param_len, oid_len, Q.curve.oid, key_data_len, xy_len, x_bytes, y_bytes)
    body = '\x30{}{}'.format(pack('=B', len(bit_string)), bit_string)

    return header + '\n'.join(wrap(b2a_base64(body), 64)) + footer


def encode_public_key(Q):
    header = '-----BEGIN PUBLIC KEY-----\n'
    footer = '\n-----END PUBLIC KEY-----'

    x_bytes = _int_to_bytes(Q.x)
    y_bytes = _int_to_bytes(Q.y)

    param_len = pack('=B', len(Q.curve.oid) + 2)
    oid_len = pack('=B', len(Q.curve.oid))
    key_data_len = pack('=B', len(x_bytes + y_bytes) + 4)
    xy_len = pack('=B', len(x_bytes + y_bytes) + 2)

    bit_string = '\x02\x01\x01\xa0{}\x06{}{}\xa1{}\x03{}\x00\x04{}{}'.format(
        param_len, oid_len, Q.curve.oid, key_data_len, xy_len, x_bytes, y_bytes)
    body = '\x30{}{}'.format(pack('=B', len(bit_string)), bit_string)

    return header + '\n'.join(wrap(b2a_base64(body), 64)) + footer
