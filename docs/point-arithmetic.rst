Point Arithmetic
================

Arbitrary Elliptic Curve Arithmetic
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
The :code:`Point` class allows arbitrary arithmetic to be performed over curves. The two main
operations are point addition and point multiplication (by a scalar) which can be done via the
standard python operators (:code:`+` and :code:`*` respectively):

.. code:: python

    # example taken from the document below (section 4.3.2):
    # https://koclab.cs.ucsb.edu/teaching/cren/docs/w02/nist-routines.pdf

    from fastecdsa.curve import P256
    from fastecdsa.point import Point

    xs = 0xde2444bebc8d36e682edd27e0f271508617519b3221a8fa0b77cab3989da97c9
    ys = 0xc093ae7ff36e5380fc01a5aad1e66659702de80f53cec576b6350b243042a256
    S = Point(xs, ys, curve=P256)

    xt = 0x55a8b00f8da1d44e62f6b3b25316212e39540dc861c89575bb8cf92e35e0986b
    yt = 0x5421c3209c2d6c704835d82ac4c3dd90f61a8a52598b9e7ab656e9d8c8b24316
    T = Point(xt, yt, curve=P256)

    # Point Addition
    R = S + T

    # Point Subtraction: (xs, ys) - (xt, yt) = (xs, ys) + (xt, -yt)
    R = S - T

    # Point Doubling
    R = S + S  # produces the same value as the operation below
    R = 2 * S  # S * 2 works fine too i.e. order doesn't matter

    d = 0xc51e4753afdec1e6b6c6a5b992f43f8dd0c7a8933072708b6522468b2ffb06fd

    # Scalar Multiplication
    R = d * S  # S * d works fine too i.e. order doesn't matter

    e = 0xd37f628ece72a462f0145cbefe3f0b355ee8332d37acdd83a358016aea029db7

    # Joint Scalar Multiplication
    R = d * S + e * T

Points on Edwards Curves
________________________
The edwards curves that underpin Ed25519 and Ed448 have their own dedicated point classes
as they use different point arithmetic and representations than the Weierstrass points. See
below for some example usage.

.. code:: python

    In [1]: from fastecdsa.eddsa import Ed25519Point as e25519

    In [2]: G = e25519.g()

    In [3]: G
    Out[3]:
    X: 0x216936d3cd6e53fec0a4e231fdd6dc5c692cc7609525a7b2c9562d608f25d51a
    Y: 0x6666666666666666666666666666666666666666666666666666666666666658
    (Affine point on curve Edwards25519)

    In [4]: G * 2
    Out[4]:
    X: 0x36ab384c9f5a046c3d043b7d1833e7ac080d8e4515d7a45f83c5a14e2843ce0e
    Y: 0x2260cdf3092329c21da25ee8c9a21f5697390f51643851560e5f46ae6af8a3c9
    (Affine point on curve Edwards25519)

    In [5]: G + G
    Out[5]:
    X: 0x36ab384c9f5a046c3d043b7d1833e7ac080d8e4515d7a45f83c5a14e2843ce0e
    Y: 0x2260cdf3092329c21da25ee8c9a21f5697390f51643851560e5f46ae6af8a3c9
    (Affine point on curve Edwards25519)

    In [6]: e25519.scale_base(2)
    Out[6]:
    X: 0x5881d418cc897a3a0eb49cdb9e6859fe2c55894bcdf7f8095714be2d5cf2e79a
    Y: 0x27e006a46c706615d5ef0e2ff33fb48a6fbc113c77c0bd916574e8d0e2c42106
    Z: 0x1f7616052c96e0810d7ff513e392dfbad03e66e0a95c8d2796a29516b07725f4
    T: 0x5dec8aec83e2c5266dd025b8da706eed6a2729713b3f035372c8624d35caa7b
    (Extended projective point on curve Edwards25519)

    In [7]: _.normalize()
    Out[7]:
    X: 0x36ab384c9f5a046c3d043b7d1833e7ac080d8e4515d7a45f83c5a14e2843ce0e
    Y: 0x2260cdf3092329c21da25ee8c9a21f5697390f51643851560e5f46ae6af8a3c9
    (Affine point on curve Edwards25519

Note that :code:`fastecdsa.eddsa.Ed448Point` has the same interface, but it uses standard
projective coordinates that omit "T".

Projective Points
~~~~~~~~~~~~~~~~~
As of release 4.0.0 you can also use the projective representation of points. This
may be desirable if you are doing a lot of intermediate calculations with points
before you need an affine representation as arithmetic on projective points is cheaper.
Note that comparisons between projective points and affine points are also possible,
but they are less performant than comparisons between two affine points. By default
all representations and operations on points are affine. To convert a projective point
back to an affine point use the :code:`normalize` method.

.. code:: python

    In [1]: from fastecdsa.curve import P256

    In [2]: from fastecdsa.point import Point

    In [3]: x = 0xdeadc0de

    In [4]: x * P256.G
    Out[4]:
    X: 0x32079326d26449f8b36bde4410f805eb520c0120da1585c79c369f356c8a298f
    Y: 0x249daa2c57c8d6a90575630635aa5448fa56d21f5e363c155fe98c597b1c70ae
    (Affine point on curve "P256")

    In [5]: g_ = Point(P256.G.x, P256.G.y, P256, projective=True)

    In [6]: h = x * g_

    In [7]: h
    Out[7]:
    X: 0x15819ed127c46b11b751a1d575a6e7712fe72c03e693ea0783268e2f0d4bdfac
    Y: 0x879367bd0ebcf1b97fdb0841c33333f7c321a41c2ced6ab14a1cd2cc7684f6bf
    Z: 0x99682c948ddc2ddd15966aecfe83fe52b1df7d76255d460f90d7bd615bb80cfc
    (Projective point on curve "P256")

    In [8]: h.normalize()  # gives the same result as x * P256.G
    Out[8]:
    X: 0x32079326d26449f8b36bde4410f805eb520c0120da1585c79c369f356c8a298f
    Y: 0x249daa2c57c8d6a90575630635aa5448fa56d21f5e363c155fe98c597b1c70ae
    (Affine point on curve "P256")
