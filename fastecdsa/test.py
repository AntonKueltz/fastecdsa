from hashlib import sha1, sha224, sha256, sha384, sha512
from random import choice, randint
import unittest

from .curve import P192, P224, P256, P384, P521, secp256k1, K163, K233, K283, K409, K571
from .ecdsa import sign, verify
from .util import RFC6979


class TestPrimeFieldCurve(unittest.TestCase):
    ''' cases taken from https://www.nsa.gov/ia/_files/nist-routines.pdf '''

    def test_P192_arith(self):
        S = (
            0xd458e7d127ae671b0c330266d246769353a012073e97acf8,
            0x325930500d851f336bddc050cf7fb11b5673a1645086df3b
        )
        d = 0xa78a236d60baec0c5dd41b33a542463a8255391af64c74ee
        expected = (
            0x1faee4205a4f669d2d0a8f25e3bcec9a62a6952965bf6d31,
            0x5ff2cdfa508a2581892367087c696f179e7a4d7e8260fb06
        )
        R = P192.point_mul(S, d)
        self.assertTrue(R[0] == expected[0])
        self.assertTrue(R[1] == expected[1])

    def test_P224_arith(self):
        S = (
            0x6eca814ba59a930843dc814edd6c97da95518df3c6fdf16e9a10bb5b,
            0xef4b497f0963bc8b6aec0ca0f259b89cd80994147e05dc6b64d7bf22
        )
        d = 0xa78ccc30eaca0fcc8e36b2dd6fbb03df06d37f52711e6363aaf1d73b
        expected = (
            0x96a7625e92a8d72bff1113abdb95777e736a14c6fdaacc392702bca4,
            0x0f8e5702942a3c5e13cd2fd5801915258b43dfadc70d15dbada3ed10
        )
        R = P224.point_mul(S, d)
        self.assertTrue(R[0] == expected[0])
        self.assertTrue(R[1] == expected[1])

    def test_P256_arith(self):
        S = (
            0xde2444bebc8d36e682edd27e0f271508617519b3221a8fa0b77cab3989da97c9,
            0xc093ae7ff36e5380fc01a5aad1e66659702de80f53cec576b6350b243042a256
        )
        d = 0xc51e4753afdec1e6b6c6a5b992f43f8dd0c7a8933072708b6522468b2ffb06fd
        expected = (
            0x51d08d5f2d4278882946d88d83c97d11e62becc3cfc18bedacc89ba34eeca03f,
            0x75ee68eb8bf626aa5b673ab51f6e744e06f8fcf8a6c0cf3035beca956a7b41d5
        )
        R = P256.point_mul(S, d)
        self.assertTrue(R[0] == expected[0])
        self.assertTrue(R[1] == expected[1])

    def test_P384_arith(self):
        S = (
            int('fba203b81bbd23f2b3be971cc23997e1ae4d89e69cb6f92385dda82768ada415ebab4167459da98e6'
                '2b1332d1e73cb0e', 16),
            int('5ffedbaefdeba603e7923e06cdb5d0c65b22301429293376d5c6944e3fa6259f162b4788de6987fd5'
                '9aed5e4b5285e45', 16)
        )
        d = int('a4ebcae5a665983493ab3e626085a24c104311a761b5a8fdac052ed1f111a5c44f76f45659d2d111a'
                '61b5fdd97583480', 16)
        expected = (
            int('e4f77e7ffeb7f0958910e3a680d677a477191df166160ff7ef6bb5261f791aa7b45e3e653d151b95d'
                'ad3d93ca0290ef2', 16),
            int('ac7dee41d8c5f4a7d5836960a773cfc1376289d3373f8cf7417b0c6207ac32e913856612fc9ff2e35'
                '7eb2ee05cf9667f', 16)
        )
        R = P384.point_mul(S, d)
        self.assertTrue(R[0] == expected[0])
        self.assertTrue(R[1] == expected[1])

    def test_P521_arith(self):
        S = (
            int('000001d5c693f66c08ed03ad0f031f937443458f601fd098d3d0227b4bf62873af50740b0bb84aa15'
                '7fc847bcf8dc16a8b2b8bfd8e2d0a7d39af04b089930ef6dad5c1b4', 16),
            int('00000144b7770963c63a39248865ff36b074151eac33549b224af5c8664c54012b818ed037b2b7c1a'
                '63ac89ebaa11e07db89fcee5b556e49764ee3fa66ea7ae61ac01823', 16)
        )
        d = int('000001eb7f81785c9629f136a7e8f8c674957109735554111a2a866fa5a166699419bfa9936c78b62'
                '653964df0d6da940a695c7294d41b2d6600de6dfcf0edcfc89fdcb1', 16)
        expected = (
            int('00000091b15d09d0ca0353f8f96b93cdb13497b0a4bb582ae9ebefa35eee61bf7b7d041b8ec34c6c0'
                '0c0c0671c4ae063318fb75be87af4fe859608c95f0ab4774f8c95bb', 16),
            int('00000130f8f8b5e1abb4dd94f6baaf654a2d5810411e77b7423965e0c7fd79ec1ae563c207bd255ee'
                '9828eb7a03fed565240d2cc80ddd2cecbb2eb50f0951f75ad87977f', 16)
        )
        R = P521.point_mul(S, d)
        self.assertTrue(R[0] == expected[0])
        self.assertTrue(R[1] == expected[1])

    def test_secp256k1_arith(self):
        # http://crypto.stackexchange.com/a/787/17884
        m = 0xAA5E28D6A97A2479A65527F7290311A3624D4CC0FA1578598EE3C2613BF99522
        expected = (
            0x34F9460F0E4F08393D192B3C5133A6BA099AA0AD9FD54EBCCFACDFA239FF49C6,
            0x0B71EA9BD730FD8923F6D25A7A91E7DD7728A960686CB5A901BB419E0F2CA232
        )
        (X, Y) = secp256k1.point_mul(secp256k1.G, m)
        self.assertTrue(X == expected[0])
        self.assertTrue(Y == expected[1])

        m = 0x7E2B897B8CEBC6361663AD410835639826D590F393D90A9538881735256DFAE3
        expected = (
            0xD74BF844B0862475103D96A611CF2D898447E288D34B360BC885CB8CE7C00575,
            0x131C670D414C4546B88AC3FF664611B1C38CEB1C21D76369D7A7A0969D61D97D
        )
        (X, Y) = secp256k1.point_mul(secp256k1.G, m)
        self.assertTrue(X == expected[0])
        self.assertTrue(Y == expected[1])

        m = 0x6461E6DF0FE7DFD05329F41BF771B86578143D4DD1F7866FB4CA7E97C5FA945D
        expected = (
            0xE8AECC370AEDD953483719A116711963CE201AC3EB21D3F3257BB48668C6A72F,
            0xC25CAF2F0EBA1DDB2F0F3F47866299EF907867B7D27E95B3873BF98397B24EE1
        )
        (X, Y) = secp256k1.point_mul(secp256k1.G, m)
        self.assertTrue(X == expected[0])
        self.assertTrue(Y == expected[1])

        m = 0x376A3A2CDCD12581EFFF13EE4AD44C4044B8A0524C42422A7E1E181E4DEECCEC
        expected = (
            0x14890E61FCD4B0BD92E5B36C81372CA6FED471EF3AA60A3E415EE4FE987DABA1,
            0x297B858D9F752AB42D3BCA67EE0EB6DCD1C2B7B0DBE23397E66ADC272263F982
        )
        (X, Y) = secp256k1.point_mul(secp256k1.G, m)
        self.assertTrue(X == expected[0])
        self.assertTrue(Y == expected[1])

        m = 0x1B22644A7BE026548810C378D0B2994EEFA6D2B9881803CB02CEFF865287D1B9
        expected = (
            0xF73C65EAD01C5126F28F442D087689BFA08E12763E0CEC1D35B01751FD735ED3,
            0xF449A8376906482A84ED01479BD18882B919C140D638307F0C0934BA12590BDE
        )
        (X, Y) = secp256k1.point_mul(secp256k1.G, m)
        self.assertTrue(X == expected[0])
        self.assertTrue(Y == expected[1])

    def test_arbitrary_arithmetic(self):
        curves = [P192, P224, P256, P384, P521, secp256k1]

        for _ in range(100):
            curve = choice(curves)
            a, b = randint(0, curve.q), randint(0, curve.q)
            c = (a + b) % curve.q
            P, Q = curve.point_mul(curve.G, a), curve.point_mul(curve.G, b)
            R = curve.point_mul(curve.G, c)
            pq_sum, qp_sum = curve.point_add(P, Q), curve.point_add(Q, P)
            self.assertTrue(pq_sum == qp_sum)
            self.assertTrue(qp_sum == R)


