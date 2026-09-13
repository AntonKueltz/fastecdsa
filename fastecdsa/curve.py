from fastecdsa.rust import Curve

brainpoolP160r1 = Curve.brainpoolp160r1()
brainpoolP192r1 = Curve.brainpoolp192r1()
brainpoolP224r1 = Curve.brainpoolp224r1()
brainpoolP256r1 = Curve.brainpoolp256r1()
brainpoolP320r1 = Curve.brainpoolp320r1()
brainpoolP384r1 = Curve.brainpoolp384r1()
brainpoolP512r1 = Curve.brainpoolp512r1()
P192 = Curve.p192()
P224 = Curve.p224()
P256 = Curve.p256()
P384 = Curve.p384()
P521 = Curve.p521()
secp192k1 = Curve.secp192k1()
secp224k1 = Curve.secp224k1()
secp256k1 = Curve.secp256k1()

CURVES = [
    P192,
    P224,
    P256,
    P384,
    P521,
    secp192k1,
    secp224k1,
    secp256k1,
    brainpoolP160r1,
    brainpoolP192r1,
    brainpoolP224r1,
    brainpoolP256r1,
    brainpoolP320r1,
    brainpoolP384r1,
    brainpoolP512r1,
]

oid_lookup = {c.oid: c for c in CURVES}


def get_curve_by_oid(oid: bytes) -> Curve | None:
    return oid_lookup.get(oid)
