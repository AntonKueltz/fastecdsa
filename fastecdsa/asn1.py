from binascii import b2a_base64, hexlify
from struct import pack, unpack
from textwrap import wrap

INTEGER = b'\x02'
BIT_STRING = b'\x03'
OCTET_STRING = b'\x04'
OBJECT_IDENTIFIER = b'\x06'
SEQUENCE = b'\x30'
PARAMETERS = b'\xa0'
PUBLIC_KEY = b'\xa1'

EC_PRIVATE_HEADER = '-----BEGIN EC PRIVATE KEY-----'
EC_PRIVATE_FOOTER = '-----END EC PRIVATE KEY-----'
EC_PUBLIC_HEADER = '-----BEGIN PUBLIC KEY-----'
EC_PUBLIC_FOOTER = '-----END PUBLIC KEY-----'

ASN1_PARSED_DATA = []


def _int_bytelen(x):
    length = 0

    while x:
        length += 1
        x >>= 8

    return length


def _int_to_bytes(x):
    bs = b''

    while x:
        bs = pack('=B', (0xff & x)) + bs
        x >>= 8

    return bs


def _bytes_to_int(bytestr):
    """Make an integer from a big endian bytestring."""
    value = 0
    for i in range(len(bytestr)):
        value = value * 256 + indexbytes(bytestr, i)
    return (value)


def _asn1_len(data):
    # https://www.itu.int/ITU-T/studygroups/com17/languages/X.690-0207.pdf
    # section 8.1.3.3
    dlen = len(data)

    if dlen < 0x80:
        return pack('=B', dlen)
    else:
        encoded = b''

        while dlen:
            len_byte = pack('=B', dlen & 0xff)
            encoded = len_byte + encoded
            dlen >>= 8

        return pack('=B', 0x80 | len(encoded)) + encoded


def _asn1_structure(data_type, data):
    return data_type + _asn1_len(data) + data


def _asn1_private_key(d, curve):
    d_bytes = _int_to_bytes(d)
    padding = b'\x00' * (_int_bytelen(curve.q) - len(d_bytes))
    return _asn1_structure(OCTET_STRING, padding + d_bytes)


def _asn1_ecversion(version=1):
    version_bytes = _int_to_bytes(version)
    return _asn1_structure(INTEGER, version_bytes)


def _asn1_ecpublickey():
    # via RFC 5480 - The "unrestricted" algorithm identifier is:
    # id-ecPublicKey OBJECT IDENTIFIER ::= {
    #   iso(1) member-body(2) us(840) ansi-X9-62(10045) keyType(2) 1 }
    return _asn1_structure(OBJECT_IDENTIFIER, b'\x2A\x86\x48\xCE\x3D\x02\x01')


def _asn1_oid(curve):
    oid_bytes = curve.oid
    return _asn1_structure(OBJECT_IDENTIFIER, oid_bytes)


def _asn1_public_key(Q):
    p_len = _int_bytelen(Q.curve.p)

    x_bytes = _int_to_bytes(Q.x)
    x_padding = b'\x00' * (p_len - len(x_bytes))

    y_bytes = _int_to_bytes(Q.y)
    y_padding = b'\x00' * (p_len - len(y_bytes))

    key_bytes = b'\x00\x04' + x_padding + x_bytes + y_padding + y_bytes
    return _asn1_structure(BIT_STRING, key_bytes)


def encode_keypair(d, Q):
    """Encode an EC keypair as described in `RFC 5915 <https://tools.ietf.org/html/rfc5915.html>`_.

    Args:
        | d (long): An ECDSA private key.
        | Q (fastecdsa.point.Point): The ECDSA public key.

    Returns:
        str: The ASCII armored encoded EC keypair.
    """
    version = _asn1_ecversion()
    private_key = _asn1_private_key(d, Q.curve)
    oid = _asn1_oid(Q.curve)
    parameters = _asn1_structure(PARAMETERS, oid)
    public_key_bitstring = _asn1_public_key(Q)
    public_key = _asn1_structure(PUBLIC_KEY, public_key_bitstring)

    sequence = version + private_key + parameters + public_key
    ec_private_key = _asn1_structure(SEQUENCE, sequence)
    b64_data = '\n'.join(wrap(b2a_base64(ec_private_key).decode(), 64))

    return EC_PRIVATE_HEADER + '\n' + b64_data + '\n' + EC_PRIVATE_FOOTER


def encode_public_key(Q):
    """Encode an EC public key as described in `RFC 5480 <https://tools.ietf.org/html/rfc5480>`_.

    Args:
        Q (fastecdsa.point.Point): The ECDSA public key.

    Returns:
        str: The ASCII armored encoded EC public key.
    """
    algorithm = _asn1_ecpublickey()
    oid = _asn1_oid(Q.curve)
    parameters = _asn1_structure(SEQUENCE, algorithm + oid)
    public_key = _asn1_public_key(Q)

    sequence = parameters + public_key
    ec_public_key = _asn1_structure(SEQUENCE, sequence)
    b64_data = '\n'.join(wrap(b2a_base64(ec_public_key).decode(), 64))

    return EC_PUBLIC_HEADER + '\n' + b64_data + '\n' + EC_PUBLIC_FOOTER


