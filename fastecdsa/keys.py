from binascii import hexlify
from os import urandom


def gen_keypair(curve):
    ''' Generate a tuple (private, public) where private is in integer and public is a tuple
    (integer, integer)'''
    private_key = gen_private_key(curve)
    public_key = get_public_key(private_key, curve)
    return private_key, public_key


def gen_private_key(curve):
    ''' Only generate a private key. This can be useful if you want to delay generating the public
    key, which is a bit of an expensive operation'''
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
    ''' Get the curve point corresponding to the curves generator point multiplied by the private
    key "d"'''
    return d * curve.G
