from os import urandom


def gen_private_key(curve):
    order_bits = 0
    order = curve.q

    while order > 0:
        order >>= 1
        order_bits += 1

    order_bytes = (order_bits + 7) / 8  # urandom only takes bytes
    extra_bits = order_bytes * 8 - order_bits  # bits to shave off after getting bytes

    rand = int(urandom(order_bytes).encode('hex'), 16)
    rand >>= extra_bits

    # no modding by group order or we'll introduce biases
    while rand >= curve.q:
        rand = int(urandom(order_bytes).encode('hex'), 16)
        rand >>= extra_bits

    return rand


def get_public_key(d, curve):
    return curve.point_mul(curve.G, d)
