from binascii import a2b_base64, b2a_base64, hexlify
from textwrap import wrap
from typing import List, Tuple

from . import KeyEncoder
from .asn1 import (
    BIT_STRING,
    OBJECT_IDENTIFIER,
    OCTET_STRING,
    PARAMETERS,
    PUBLIC_KEY,
    SEQUENCE,
    asn1_ecpublickey,
    asn1_ecversion,
    asn1_oid,
    asn1_private_key,
    asn1_public_key,
    asn1_structure,
    parse_asn1_length,
)
from ..curve import Curve
from ..point import Point


class PEMEncoderError(Exception):
    def __init__(self, msg: str) -> None:
        self.msg = msg


class PEMEncoder(KeyEncoder):
    EC_PRIVATE_HEADER = b"-----BEGIN EC PRIVATE KEY-----"
    EC_PRIVATE_FOOTER = b"-----END EC PRIVATE KEY-----"

    EC_PUBLIC_HEADER = b"-----BEGIN PUBLIC KEY-----"
    EC_PUBLIC_FOOTER = b"-----END PUBLIC KEY-----"

    def __init__(self) -> None:
        self.asn1_parsed_data: List[Tuple[bytes, bytes]] = []

    @classmethod
    def _parse_ascii_armored_base64(cls, data: bytes) -> bytes:
        """Convert an ASCII armored key to raw binary data"""
        data = data.strip()
        lines = (line for line in data.split(b"\n"))
        next(lines).rstrip()  # header lines

        base64_data = b""
        line = next(lines).rstrip()

        while (
            line and (line != cls.EC_PRIVATE_FOOTER) and (line != cls.EC_PUBLIC_FOOTER)
        ):
            base64_data += line
            line = next(lines).rstrip()

        return a2b_base64(base64_data)

    def _parse_asn1_structure(self, data: bytes) -> None:
        """Recursively parse ASN.1 data"""
        data_type = data[:1]
        _, data, remaining = parse_asn1_length(data[1:])

        if data_type in [OCTET_STRING, BIT_STRING, OBJECT_IDENTIFIER]:
            self.asn1_parsed_data.append((data_type, data))
        elif data_type in [SEQUENCE, PUBLIC_KEY, PARAMETERS]:
            self._parse_asn1_structure(data)

        if remaining:
            self._parse_asn1_structure(remaining)

    def encode_public_key(self, Q: Point) -> bytes:
        """Encode an EC public key as described in `RFC 5480 <https://tools.ietf.org/html/rfc5480>`_.

        Args:
            | Q (fastecdsa.point.Point): An ECDSA public key.

        Returns:
            bytes: The ASCII armored encoded EC public key.
        """
        algorithm = asn1_ecpublickey()
        oid = asn1_oid(Q.curve)
        parameters = asn1_structure(SEQUENCE, algorithm + oid)
        public_key = asn1_public_key(Q)

        sequence = parameters + public_key
        ec_public_key = asn1_structure(SEQUENCE, sequence)
        b64_data = "\n".join(wrap(b2a_base64(ec_public_key).decode(), 64))

        return (
            self.EC_PUBLIC_HEADER
            + b"\n"
            + b64_data.encode()
            + b"\n"
            + self.EC_PUBLIC_FOOTER
        )

    def encode_private_key(self, d: int, curve: Curve) -> bytes:
        """Encode a private EC key as described in `RFC 5915 <https://tools.ietf.org/html/rfc5915.html>`_.

        Args:
            | d (int): An ECDSA private key.
            | curve (fastecdsa.curve.Curve): The curve that the private key is for.

        Returns:
            bytes: The ASCII armored encoded EC key.
        """
        Q: Point = d * curve.G

        version = asn1_ecversion()
        private_key = asn1_private_key(d, Q.curve)
        oid = asn1_oid(Q.curve)
        parameters = asn1_structure(PARAMETERS, oid)
        public_key_bitstring = asn1_public_key(Q)
        public_key = asn1_structure(PUBLIC_KEY, public_key_bitstring)

        sequence = version + private_key + parameters + public_key
        ec_private_key = asn1_structure(SEQUENCE, sequence)
        b64_data = "\n".join(wrap(b2a_base64(ec_private_key).decode(), 64))

        return (
            self.EC_PRIVATE_HEADER
            + b"\n"
            + b64_data.encode()
            + b"\n"
            + self.EC_PRIVATE_FOOTER
        )

    def decode_public_key(self, pemdata: bytes, curve: Curve) -> Point:
        """Decode a PEM encoded public key as described in
        `RFC 5480 <https://tools.ietf.org/html/rfc5480>`_.

        Args:
            pemdata (bytes): A sequence of bytes representing an encoded EC key.

        Returns:
            (long, fastecdsa.point.Point): A private key, public key tuple. If the encoded key was a
            public key the first entry in the tuple is None.
        """
        parsed = self._parse_ascii_armored_base64(pemdata)
        self._parse_asn1_structure(parsed)

        x, y = None, None
        for value_type, value in self.asn1_parsed_data:
            if value_type == OBJECT_IDENTIFIER:
                # override curve if explicitly defined in the encoded key
                encoded_curve = Curve.get_curve_by_oid(value)
                if encoded_curve is not None:
                    encoded_curve = curve

            elif value_type == BIT_STRING:
                value = value[2:]  # strip off b'\x00\x04'
                x = int.from_bytes(value[: len(value) // 2], "big")
                y = int.from_bytes(value[len(value) // 2 :], "big")

        self.asn1_parsed_data = []

        if curve is None or x is None or y is None:
            raise PEMEncoderError(f"Could not parse public key. {x=}, {y=}, {curve=}")

        return Point(x, y, curve)

    def decode_private_key(self, pemdata: bytes) -> int:
        """Decode a PEM encoded EC private key as described in
        `RFC 5915 <https://tools.ietf.org/html/rfc5915.html>`_.

        Args:
            pemdata (bytes): A sequence of bytes representing an encoded EC key.

        Returns:
            int: The private key.
        """
        parsed = self._parse_ascii_armored_base64(pemdata)
        self._parse_asn1_structure(parsed)

        d = None
        for value_type, value in self.asn1_parsed_data:
            if value_type == OCTET_STRING:
                d = int(hexlify(value), 16)

        self.asn1_parsed_data = []

        if d is None:
            raise PEMEncoderError("Could not parse private key.")

        return d
