import unittest

import ecdsa


class ECDSA(unittest.TestCase):
    # https://www.nsa.gov/ia/_files/ecdsa.pdf
    def test_ecdsa_P256(self):
        d = 0x70a12c2db16845ed56ff68cfc21a472b3f04d7d6851bf6349f2d7d5b3452b38a
        msg = 'This is only a test message. It is 48 bytes long'
        expected = (
            0x7214bc9647160bbd39ff2f80533f5dc6ddd70ddf86bb815661e805d5d4e6f27c,
            0x7d1ff961980f961bdaa3233b6209f4013317d3e3f9e1493592dbeaa1af2bc367
        )
        got = ecdsa.sign(msg, d)
        self.assertTrue(got == expected)

if __name__ == '__main__':
    unittest.main()
