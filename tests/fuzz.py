from random import choice, randint

from fastecdsa.curve import CURVES
from fastecdsa.eddsa import Ed25519Point, Ed448Point, Q25519, Q448

q_to_g = {c.q: c.G for c in CURVES}
q_to_g[Q25519] = Ed25519Point.g()
q_to_g[Q448] = Ed448Point.g()

qs = list(q_to_g.keys())

for q, G in q_to_g.items():
    assert 0 * G == q * G
    assert (q - 1) * G == -G
    assert G + G == (2 - q) * G
    assert G + G == (2 + q) * G

while True:
    q = choice(qs)
    G = q_to_g[q]

    r, s = randint(0, q-1), randint(0, q-1)
    t, u = r + s, r - s
    assert r * G + s * G == t * G
    assert r * G - s * G == u * G
    assert (r * s % q) * G == r * (s * G)
    assert r * G + s * G == s * G + r * G
    assert r * (G + G) == r * (2 * G)
    assert r * G == (r + q) * G

    P = randint(1, q-1) * G
    Q = randint(1, q-1) * G
    assert P + Q == Q + P
    assert (P + Q) - Q == P
    assert r * (P + Q) == r * P + r * Q
    assert P + (-P) == 0 * G

    clz = type(P)
    if clz is Ed25519Point or clz is Ed448Point:
        Pt = clz(P.x, P.y, projective=True)
    else:
        Pt = clz(P.x, P.y, P.curve, projective=True)

    assert (r * Pt).normalize() == r * P
    assert (r * Pt + s * Pt).normalize() == t * P
    assert (r * Pt - s * Pt).normalize() == u * P
