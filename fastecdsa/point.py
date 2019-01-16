from fastecdsa import curvemath
from .curve import P256
from .asn1 import _int_to_bytes, _int_bytelen, _bytes_to_int
from .util import mod_sqrt


class CurveMismatchError(Exception):
    def __init__(self, curve1, curve2):
        self.msg = 'Tried to add points on two different curves <{}> & <{}>'.format(
            curve1.name, curve2.name)


class InvalidSEC1PublicKey(Exception):
    pass


class Point:
    """Representation of a point on an elliptic curve.

    Attributes:
        |  x (long): The x coordinate of the point.
        |  y (long): The y coordinate of the point.
        |  curve (:class:`Curve`): The curve that the point lies on.
    """

    def __init__(self, x, y, curve=P256):
        """Initialize a point on an elliptic curve.

        The x and y parameters must satisfy the equation :math:`y^2 \equiv x^3 + ax + b \pmod{p}`,
        where a, b, and p are attributes of the curve parameter.

        Args:
            |  x (long): The x coordinate of the point.
            |  y (long): The y coordinate of the point.
            |  curve (:class:`Curve`): The curve that the point lies on.
        """
        if not (x == 0 and y == 1 and curve is None) and not curve.is_point_on_curve((x, y)):
            raise ValueError(
                'coordinates are not on curve <{}>\n\tx={:x}\n\ty={:x}'.format(curve.name, x, y))
        else:
            self.x = x
            self.y = y
            self.curve = curve

    @classmethod
    def decode(cls, curve, key):
        """ Decode a public key as described in http://www.secg.org/SEC1-Ver-1.0.pdf
            in sections 2.3.3/2.3.4

                compressed:     04 + x_bytes + y_bytes
                uncompressed:   02 or 03 + x_bytes

        Args:
            curve (Curve): Curve to use when decoding the public key
            key (bytes): public key encoded using the SEC1 format

        Returns:
            Point: The decoded public key

        Raises:
            InvalidSEC1PublicKey
        """
        bytelen = _int_bytelen(curve.q)
        if key.startswith(b'\x04'):        # uncompressed key
            if len(key) != bytelen * 2 + 1:
                raise InvalidSEC1PublicKey('An uncompressed public key must be %d bytes long' % (bytelen * 2 + 1))
            x, y = _bytes_to_int(key[1:bytelen + 1]), _bytes_to_int(key[bytelen + 1:])
        else:                              # compressed key
            if len(key) != bytelen + 1:
                raise InvalidSEC1PublicKey('A compressed public key must be %d bytes long' % (bytelen + 1))
            x = _bytes_to_int(key[1:])
            root = mod_sqrt(curve.evaluate(x), curve.p)[0]
            if key.startswith(b'\x03'):    # odd root
                y = root if root % 2 == 1 else -root % curve.p
            elif key.startswith(b'\x02'):  # even root
                y = root if root % 2 == 0 else -root % curve.p
            else:
                raise InvalidSEC1PublicKey('Wrong key format')
        return cls(x, y, curve=curve)

    def encode(self, compressed=True):
        """ Encode a public key as described in http://www.secg.org/SEC1-Ver-1.0.pdf
            in sections 2.3.3/2.3.4
                compressed:     04 + x_bytes + y_bytes
                uncompressed:   02 or 03 + x_bytes
        Args:
            compressed (bool): Set to False if you want an uncompressed format

        Returns:
            bytes: The SEC1 encoded public key
        """
        bytelen = _int_bytelen(self.curve.q)
        if compressed:
            if self.y & 1:  # odd root
                return b'\x03' + _int_to_bytes(self.x).rjust(bytelen, b'\x00')
            else:           # even root
                return b'\x02' + _int_to_bytes(self.x).rjust(bytelen, b'\x00')
        return b'\x04' + _int_to_bytes(self.x).rjust(bytelen, b'\x00') + _int_to_bytes(self.y).rjust(bytelen, b'\x00')

    def __str__(self):
        if self == self.IDENTITY_ELEMENT:
            return '<POINT AT INFINITY>'
        else:
            return 'X: 0x{:x}\nY: 0x{:x}\n(On curve <{}>)'.format(self.x, self.y, self.curve.name)

    def __unicode__(self):
        return self.__str__()

    def __repr__(self):
        return self.__str__()

    def __eq__(self, other):
        return self.x == other.x and self.y == other.y and self.curve is other.curve

    def __add__(self, other):
        """Add two points on the same elliptic curve.

        Args:
            | self (:class:`Point`): a point :math:`P` on the curve
            | other (:class:`Point`): a point :math:`Q` on the curve

        Returns:
            :class:`Point`: A point :math:`R` such that :math:`R = P + Q`
        """
        if self == self.IDENTITY_ELEMENT:
            return other
        elif other == self.IDENTITY_ELEMENT:
            return self
        elif self.curve is not other.curve:
            raise CurveMismatchError(self.curve, other.curve)
        else:
            x, y = curvemath.add(
                str(self.x),
                str(self.y),
                str(other.x),
                str(other.y),
                str(self.curve.p),
                str(self.curve.a),
                str(self.curve.b),
                str(self.curve.q),
                str(self.curve.gx),
                str(self.curve.gy)
            )
            return Point(int(x), int(y), self.curve)

    def __radd__(self, other):
        """Add two points on the same elliptic curve.

        Args:
            | self (:class:`Point`): a point :math:`P` on the curve
            | other (:class:`Point`): a point :math:`Q` on the curve

        Returns:
            :class:`Point`: A point :math:`R` such that :math:`R = P + Q`
        """
        return self.__add__(other)

    def __sub__(self, other):
        """Subtract two points on the same elliptic curve.

        Args:
            | self (:class:`Point`): a point :math:`P` on the curve
            | other (:class:`Point`): a point :math:`Q` on the curve

        Returns:
            :class:`Point`: A point :math:`R` such that :math:`R = P - Q`
        """
        if self == other:
            return self.IDENTITY_ELEMENT
        elif other == self.IDENTITY_ELEMENT:
            return self

        negative = Point(other.x, -other.y % other.curve.p, other.curve)
        return self.__add__(negative)

    def __mul__(self, scalar):
        """Multiply a :class:`Point` on an elliptic curve by an integer.

        Args:
            | self (:class:`Point`): a point :math:`P` on the curve
            | other (long): an integer :math:`d \in \mathbb{Z_q}` where :math:`q` is the order of
                the curve that :math:`P` is on

        Returns:
            :class:`Point`: A point :math:`R` such that :math:`R = P * d`
        """
        try:
            d = int(scalar) % self.curve.q
        except ValueError:
            raise TypeError('Curve point multiplication must be by an integer')
        else:
            if d == 0:
                return self.IDENTITY_ELEMENT

            x, y = curvemath.mul(
                str(self.x),
                str(self.y),
                str(d),
                str(self.curve.p),
                str(self.curve.a),
                str(self.curve.b),
                str(self.curve.q),
                str(self.curve.gx),
                str(self.curve.gy)
            )
            return Point(int(x), int(y), self.curve)

    def __rmul__(self, scalar):
        """Multiply a :class:`Point` on an elliptic curve by an integer.

        Args:
            | self (:class:`Point`): a point :math:`P` on the curve
            | other (long): an integer :math:`d \in \mathbb{Z_q}` where :math:`q` is the order of
                the curve that :math:`P` is on

        Returns:
            :class:`Point`: A point :math:`R` such that :math:`R = d * P`
        """
        return self.__mul__(scalar)

    def __neg__(self):
        """Return the negation of a :class:`Point` i.e. the points reflection over the x-axis.

        Args:
            | self (:class:`Point`): a point :math:`P` on the curve

        Returns:
            :class:`Point`: A point :math:`R = (P_x, -P_y)`
        """
        if self == self.IDENTITY_ELEMENT:
            return self

        return Point(self.x, -self.y % self.curve.p, self.curve)


Point.IDENTITY_ELEMENT = Point(0, 1, curve=None)  # also known as the point at infinity
