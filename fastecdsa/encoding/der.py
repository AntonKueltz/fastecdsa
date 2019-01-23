from six import indexbytes
from struct import pack

from . import SigEncoder
from .asn1 import INTEGER, SEQUENCE
from .util import bytes_to_int, int_to_bytes


class InvalidDerSignature(Exception):
    pass


class DEREncoder(SigEncoder):
    @staticmethod
    def encode_signature(r, s):
        """Encode an EC signature in serialized DER format as described in
           https://tools.ietf.org/html/rfc2459 (section 7.2.2) and as detailed by
           bip-0066

        Args:
            r, s

        Returns:
            bytes: The DER encoded signature

        """
        r_bytes = int_to_bytes(r)
        if indexbytes(r_bytes, 0) & 0x80:
            r_bytes = b"\x00" + r_bytes
        s_bytes = int_to_bytes(s)
        if indexbytes(s_bytes, 0) & 0x80:
            s_bytes = b"\x00" + s_bytes
        r_s = INTEGER + pack('B', len(r_bytes)) + r_bytes + INTEGER + pack('B', len(s_bytes)) + s_bytes
        return SEQUENCE + pack('B', len(r_s)) + r_s

    @staticmethod
    def decode_signature(sig):
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
        return bytes_to_int(r_data), bytes_to_int(s_data)