class TestNonceGeneration(unittest.TestCase):
    def test_rfc_6979(self):
        msg = 'sample'
        x = 0x09A4D6792295A7F730FC3F2B49CBC0F62E862272F
        q = 0x4000000000000000000020108A2E0CC0D99F8A5EF

        expected = 0x09744429FA741D12DE2BE8316E35E84DB9E5DF1CD
        nonce = RFC6979(msg, x, q, sha1).gen_nonce()
        self.assertTrue(nonce == expected)

        expected = 0x323E7B28BFD64E6082F5B12110AA87BC0D6A6E159
        nonce = RFC6979(msg, x, q, sha224).gen_nonce()
        self.assertTrue(nonce == expected)

        expected = 0x23AF4074C90A02B3FE61D286D5C87F425E6BDD81B
        nonce = RFC6979(msg, x, q, sha256).gen_nonce()
        self.assertTrue(nonce == expected)

        expected = 0x2132ABE0ED518487D3E4FA7FD24F8BED1F29CCFCE
        nonce = RFC6979(msg, x, q, sha384).gen_nonce()
        self.assertTrue(nonce == expected)

        expected = 0x00BBCC2F39939388FDFE841892537EC7B1FF33AA3
        nonce = RFC6979(msg, x, q, sha512).gen_nonce()
        self.assertTrue(nonce == expected)


