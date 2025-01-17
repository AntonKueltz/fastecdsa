from __future__ import annotations

from fastecdsa import curvemath  # type: ignore
from .curve import Curve


class CurveMismatchError(Exception):
    def __init__(self, curve1: Curve, curve2: Curve) -> None:
        self.msg = (
            f"Tried to add points on two different curves <{curve1}> & <{curve2}>"
        )


class Point:
    """Representation of a point on an elliptic curve.

    Attributes:
        |  x (int): The x coordinate of the point.
        |  y (int): The y coordinate of the point.
        |  curve (:class:`Curve`): The curve that the point lies on.
    """

    def __init__(self, x: int, y: int, curve: Curve) -> None:
        r"""Initialize a point on an elliptic curve.

        The x and y parameters must satisfy the equation :math:`y^2 \equiv x^3 + ax + b \pmod{p}`,
        where a, b, and p are attributes of the curve parameter.

        Args:
            |  x (int): The x coordinate of the point.
            |  y (int): The y coordinate of the point.
            |  curve (:class:`Curve`): The curve that the point lies on.
        """

        # Reduce numbers before computation to avoid errors and limit computations.
        if curve is not None:
            x = x % curve.p
            y = y % curve.p

        if not (x == 0 and y == 0 and curve is None) and not curve.is_point_on_curve(
            (x, y)
        ):
            raise ValueError(
                f"coordinates are not on curve <{curve}>\n\tx={x:x}\n\ty={y:x}"
            )
        else:
            self.x = x
            self.y = y
            self.curve = curve

    def __str__(self) -> str:
        if self._is_identity():
            return "<POINT AT INFINITY>"
        else:
            return f"X: 0x{self.x:x}\nY: 0x{self.y:x}\n(On curve <{self.curve}>)"

    def __repr__(self) -> str:
        return self.__str__()

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, Point):
            raise TypeError(f"Cannot compare Point to {type(other)}")

        return self.x == other.x and self.y == other.y and self.curve is other.curve

    def __add__(self, other: Point) -> Point:
        """Add two points on the same elliptic curve.

        Args:
            | self (:class:`Point`): a point :math:`P` on the curve
            | other (:class:`Point`): a point :math:`Q` on the curve

        Returns:
            :class:`Point`: A point :math:`R` such that :math:`R = P + Q`
        """
        if not isinstance(other, Point):
            raise TypeError(f"Cannot add {type(other)} to Point")

        if self._is_identity():
            return other
        elif other._is_identity():
            return self
        elif self.curve is not other.curve:
            raise CurveMismatchError(self.curve, other.curve)
        elif self == -other:
            return self._identity_element()
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
                str(self.curve.gy),
            )
            return Point(int(x), int(y), self.curve)

    def __sub__(self, other: Point) -> Point:
        """Subtract two points on the same elliptic curve.

        Args:
            | self (:class:`Point`): a point :math:`P` on the curve
            | other (:class:`Point`): a point :math:`Q` on the curve

        Returns:
            :class:`Point`: A point :math:`R` such that :math:`R = P - Q`
        """
        if not isinstance(other, Point):
            raise TypeError(f"Cannot subtract {type(other)} from Point")

        if self == other:
            return self._identity_element()
        elif other._is_identity():
            return self

        negative = Point(other.x, -other.y % other.curve.p, other.curve)
        return self.__add__(negative)

    def __mul__(self, scalar: int) -> Point:
        r"""Multiply a :class:`Point` on an elliptic curve by an integer.

        Args:
            | self (:class:`Point`): a point :math:`P` on the curve
            | scalar (int): an integer :math:`d \in \mathbb{Z_q}` where :math:`q` is the order of
                the curve that :math:`P` is on

        Returns:
            :class:`Point`: A point :math:`R` such that :math:`R = P * d`
        """
        if not isinstance(scalar, int):
            raise TypeError(f"Cannot multiply Point by {type(scalar)}")

        if scalar == 0:
            return self._identity_element()

        x, y = curvemath.mul(
            str(self.x),
            str(self.y),
            str(abs(scalar)),
            str(self.curve.p),
            str(self.curve.a),
            str(self.curve.b),
            str(self.curve.q),
            str(self.curve.gx),
            str(self.curve.gy),
        )
        x = int(x)
        y = int(y)
        if x == 0 and y == 0:
            return self._identity_element()
        return Point(x, y, self.curve) if scalar > 0 else -Point(x, y, self.curve)

    def __rmul__(self, scalar: int) -> Point:
        r"""Multiply a :class:`Point` on an elliptic curve by an integer.

        Args:
            | self (:class:`Point`): a point :math:`P` on the curve
            | scalar (int): an integer :math:`d \in \mathbb{Z_q}` where :math:`q` is the order of
                the curve that :math:`P` is on

        Returns:
            :class:`Point`: A point :math:`R` such that :math:`R = d * P`
        """
        return self.__mul__(scalar)

    def __neg__(self) -> Point:
        """Return the negation of a :class:`Point` i.e. the points reflection over the x-axis.

        Args:
            | self (:class:`Point`): a point :math:`P` on the curve

        Returns:
            :class:`Point`: A point :math:`R = (P_x, -P_y)`
        """
        if self._is_identity():
            return self

        return Point(self.x, -self.y % self.curve.p, self.curve)

    def _is_identity(self) -> bool:
        return self.x == 0 and self.y == 0 and self.curve is None  # type: ignore

    @staticmethod
    def _identity_element() -> Point:
        return Point(0, 0, curve=None)  # type: ignore
