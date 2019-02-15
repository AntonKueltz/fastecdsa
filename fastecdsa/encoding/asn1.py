from struct import pack, unpack

from .util import int_bytelen, int_to_bytes

INTEGER = b'\x02'
BIT_STRING = b'\x03'
OCTET_STRING = b'\x04'
OBJECT_IDENTIFIER = b'\x06'
SEQUENCE = b'\x30'
PARAMETERS = b'\xa0'
PUBLIC_KEY = b'\xa1'


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


def asn1_structure(data_type, data):
    return data_type + _asn1_len(data) + data


def asn1_private_key(d, curve):
    d_bytes = int_to_bytes(d)
    padding = b'\x00' * (int_bytelen(curve.q) - len(d_bytes))
    return asn1_structure(OCTET_STRING, padding + d_bytes)


def asn1_ecversion(version=1):
    version_bytes = int_to_bytes(version)
    return asn1_structure(INTEGER, version_bytes)


def asn1_ecpublickey():
    # via RFC 5480 - The "unrestricted" algorithm identifier is:
    # id-ecPublicKey OBJECT IDENTIFIER ::= {
    #   iso(1) member-body(2) us(840) ansi-X9-62(10045) keyType(2) 1 }
    return asn1_structure(OBJECT_IDENTIFIER, b'\x2A\x86\x48\xCE\x3D\x02\x01')


def asn1_oid(curve):
    oid_bytes = curve.oid
    return asn1_structure(OBJECT_IDENTIFIER, oid_bytes)


def asn1_public_key(Q):
    p_len = int_bytelen(Q.curve.p)

    x_bytes = int_to_bytes(Q.x)
    x_padding = b'\x00' * (p_len - len(x_bytes))

    y_bytes = int_to_bytes(Q.y)
    y_padding = b'\x00' * (p_len - len(y_bytes))

    key_bytes = b'\x00\x04' + x_padding + x_bytes + y_padding + y_bytes
    return asn1_structure(BIT_STRING, key_bytes)


def parse_asn1_length(data):
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
