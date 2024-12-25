from unittest import TestCase

from ..curve import W25519
from ..point import Point


class TestTypeValidation(TestCase):
    def test_type_validation_add(self):
        with self.assertRaises(TypeError):
            _ = Point._identity_element() + 2  # type: ignore

        with self.assertRaises(TypeError):
            _ = W25519.G + 2  # type: ignore

        with self.assertRaises(TypeError):
            _ = 2 + Point._identity_element()  # type: ignore

        with self.assertRaises(TypeError):
            _ = 2 + W25519.G  # type: ignore

    def test_type_validation_sub(self):
        with self.assertRaises(TypeError):
            _ = Point._identity_element() - 2  # type: ignore

        with self.assertRaises(TypeError):
            _ = W25519.G - 2  # type: ignore

        with self.assertRaises(TypeError):
            _ = 2 - Point._identity_element()  # type: ignore

        with self.assertRaises(TypeError):
            _ = 2 - W25519.G  # type: ignore

    def test_type_validation_mul(self):
        with self.assertRaises(TypeError):
            _ = Point._identity_element() * 1.5  # type: ignore

        with self.assertRaises(TypeError):
            _ = W25519.G * 1.5  # type: ignore

        with self.assertRaises(TypeError):
            _ = 1.5 * Point._identity_element()  # type: ignore

        with self.assertRaises(TypeError):
            _ = 1.5 * W25519.G  # type: ignore

    def test_point_at_infinity(self):
        self.assertEqual(W25519.G * 0, Point._identity_element())
        self.assertEqual(W25519.G * W25519.q, Point._identity_element())
        self.assertEqual(W25519.G + Point._identity_element(), W25519.G)
        self.assertEqual(W25519.G - Point._identity_element(), W25519.G)
