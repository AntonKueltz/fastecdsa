from abc import ABC, abstractmethod
from typing import Tuple

from ..curve import Curve
from ..point import Point
from ..rust import Curve as RustCurve, Point as RustPoint


class KeyEncoder(ABC):
    """Base class that any encoding class for EC keys should derive from."""

    @abstractmethod
    def encode_public_key(self, Q: Point | RustPoint) -> bytes:
        pass

    @abstractmethod
    def encode_private_key(self, d: int, curve: Curve | RustCurve) -> bytes:
        pass

    @abstractmethod
    def decode_public_key(
        self, key: bytes, curve: Curve | RustCurve
    ) -> Point | RustPoint:
        pass

    @abstractmethod
    def decode_private_key(self, key: bytes) -> int:
        pass


class SigEncoder(ABC):
    """Base class that any encoding class for EC signatures should derive from.

    All overriding methods should be static.
    """

    @staticmethod
    @abstractmethod
    def encode_signature(r: int, s: int) -> bytes:
        pass

    @staticmethod
    @abstractmethod
    def decode_signature(binary_data: bytes) -> Tuple[int, int]:
        pass
