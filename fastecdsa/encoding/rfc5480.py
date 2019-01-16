from binascii import b2a_base64
from textwrap import wrap

from . import KeyEncoder
from .asn1 import SEQUENCE, asn1_ecpublickey, asn1_oid, asn1_public_key, asn1_structure
from ..point import Point

EC_PUBLIC_HEADER = '-----BEGIN PUBLIC KEY-----'
EC_PUBLIC_FOOTER = '-----END PUBLIC KEY-----'


class RFC5480(KeyEncoder):
    def encode_public_key(self, Q):
        """Encode an EC public key as described in `RFC 5480 <https://tools.ietf.org/html/rfc5480>`_.

        Returns:
            str: The ASCII armored encoded EC public key.
        """
        algorithm = asn1_ecpublickey()
        oid = asn1_oid(Q.curve)
        parameters = asn1_structure(SEQUENCE, algorithm + oid)
        public_key = asn1_public_key(Q)

        sequence = parameters + public_key
        ec_public_key = asn1_structure(SEQUENCE, sequence)
        b64_data = '\n'.join(wrap(b2a_base64(ec_public_key).decode(), 64))

        return EC_PUBLIC_HEADER + '\n' + b64_data + '\n' + EC_PUBLIC_FOOTER

    def encode_private_key(self, d, Q=None, curve=None):
        raise NotImplementedError(
            'RFC5480 does not define an encoding for private EC keys (try using RFC5915)')

    def decode_public_key(self, binary_data):
        raise NotImplementedError(
            'RFC5480 does not define a decoding for public EC keys (try using RFC5915)')

    def decode_private_key(self, binary_data):
        raise NotImplementedError(
            'RFC5480 does not define a decoding for private EC keys (try using RFC5915)')