class TestPrimeFieldECDSA(unittest.TestCase):
    ''' case taken from http://tools.ietf.org/html/rfc6979#appendix-A.2.5 '''
    def test_ecdsa_P256_SHA1_sign(self):
        d = 0xC9AFA9D845BA75166B5C215767B1D6934E50C3DB36E89B127B8A622B120F6721
        expected = (
            0x61340C88C3AAEBEB4F6D667F672CA9759A6CCAA9FA8811313039EE4A35471D32,
            0x6D7F147DAC089441BB2E2FE8F7A3FA264B9C475098FDCF6E00D7C996E1B8B7EB
        )
        sig = sign('sample', d, curve=P256, hashfunc=sha1)
        self.assertTrue(sig == expected)

        Q = P256.point_mul(P256.G, d)
        self.assertTrue(verify(sig, 'sample', Q, curve=P256, hashfunc=sha1))

    ''' case taken from http://tools.ietf.org/html/rfc6979#appendix-A.2.5 '''
    def test_ecdsa_P256_SHA224_sign(self):
        d = 0xC9AFA9D845BA75166B5C215767B1D6934E50C3DB36E89B127B8A622B120F6721
        expected = (
            0x53B2FFF5D1752B2C689DF257C04C40A587FABABB3F6FC2702F1343AF7CA9AA3F,
            0xB9AFB64FDC03DC1A131C7D2386D11E349F070AA432A4ACC918BEA988BF75C74C
        )
        sig = sign('sample', d, curve=P256, hashfunc=sha224)
        self.assertTrue(sig == expected)

        Q = P256.point_mul(P256.G, d)
        self.assertTrue(verify(sig, 'sample', Q, curve=P256, hashfunc=sha224))

    ''' case taken from http://tools.ietf.org/html/rfc6979#appendix-A.2.5 '''
    def test_ecdsa_P256_SHA2_sign(self):
        d = 0xC9AFA9D845BA75166B5C215767B1D6934E50C3DB36E89B127B8A622B120F6721
        expected = (
            0xEFD48B2AACB6A8FD1140DD9CD45E81D69D2C877B56AAF991C34D0EA84EAF3716,
            0xF7CB1C942D657C41D436C7A1B6E29F65F3E900DBB9AFF4064DC4AB2F843ACDA8
        )
        sig = sign('sample', d, curve=P256, hashfunc=sha256)
        self.assertTrue(sig == expected)

        Q = P256.point_mul(P256.G, d)
        self.assertTrue(verify(sig, 'sample', Q, curve=P256, hashfunc=sha256))

    ''' case taken from http://tools.ietf.org/html/rfc6979#appendix-A.2.5 '''
    def test_ecdsa_P256_SHA384_sign(self):
        d = 0xC9AFA9D845BA75166B5C215767B1D6934E50C3DB36E89B127B8A622B120F6721
        expected = (
            0x0EAFEA039B20E9B42309FB1D89E213057CBF973DC0CFC8F129EDDDC800EF7719,
            0x4861F0491E6998B9455193E34E7B0D284DDD7149A74B95B9261F13ABDE940954
        )
        sig = sign('sample', d, curve=P256, hashfunc=sha384)
        self.assertTrue(sig == expected)

        Q = P256.point_mul(P256.G, d)
        self.assertTrue(verify(sig, 'sample', Q, curve=P256, hashfunc=sha384))

    ''' case taken from http://tools.ietf.org/html/rfc6979#appendix-A.2.5 '''
    def test_ecdsa_P256_SHA512_sign(self):
        d = 0xC9AFA9D845BA75166B5C215767B1D6934E50C3DB36E89B127B8A622B120F6721
        expected = (
            0x8496A60B5E9B47C825488827E0495B0E3FA109EC4568FD3F8D1097678EB97F00,
            0x2362AB1ADBE2B8ADF9CB9EDAB740EA6049C028114F2460F96554F61FAE3302FE
        )
        sig = sign('sample', d, curve=P256, hashfunc=sha512)
        self.assertTrue(sig == expected)

        Q = P256.point_mul(P256.G, d)
        self.assertTrue(verify(sig, 'sample', Q, curve=P256, hashfunc=sha512))

    ''' case taken from https://www.nsa.gov/ia/_files/ecdsa.pdf '''
    def test_ecdsa_P256_verify(self):
        Q = (
            0x8101ece47464a6ead70cf69a6e2bd3d88691a3262d22cba4f7635eaff26680a8,
            0xd8a12ba61d599235f67d9cb4d58f1783d3ca43e78f0a5abaa624079936c0c3a9
        )
        msg = 'This is only a test message. It is 48 bytes long'
        sig = (
            0x7214bc9647160bbd39ff2f80533f5dc6ddd70ddf86bb815661e805d5d4e6f27c,
            0x7d1ff961980f961bdaa3233b6209f4013317d3e3f9e1493592dbeaa1af2bc367
        )
        self.assertTrue(verify(sig, msg, Q, curve=P256, hashfunc=sha256))

        sig = (
            0x7214bc9647160bbd39ff2f80533f5dc6ddd70ddf86bb815661e805d5d4e6fbad,
            0x7d1ff961980f961bdaa3233b6209f4013317d3e3f9e1493592dbeaa1af2bc367
        )
        self.assertFalse(verify(sig, msg, Q, curve=P256, hashfunc=sha256))


