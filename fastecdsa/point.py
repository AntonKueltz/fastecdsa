from fastecdsa import curvemath
from .curve import P256


class CurveMismatchError(Exception):
    def __init__(self, curve1, curve2):
        self.msg = 'Tried to add points on two different curves <{}> & <{}>'.format(
            curve1.name, curve2.name)


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
        if not curve.is_point_on_curve((x, y)):
            raise ValueError('(x, y) coordinates are not on curve <{}>'.format(curve.name))
        else:
            self.x = x
            self.y = y
            self.curve = curve

    def __str__(self):
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
        if self.curve is not other.curve:
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
        return Point(self.x, -self.y % self.curve.p, self.curve)
