from random import choice, randint
from unittest import TestCase

from . import CURVES
from fastecdsa.curve import P192, P224, P256, P384, P521, secp256k1
from fastecdsa.point import Point


class TestPrimeFieldCurve(TestCase):
    """NIST P curve tests taken from https://www.nsa.gov/ia/_files/nist-routines.pdf"""

    def test_P192_arith(self):
        S = Point(
            0xD458E7D127AE671B0C330266D246769353A012073E97ACF8,
            0x325930500D851F336BDDC050CF7FB11B5673A1645086DF3B,
            curve=P192,
        )
        d = 0xA78A236D60BAEC0C5DD41B33A542463A8255391AF64C74EE
        expected = Point(
            0x1FAEE4205A4F669D2D0A8F25E3BCEC9A62A6952965BF6D31,
            0x5FF2CDFA508A2581892367087C696F179E7A4D7E8260FB06,
            curve=P192,
        )
        R = d * S
        self.assertEqual(R, expected)

    def test_P224_arith(self):
        S = Point(
            0x6ECA814BA59A930843DC814EDD6C97DA95518DF3C6FDF16E9A10BB5B,
            0xEF4B497F0963BC8B6AEC0CA0F259B89CD80994147E05DC6B64D7BF22,
            curve=P224,
        )
        d = 0xA78CCC30EACA0FCC8E36B2DD6FBB03DF06D37F52711E6363AAF1D73B
        expected = Point(
            0x96A7625E92A8D72BFF1113ABDB95777E736A14C6FDAACC392702BCA4,
            0x0F8E5702942A3C5E13CD2FD5801915258B43DFADC70D15DBADA3ED10,
            curve=P224,
        )
        R = d * S
        self.assertEqual(R, expected)

    def test_P256_arith(self):
        S = Point(
            0xDE2444BEBC8D36E682EDD27E0F271508617519B3221A8FA0B77CAB3989DA97C9,
            0xC093AE7FF36E5380FC01A5AAD1E66659702DE80F53CEC576B6350B243042A256,
            curve=P256,
        )
        d = 0xC51E4753AFDEC1E6B6C6A5B992F43F8DD0C7A8933072708B6522468B2FFB06FD
        expected = Point(
            0x51D08D5F2D4278882946D88D83C97D11E62BECC3CFC18BEDACC89BA34EECA03F,
            0x75EE68EB8BF626AA5B673AB51F6E744E06F8FCF8A6C0CF3035BECA956A7B41D5,
            curve=P256,
        )
        R = d * S
        self.assertEqual(R, expected)

    def test_P384_arith(self):
        S = Point(
            int(
                "fba203b81bbd23f2b3be971cc23997e1ae4d89e69cb6f92385dda82768ada415ebab4167459da98e6"
                "2b1332d1e73cb0e",
                16,
            ),
            int(
                "5ffedbaefdeba603e7923e06cdb5d0c65b22301429293376d5c6944e3fa6259f162b4788de6987fd5"
                "9aed5e4b5285e45",
                16,
            ),
            curve=P384,
        )
        d = int(
            "a4ebcae5a665983493ab3e626085a24c104311a761b5a8fdac052ed1f111a5c44f76f45659d2d111a"
            "61b5fdd97583480",
            16,
        )
        expected = Point(
            int(
                "e4f77e7ffeb7f0958910e3a680d677a477191df166160ff7ef6bb5261f791aa7b45e3e653d151b95d"
                "ad3d93ca0290ef2",
                16,
            ),
            int(
                "ac7dee41d8c5f4a7d5836960a773cfc1376289d3373f8cf7417b0c6207ac32e913856612fc9ff2e35"
                "7eb2ee05cf9667f",
                16,
            ),
            curve=P384,
        )
        R = d * S
        self.assertEqual(R, expected)

    def test_P521_arith(self):
        S = Point(
            int(
                "000001d5c693f66c08ed03ad0f031f937443458f601fd098d3d0227b4bf62873af50740b0bb84aa15"
                "7fc847bcf8dc16a8b2b8bfd8e2d0a7d39af04b089930ef6dad5c1b4",
                16,
            ),
            int(
                "00000144b7770963c63a39248865ff36b074151eac33549b224af5c8664c54012b818ed037b2b7c1a"
                "63ac89ebaa11e07db89fcee5b556e49764ee3fa66ea7ae61ac01823",
                16,
            ),
            curve=P521,
        )
        d = int(
            "000001eb7f81785c9629f136a7e8f8c674957109735554111a2a866fa5a166699419bfa9936c78b62"
            "653964df0d6da940a695c7294d41b2d6600de6dfcf0edcfc89fdcb1",
            16,
        )
        expected = Point(
            int(
                "00000091b15d09d0ca0353f8f96b93cdb13497b0a4bb582ae9ebefa35eee61bf7b7d041b8ec34c6c0"
                "0c0c0671c4ae063318fb75be87af4fe859608c95f0ab4774f8c95bb",
                16,
            ),
            int(
                "00000130f8f8b5e1abb4dd94f6baaf654a2d5810411e77b7423965e0c7fd79ec1ae563c207bd255ee"
                "9828eb7a03fed565240d2cc80ddd2cecbb2eb50f0951f75ad87977f",
                16,
            ),
            curve=P521,
        )
        R = d * S
        self.assertEqual(R, expected)

    def test_secp256k1_arith(self):
        # http://crypto.stackexchange.com/a/787/17884
        m = 0xAA5E28D6A97A2479A65527F7290311A3624D4CC0FA1578598EE3C2613BF99522
        expected = Point(
            0x34F9460F0E4F08393D192B3C5133A6BA099AA0AD9FD54EBCCFACDFA239FF49C6,
            0x0B71EA9BD730FD8923F6D25A7A91E7DD7728A960686CB5A901BB419E0F2CA232,
            curve=secp256k1,
        )
        R = m * secp256k1.G
        self.assertEqual(R, expected)

        m = 0x7E2B897B8CEBC6361663AD410835639826D590F393D90A9538881735256DFAE3
        expected = Point(
            0xD74BF844B0862475103D96A611CF2D898447E288D34B360BC885CB8CE7C00575,
            0x131C670D414C4546B88AC3FF664611B1C38CEB1C21D76369D7A7A0969D61D97D,
            curve=secp256k1,
        )
        R = m * secp256k1.G
        self.assertEqual(R, expected)

        m = 0x6461E6DF0FE7DFD05329F41BF771B86578143D4DD1F7866FB4CA7E97C5FA945D
        expected = Point(
            0xE8AECC370AEDD953483719A116711963CE201AC3EB21D3F3257BB48668C6A72F,
            0xC25CAF2F0EBA1DDB2F0F3F47866299EF907867B7D27E95B3873BF98397B24EE1,
            curve=secp256k1,
        )
        R = m * secp256k1.G
        self.assertEqual(R, expected)

        m = 0x376A3A2CDCD12581EFFF13EE4AD44C4044B8A0524C42422A7E1E181E4DEECCEC
        expected = Point(
            0x14890E61FCD4B0BD92E5B36C81372CA6FED471EF3AA60A3E415EE4FE987DABA1,
            0x297B858D9F752AB42D3BCA67EE0EB6DCD1C2B7B0DBE23397E66ADC272263F982,
            curve=secp256k1,
        )
        R = m * secp256k1.G
        self.assertEqual(R, expected)

        m = 0x1B22644A7BE026548810C378D0B2994EEFA6D2B9881803CB02CEFF865287D1B9
        expected = Point(
            0xF73C65EAD01C5126F28F442D087689BFA08E12763E0CEC1D35B01751FD735ED3,
            0xF449A8376906482A84ED01479BD18882B919C140D638307F0C0934BA12590BDE,
            curve=secp256k1,
        )
        R = m * secp256k1.G
        self.assertEqual(R, expected)

    def test_secp256k1_add_large(self):
        xs = 1
        xt = secp256k1.p + 1
        ys = 0x4218F20AE6C646B363DB68605822FB14264CA8D2587FDD6FBC750D587E76A7EE
        S = Point(xs, ys, curve=secp256k1)
        T = Point(xt, ys, curve=secp256k1)
        expected = Point(
            0xC7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF37FFFD03,
            0x4298C557A7DDCC570E8BF054C4CAD9E99F396B3CE19D50F1B91C9DF4BB00D333,
            curve=secp256k1,
        )
        R = S + T
        self.assertEqual(R, expected)

    def test_arbitrary_arithmetic(self):
        for _ in range(100):
            curve = choice(CURVES)
            a, b = randint(0, curve.q), randint(0, curve.q)
            c = (a + b) % curve.q
            P, Q = a * curve.G, b * curve.G
            R = c * curve.G
            pq_sum, qp_sum = P + Q, Q + P
            self.assertEqual(pq_sum, qp_sum)
            self.assertEqual(qp_sum, R)

    def test_point_at_infinity_arithmetic(self):
        for curve in CURVES:
            a = randint(0, curve.q)
            b = curve.q - a
            P, Q = a * curve.G, b * curve.G

            self.assertEqual((P + Q).x, 0)
            self.assertEqual((P + Q) + P, P)

    def test_mul_by_negative(self):
        for curve in CURVES:
            P = curve.G * (curve.q - 1)
            self.assertEqual(P.x, (-curve.G).x)
            self.assertEqual(P.y, (-curve.G).y)

            a = randint(0, curve.q)
            P = (curve.q - a) * curve.G
            Q = a * -curve.G

            self.assertEqual(P.x, Q.x)
            self.assertEqual(P.y, Q.y)
