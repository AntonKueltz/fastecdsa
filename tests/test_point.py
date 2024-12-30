from unittest import TestCase

from fastecdsa.curve import P256, W25519
from fastecdsa.point import CurveMismatchError, Point


class TestPoint(TestCase):
    def test_init_invalid_coordinates_point(self):
        with self.assertRaises(ValueError):
            Point(0, 1, P256)

    def test_add_different_curves(self):
        with self.assertRaises(CurveMismatchError):
            P256.G + W25519.G

    def test_str_reprs(self):
        expected = (
            "X: 0x6b17d1f2e12c4247f8bce6e563a440f277037d812deb33a0f4a13945d898c296\n"
            "Y: 0x4fe342e2fe1a7f9b8ee7eb4a7c0f9e162bce33576b315ececbb6406837bf51f5\n"
            "(On curve <P256>)"
        )

        self.assertEqual(expected, str(P256.G))
        self.assertEqual(expected, repr(P256.G))

        self.assertEqual("<POINT AT INFINITY>", str(Point._identity_element()))
        self.assertEqual("<POINT AT INFINITY>", repr(Point._identity_element()))

    def test_eq(self):
        self.assertTrue(P256.G == P256.G)
        self.assertFalse(P256.G == W25519.G)

        with self.assertRaises(TypeError):
            P256.G == 2

    def test_sub(self):
        value = P256.G - P256.G

        self.assertTrue(value._is_identity())

    def test_neg(self):
        value = Point._identity_element()

        self.assertTrue((-value)._is_identity())


class TestPointTypeValidation(TestCase):
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