def indexbytes(buf, idx):
    """ Python2 and 3 compatibility for indexing bytes. Same as six.indexbytes"""
    if idx >= len(buf):
        raise IndexError("string index out of range")
    return ord(buf[idx:idx + 1])


def der_encode_signature(r, s):
    """Encode an EC signature in serialized DER format as described in
       https://tools.ietf.org/html/rfc2459 (section 7.2.2) and as detailed by
       bip-0066

    Args:
        r, s

    Returns:
        bytes: The DER encoded signature

    """
    r_bytes = _int_to_bytes(r)
    if indexbytes(r_bytes, 0) & 0x80:
        r_bytes = b"\x00" + r_bytes
    s_bytes = _int_to_bytes(s)
    if indexbytes(s_bytes, 0) & 0x80:
        s_bytes = b"\x00" + s_bytes
    r_s = INTEGER + pack('B', len(r_bytes)) + r_bytes + INTEGER + pack('B', len(s_bytes)) + s_bytes
    return SEQUENCE + pack('B', len(r_s)) + r_s


class InvalidDerSignature(Exception):
    pass


def der_decode_signature(sig):
    """Decode an EC signature from serialized DER format as described in
       https://tools.ietf.org/html/rfc2459 (section 7.2.2) and as detailed by
       bip-0066

       Returns (r,s)
    """
    if len(sig) < 8:
        raise InvalidDerSignature("bytestring too small")
    if indexbytes(sig, 0) != ord(SEQUENCE):
        raise InvalidDerSignature("missing SEQUENCE marker")
    if indexbytes(sig, 1) != len(sig) - 2:
        raise InvalidDerSignature("invalid length")
    length_r = indexbytes(sig, 3)
    if 5 + length_r >= len(sig):
        raise InvalidDerSignature("invalid length")
    length_s = indexbytes(sig, 5 + length_r)
    if length_r + length_s + 6 != len(sig):
        raise InvalidDerSignature("invalid length")
    if indexbytes(sig, 2) != ord(INTEGER):
        raise InvalidDerSignature("invalid r marker")
    if length_r == 0:
        raise InvalidDerSignature("invalid r value")
    if indexbytes(sig, 4) & 0x80:
        raise InvalidDerSignature("invalid r value")
    if (length_r > 1 and (indexbytes(sig, 4) == 0x00) and not (indexbytes(sig, 5) & 0x80)):
        raise InvalidDerSignature("invalid r value")
    if indexbytes(sig, length_r + 4) != ord(INTEGER):
        raise InvalidDerSignature("invalid s marker")
    if length_s == 0:
        raise InvalidDerSignature("invalid s value")
    if indexbytes(sig, length_r + 6) & 0x80:
        raise InvalidDerSignature("invalid s value")
    if (length_s > 1 and (indexbytes(sig, length_r + 6) == 0x00) and not (indexbytes(sig, length_r + 7) & 0x80)):
        raise InvalidDerSignature("invalid s value")
    r_data, s_data = sig[4:4 + length_r], sig[6 + length_r:]
    return _bytes_to_int(r_data), _bytes_to_int(s_data)


def _parse_asn1_length(data):
    (initial_byte,) = unpack('=B', data[:1])
    data = data[1:]

    if not (initial_byte & 0x80):
        length = initial_byte
    else:
        count = initial_byte & 0x7f
        fmt = {1: '=B', 2: '=H', 4: '=L'}

        (length,) = unpack(fmt[count], data[:count])
        data = data[count:]

    return length, data[:length], data[length:]


def _parse_asn1_structure(data):
    global ASN1_PARSED_DATA

    data_type = data[:1]
    length, data, remaining = _parse_asn1_length(data[1:])

    if data_type in [OCTET_STRING, BIT_STRING, OBJECT_IDENTIFIER]:
        ASN1_PARSED_DATA.append((data_type, data))
    elif data_type in [SEQUENCE, PUBLIC_KEY, PARAMETERS]:
        _parse_asn1_structure(data)

    if remaining:
        _parse_asn1_structure(remaining)


def decode_key(pemdata):
    """Decode an EC key as described in `RFC 5915 <https://tools.ietf.org/html/rfc5915.html>`_ and
    `RFC 5480 <https://tools.ietf.org/html/rfc5480>`_.

    Args:
        pemdata (bytes): A sequence of bytes representing an encoded EC key.

    Returns:
        (long, fastecdsa.point.Point): A private key, public key tuple. If the encoded key was a
        public key the first entry in the tuple is None.
    """
    from .curve import Curve
    from .point import Point

    global ASN1_PARSED_DATA
    _parse_asn1_structure(pemdata)

    d, x, y, curve = None, None, None, None
    for (value_type, value) in ASN1_PARSED_DATA:
        if value_type == OCTET_STRING:
            d = int(hexlify(value), 16)
        elif value_type == OBJECT_IDENTIFIER and curve is None:
            curve = Curve.get_curve_by_oid(value)
        elif value_type == BIT_STRING:
            value = value.lstrip(b'\x00\x04')
            x = int(hexlify(value[:len(value) // 2]), 16)
            y = int(hexlify(value[len(value) // 2:]), 16)

    ASN1_PARSED_DATA = []
    Q = None if (x is None) or (y is None) else Point(x, y, curve)
    return d, Q
