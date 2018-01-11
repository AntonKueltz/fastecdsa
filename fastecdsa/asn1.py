from binascii import b2a_base64, hexlify
from struct import pack
from textwrap import wrap

INTEGER = b'\x02'
BIT_STRING = b'\x03'
OCTET_STRING = b'\x04'
OBJECT_IDENTIFIER = b'\x06'
SEQUENCE = b'\x30'
PARAMETERS = b'\xa0'
PUBLIC_KEY = b'\xa1'

EC_PRIVATE_HEADER = '-----BEGIN EC PRIVATE KEY-----\n'
EC_PRIVATE_FOOTER = '\n-----END EC PRIVATE KEY-----'
EC_PUBLIC_HEADER = '-----BEGIN PUBLIC KEY-----\n'
EC_PUBLIC_FOOTER = '\n-----END PUBLIC KEY-----'


def _int_bytelen(x):
    length = 0

    while x:
        length += 1
        x >>= 8

    return length


def _int_to_bytes(x):
    bs = ''

    while x:
        bs = pack('=B', (0xff & x)) + bs
        x >>= 8

    return bs


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
    """Encode a keypair as described in RFC 5915

    https://tools.ietf.org/html/rfc5915.html
    """
    version = _asn1_ecversion()
    private_key = _asn1_private_key(d, Q.curve)
    oid = _asn1_oid(Q.curve)
    parameters = _asn1_structure(PARAMETERS, oid)
    public_key_bitstring = _asn1_public_key(Q)
    public_key = _asn1_structure(PUBLIC_KEY, public_key_bitstring)

    sequence = version + private_key + parameters + public_key
    ec_private_key = _asn1_structure(SEQUENCE, sequence)
    b64_data = '\n'.join(wrap(b2a_base64(ec_private_key), 64))

    return EC_PRIVATE_HEADER + b64_data + EC_PRIVATE_FOOTER


def encode_public_key(Q):
    """Encode a public key as described in RFC 5480

    https://tools.ietf.org/html/rfc5480
    """
    algorithm = _asn1_ecpublickey()
    oid = _asn1_oid(Q.curve)
    parameters = _asn1_structure(SEQUENCE, algorithm + oid)
    public_key = _asn1_public_key(Q)

    sequence = parameters + public_key
    ec_public_key = _asn1_structure(SEQUENCE, sequence)
    b64_data = '\n'.join(wrap(b2a_base64(ec_public_key), 64))

    return EC_PUBLIC_HEADER + b64_data + EC_PUBLIC_FOOTER