class TestBinaryFieldECDSA(unittest.TestCase):
    ''' cases taken from https://tools.ietf.org/html/rfc6979#appendix-A.2.8 '''
    def test_ecdsa_K163_SHA1_sign(self):
        d = 0x09A4D6792295A7F730FC3F2B49CBC0F62E862272F
        expected = (
            0x30C45B80BA0E1406C4EFBBB7000D6DE4FA465D505,
            0x38D87DF89493522FC4CD7DE1553BD9DBBA2123011
        )
        sig = sign('sample', d, curve=K163, hashfunc=sha1)
        self.assertTrue(sig == expected)

        Q = (
            0x79AEE090DB05EC252D5CB4452F356BE198A4FF96F,
            0x782E29634DDC9A31EF40386E896BAA18B53AFA5A3
        )
        self.assertTrue(verify(sig, 'sample', Q, curve=K163, hashfunc=sha1))

    def test_ecdsa_K163_SHA224_sign(self):
        d = 0x09A4D6792295A7F730FC3F2B49CBC0F62E862272F
        expected = (
            0x38A2749F7EA13BD5DA0C76C842F512D5A65FFAF32,
            0x064F841F70112B793FD773F5606BFA5AC2A04C1E8
        )
        sig = sign('sample', d, curve=K163, hashfunc=sha224)
        self.assertTrue(sig == expected)

        Q = (
            0x79AEE090DB05EC252D5CB4452F356BE198A4FF96F,
            0x782E29634DDC9A31EF40386E896BAA18B53AFA5A3
        )
        self.assertTrue(verify(sig, 'sample', Q, curve=K163, hashfunc=sha224))

    def test_ecdsa_K163_SHA256_sign(self):
        d = 0x09A4D6792295A7F730FC3F2B49CBC0F62E862272F
        expected = (
            0x113A63990598A3828C407C0F4D2438D990DF99A7F,
            0x1313A2E03F5412DDB296A22E2C455335545672D9F
        )
        sig = sign('sample', d, curve=K163, hashfunc=sha256)
        self.assertTrue(sig == expected)

        Q = (
            0x79AEE090DB05EC252D5CB4452F356BE198A4FF96F,
            0x782E29634DDC9A31EF40386E896BAA18B53AFA5A3
        )
        self.assertTrue(verify(sig, 'sample', Q, curve=K163, hashfunc=sha256))

    def test_ecdsa_K163_SHA384_sign(self):
        d = 0x09A4D6792295A7F730FC3F2B49CBC0F62E862272F
        expected = (
            0x34D4DE955871BB84FEA4E7D068BA5E9A11BD8B6C4,
            0x2BAAF4D4FD57F175C405A2F39F9755D9045C820BD
        )
        sig = sign('sample', d, curve=K163, hashfunc=sha384)
        self.assertTrue(sig == expected)

        Q = (
            0x79AEE090DB05EC252D5CB4452F356BE198A4FF96F,
            0x782E29634DDC9A31EF40386E896BAA18B53AFA5A3
        )
        self.assertTrue(verify(sig, 'sample', Q, curve=K163, hashfunc=sha384))

    def test_ecdsa_K163_SHA512_sign(self):
        d = 0x09A4D6792295A7F730FC3F2B49CBC0F62E862272F
        expected = (
            0x38E487F218D696A7323B891F0CCF055D895B77ADC,
            0x0972D7721093F9B3835A5EB7F0442FA8DCAA873C4
        )
        sig = sign('sample', d, curve=K163, hashfunc=sha512)
        self.assertTrue(sig == expected)

        Q = (
            0x79AEE090DB05EC252D5CB4452F356BE198A4FF96F,
            0x782E29634DDC9A31EF40386E896BAA18B53AFA5A3
        )
        self.assertTrue(verify(sig, 'sample', Q, curve=K163, hashfunc=sha512))

    def test_ecdsa_K233_SHA1_sign(self):
        d = 0x103B2142BDC2A3C3B55080D09DF1808F79336DA2399F5CA7171D1BE9B0
        expected = (
            0x5474541C988A9A1F73899F55EF28963DFFBBF0C2B1A1EE787C6A76C6A4,
            0x46301F9EC6624257BFC70D72186F17898EDBD0A3522560A88DD1B7D45A
        )
        sig = sign('sample', d, curve=K233, hashfunc=sha1)
        self.assertTrue(sig == expected)

        Q = (
            0x0682886F36C68473C1A221720C2B12B9BE13458BA907E1C4736595779F2,
            0x1B20639B41BE0927090999B7817A3B3928D20503A39546044EC13A10309
        )
        self.assertTrue(verify(sig, 'sample', Q, curve=K233, hashfunc=sha1))

    def test_ecdsa_K283_SHA1_sign(self):
        d = 0x06A0777356E87B89BA1ED3A3D845357BE332173C8F7A65BDC7DB4FAB3C4CC79ACC8194E
        expected = (
            0x1B66D1E33FBDB6E107A69B610995C93C744CEBAEAF623CB42737C27D60188BD1D045A68,
            0x02E45B62C9C258643532FD536594B46C63B063946494F95DAFF8759FD552502324295C5
        )
        sig = sign('sample', d, curve=K283, hashfunc=sha1)
        self.assertTrue(sig == expected)

        Q = (
            0x25330D0A651D5A20DC6389BC02345117725640AEC3C126612CE444EDD19649BDECC03D6,
            0x505BD60A4B67182474EC4D1C668A73140F70504A68F39EFCD972487E9530E0508A76193
        )
        self.assertTrue(verify(sig, 'sample', Q, curve=K283, hashfunc=sha1))

    def test_ecdsa_K409_SHA1_sign(self):
        d = int('29C16768F01D1B8A89FDA85E2EFD73A09558B92A178A2931F359E4D70AD853E5'
                '69CDAF16DAA569758FB4E73089E4525D8BBFCF', 16)
        expected = (
            int('7192EE99EC7AFE23E02CB1F9850D1ECE620475EDA6B65D04984029408EC1E5A6'
                '476BC940D81F218FC31D979814CAC6E78340FA', 16),
            int('1DE75DE97CBE740FC79A6B5B22BC2B7832C687E6960F0B8173D5D8BE2A75AC6C'
                'A43438BAF69C669CE6D64E0FB93BC5854E0F81', 16)
        )
        sig = sign('sample', d, curve=K409, hashfunc=sha1)
        self.assertTrue(sig == expected)

        Q = (
            int('0CF923F523FE34A6E863D8BA45FB1FE6D784C8F219C414EEF4DB8362DBBD3CA7'
                '1AEB28F568668D5D7A0093E2B84F6FAD759DB42', 16),
            int('13B1C374D5132978A1B1123EBBE9A5C54D1A9D56B09AFDB4ADE93CCD7C4D332E'
                '2916F7D4B9D18578EE3C2E2DE4D2ECE0DE63549', 16)
        )
        self.assertTrue(verify(sig, 'sample', Q, curve=K409, hashfunc=sha1))

    def test_ecdsa_K571_SHA1_sign(self):
        d = int('0C16F58550D824ED7B95569D4445375D3A490BC7E0194C41A39DEB732C29396C'
                'DF1D66DE02DD1460A816606F3BEC0F32202C7BD18A32D87506466AA92032F131'
                '4ED7B19762B0D22', 16)
        expected = (
            int('0767913F96C82E38B7146A505938B79EC07E9AA3214377651BE968B52C039D3E'
                '4837B4A2DE26C481C4E1DE96F4D9DE63845D9B32E26D0D332725678E3CE57F66'
                '8A5E3108FB6CEA5', 16),
            int('109F89F55FA39FF465E40EBCF869A9B1DB425AEA53AB4ECBCE3C310572F79315'
                'F5D4891461372A0C36E63871BEDDBB3BA2042C6410B67311F1A185589FF4C987'
                'DBA02F9D992B9DF', 16)
        )
        sig = sign('sample', d, curve=K571, hashfunc=sha1)
        self.assertTrue(sig == expected)

        Q = (
            int('6CFB0DF7541CDD4C41EF319EA88E849EFC8605D97779148082EC991C463ED323'
                '19596F9FDF4779C17CAF20EFD9BEB57E9F4ED55BFC52A2FA15CA23BC62B7BF01'
                '9DB59793DD77318', 16),
            int('1CFC91102F7759A561BD8D5B51AAAEEC7F40E659D67870361990D6DE29F6B4F7'
                'E18AE13BDE5EA5C1F77B23D676F44050C9DBFCCDD7B3756328DDA059779AAE84'
                '46FC5158A75C227', 16)
        )
        self.assertTrue(verify(sig, 'sample', Q, curve=K571, hashfunc=sha1))

if __name__ == '__main__':
    unittest.main()
