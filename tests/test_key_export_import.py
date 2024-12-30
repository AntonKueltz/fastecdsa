from os import remove
from unittest import TestCase

from . import CURVES
from fastecdsa.encoding.pem import PEMEncoder
from fastecdsa.encoding.sec1 import SEC1Encoder
from fastecdsa.keys import (
    export_private_key,
    export_public_key,
    gen_keypair,
    gen_private_key,
    import_private_key,
    import_public_key,
)

TEST_FILE_PATH = "fastecdsa_test_key.pem"


class TestExportImport(TestCase):
    def test_export_import_private_key(self):
        for curve in CURVES:
            for encoder_class in (PEMEncoder,):
                encoder = encoder_class()
                expected = gen_private_key(curve)

                export_private_key(expected, curve, encoder, filepath=TEST_FILE_PATH)
                actual = import_private_key(TEST_FILE_PATH, encoder)

                self.assertEqual(expected, actual)
                remove(TEST_FILE_PATH)

    def test_export_import_public_key(self):
        for curve in CURVES:
            for encoder_class in (PEMEncoder, SEC1Encoder):
                encoder = encoder_class()
                _, expected = gen_keypair(curve)

                export_public_key(expected, encoder, filepath=TEST_FILE_PATH)
                actual = import_public_key(TEST_FILE_PATH, curve, encoder)

                self.assertEqual(expected, actual)
                remove(TEST_FILE_PATH)
