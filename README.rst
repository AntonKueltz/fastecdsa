fastecdsa
=========
.. image:: https://img.shields.io/pypi/v/fastecdsa.svg
    :target: https://pypi.org/project/fastecdsa/
    :alt: PyPI

.. image:: https://readthedocs.org/projects/fastecdsa/badge/?version=stable
    :target: https://fastecdsa.readthedocs.io/en/stable/?badge=stable
    :alt: Documentation Status

.. contents::

About
-----
This is a python package for doing fast elliptic curve cryptography, specifically
digital signatures.

Security
--------
There is no nonce reuse, no branching on secret material,
and all points are validated before any operations are performed on them. Timing side challenges
are mitigated via Montgomery point multiplication. Nonces are generated per RFC6979_. The default
curve used throughout the package is P256 which provides 128 bits of security. If you require a
higher level of security you can specify the curve parameter in a method to use a curve over a
bigger field e.g. P384. All that being said, crypto is tricky and I'm not beyond making mistakes.
Please use a more established and reviewed library for security critical applications. Open an
issue or email me if you see any security issue or risk with this library.

Python Versions Supported
-------------------------
The initial release of this package was targeted at python2.7. Earlier versions may work but have
no guarantee of correctness or stability. As of release 1.2.1+ python3 is supported as well. Due to
python2's EOL on January 1st 2020 release 2.x of this package only supports python3.

Operating Systems Supported
---------------------------
As of v4 most flavors of Linux/MacOS/Windows are supported. v3 and below requires the GMP library
which has historically made building and installing on the Windows platform difficult. Note that
this package is optimized for 64bit operating systems that support 64bit mathematical operations
like multiplication in their instruction set.

Supported Primitives
--------------------
Curves over Prime Fields
~~~~~~~~~~~~~~~~~~~~~~~~

+---------------------------+-----------------------------------------+-------------+
| Name                      | Class                                   | Proposed By |
+===========================+=========================================+=============+
| P192 / secp192r1          | :code:`fastecdsa.curve.P192`            | NIST / NSA  |
+---------------------------+-----------------------------------------+-------------+
| P224 / secp224r1          | :code:`fastecdsa.curve.P224`            | NIST / NSA  |
+---------------------------+-----------------------------------------+-------------+
| P256 / secp256r1          | :code:`fastecdsa.curve.P256`            | NIST / NSA  |
+---------------------------+-----------------------------------------+-------------+
| P384 / secp384r1          | :code:`fastecdsa.curve.P384`            | NIST / NSA  |
+---------------------------+-----------------------------------------+-------------+
| P521 / secp521r1          | :code:`fastecdsa.curve.P521`            | NIST / NSA  |
+---------------------------+-----------------------------------------+-------------+
| secp192k1                 | :code:`fastecdsa.curve.secp192k1`       | Certicom    |
+---------------------------+-----------------------------------------+-------------+
| secp224k1                 | :code:`fastecdsa.curve.secp224k1`       | Certicom    |
+---------------------------+-----------------------------------------+-------------+
| secp256k1 (bitcoin curve) | :code:`fastecdsa.curve.secp256k1`       | Certicom    |
+---------------------------+-----------------------------------------+-------------+
| brainpoolP160r1           | :code:`fastecdsa.curve.brainpoolP160r1` | BSI         |
+---------------------------+-----------------------------------------+-------------+
| brainpoolP192r1           | :code:`fastecdsa.curve.brainpoolP192r1` | BSI         |
+---------------------------+-----------------------------------------+-------------+
| brainpoolP224r1           | :code:`fastecdsa.curve.brainpoolP224r1` | BSI         |
+---------------------------+-----------------------------------------+-------------+
| brainpoolP256r1           | :code:`fastecdsa.curve.brainpoolP256r1` | BSI         |
+---------------------------+-----------------------------------------+-------------+
| brainpoolP320r1           | :code:`fastecdsa.curve.brainpoolP320r1` | BSI         |
+---------------------------+-----------------------------------------+-------------+
| brainpoolP384r1           | :code:`fastecdsa.curve.brainpoolP384r1` | BSI         |
+---------------------------+-----------------------------------------+-------------+
| brainpoolP512r1           | :code:`fastecdsa.curve.brainpoolP512r1` | BSI         |
+---------------------------+-----------------------------------------+-------------+

Arbitrary Curves
~~~~~~~~~~~~~~~~
As of version 1.5.1 construction of arbitrary curves in Weierstrass form
(:code:`y^2 = x^3 + ax + b (mod p)`) is supported. I advise against using custom curves for any
security sensitive applications. Some sanity checks are done -
* `p` must be a non-even (not 2) prime
* The curve cannot be singular (discriminant equal to 0)
* `(gx, gy)` must be a point on the curve
* `q` must be prime

Exhaustive validation checks are not performed e.g., no check that `q` is actually the order of
point `(gx, gy)` is performed.

.. code:: python

    from fastecdsa.curve import Curve
    curve = Curve(
        name,  # (str): The name of the curve
        p,  # (long): The value of p in the curve equation.
        a,  # (long): The value of a in the curve equation.
        b,  # (long): The value of b in the curve equation.
        q,  # (long): The order of the base point of the curve.
        gx,  # (long): The x coordinate of the base point of the curve.
        gy,  # (long): The y coordinate of the base point of the curve.
    )

Hash Functions
~~~~~~~~~~~~~~
Any hash function in the :code:`hashlib` module (:code:`md5, sha1, sha224, sha256, sha384, sha512`)
will work, as will any hash function that implements the same interface / core functionality as the
those in :code:`hashlib`. For instance, if you wish to use SHA3 as the hash function the
:code:`pysha3` package will work with this library as long as it is at version >=1.0b1 (as previous
versions didn't work with the :code:`hmac` module which is used in nonce generation). Note
that :code:`sha3_224, sha3_256, sha3_384, sha3_512` are all in :code:`hashlib` as of python3.6.

Performance
-----------

Curves over Prime Fields
~~~~~~~~~~~~~~~~~~~~~~~~
You can see the times for 1,000 signature and verification operations over
various curves below. These were run on a machine with a 3.6 GHz Intel Core i9-9900K.

+-----------------+------------------------+-------------------------------+-------------------------+
| Curve           | :code:`fastecdsa` v4   | :code:`ecdsa` (gmpy2 backend) |    :code:`fastecdsa` v3 |
+-----------------+------------------------+-------------------------------+-------------------------+
| P192            | 0.16s                  | 0.98s                         | 1.16s                   |
+-----------------+------------------------+-------------------------------+-------------------------+
| P224            | 0.30s                  | 1.21s                         | 1.51s                   |
+-----------------+------------------------+-------------------------------+-------------------------+
| P256            | 0.36s                  | 1.38s                         | 1.91s                   |
+-----------------+------------------------+-------------------------------+-------------------------+
| P384            | 0.91s                  | 2.25s                         | 4.08s                   |
+-----------------+------------------------+-------------------------------+-------------------------+
| P521            | 1.92s                  | 3.63s                         | 7.08s                   |
+-----------------+------------------------+-------------------------------+-------------------------+
| Secp192k1       | 0.12s                  | N/A                           | 1.19s                   |
+-----------------+------------------------+-------------------------------+-------------------------+
| Secp224k1       | 0.23s                  | N/A                           | 1.56s                   |
+-----------------+------------------------+-------------------------------+-------------------------+
| Secp256k1       | 0.26s                  | 1.34s                         | 1.92s                   |
+-----------------+------------------------+-------------------------------+-------------------------+
| Brainpoolp160r1 | 0.14s                  | 0.83s                         | 0.88s                   |
+-----------------+------------------------+-------------------------------+-------------------------+
| Brainpoolp192r1 | 0.16s                  | 1.01s                         | 1.18s                   |
+-----------------+------------------------+-------------------------------+-------------------------+
| Brainpoolp224r1 | 0.31s                  | 1.21s                         | 1.55s                   |
+-----------------+------------------------+-------------------------------+-------------------------+
| Brainpoolp256r1 | 0.36s                  | 1.35                          | 1.88s                   |
+-----------------+------------------------+-------------------------------+-------------------------+
| Brainpoolp320r1 | 0.67s                  | 1.80s                         | 2.91s                   |
+-----------------+------------------------+-------------------------------+-------------------------+
| Brainpoolp384r1 | 1.03s                  | 2.27s                         | 4.08s                   |
+-----------------+------------------------+-------------------------------+-------------------------+
| Brainpoolp512r1 | 2.07s                  | 3.33s                         | 7.06s                   |
+-----------------+------------------------+-------------------------------+-------------------------+

If you'd like to benchmark performance on your machine you can do so using the command:

.. code:: bash

    $ uv run benchmark

This will use the :code:`timeit` module to benchmark 1000 signature and verification operations
for each curve supported by this package. Alternatively, if you have not cloned the repo but
have installed the package to your site packages you can use the following command:

.. code:: bash

    $ python -m fastecdsa.benchmark

Installing
----------
You can use the usual tools to install / add this package to your project -

.. code:: bash

    $ uv add fastecdsa
    $ poetry add fastecdsa
    $ pip install fastecdsa

You can also clone the repo and use :code:`$ uv run maturin develop`. Note that you need to have
the rust toolchain on your machine if you clone and build.

Development
-----------
This package uses :code:`uv` for package management. You can install it via `pip install uv`. First build
the rust bindings

.. code:: bash

    $ uv run maturin develop -r

To run the test suite use the following command

.. code:: bash

    $ uv run pytest

Install pre-commit hooks to ensure type checking and autoformatting happens before you commit your code

.. code:: bash

    $ uv run pre-commit install

To build the docs use the following command, which will create a :code:`docs/_build` directory with the
docs built as HTML files

.. code:: bash

    $ cd docs
    $ uv run make html

Publishing
~~~~~~~~~~
Note that currently only the package owner is able to publish releases to PyPI. The following steps
can still be used to generate source and wheel distributions, but note that the publish command will
not work.

To build a release first install all supported versions of python into the environment (double check
:code:`pyproject.toml` for which python versions are supported)

.. code:: bash

   $ uv python install 3.11 3.12 3.13 3.14

Then build a source distribution, followed by wheels for each supported python version

.. code:: bash

    $ uv run maturin sdist
    $ uv run maturin build -r -i python3.x

Then publish the source and wheels distributions to the test PyPI account.

.. code:: bash

    $ uv publish --token {token} --url https://test.pypi.org/simple/


Usage
-----
Generating Keys
~~~~~~~~~~~~~~~
You can use this package to generate keys if you like. Recall that private keys on elliptic curves
are integers, and public keys are points i.e. integer pairs.

.. code:: python

    from fastecdsa import keys, curve

    """The reason there are two ways to generate a keypair is that generating the public key requires
    a point multiplication, which can be expensive. That means sometimes you may want to delay
    generating the public key until it is actually needed."""

    # generate a keypair (i.e. both keys) for curve P256
    priv_key, pub_key = keys.gen_keypair(curve.P256)

    # generate a private key for curve P256
    priv_key = keys.gen_private_key(curve.P256)

    # get the public key corresponding to the private key we just generated
    pub_key = keys.get_public_key(priv_key, curve.P256)


Signing and Verifying
~~~~~~~~~~~~~~~~~~~~~
Some basic usage is shown below:

.. code:: python

    from fastecdsa import curve, ecdsa, keys
    from hashlib import sha384

    m = "a message to sign via ECDSA"  # some message

    ''' use default curve and hash function (P256 and SHA2) '''
    private_key = keys.gen_private_key(curve.P256)
    public_key = keys.get_public_key(private_key, curve.P256)
    # standard signature, returns two integers
    r, s = ecdsa.sign(m, private_key)
    # should return True as the signature we just generated is valid.
    valid = ecdsa.verify((r, s), m, public_key)

    ''' specify a different hash function to use with ECDSA '''
    r, s = ecdsa.sign(m, private_key, hashfunc=sha384)
    valid = ecdsa.verify((r, s), m, public_key, hashfunc=sha384)

    ''' specify a different curve to use with ECDSA '''
    private_key = keys.gen_private_key(curve.P224)
    public_key = keys.get_public_key(private_key, curve.P224)
    r, s = ecdsa.sign(m, private_key, curve=curve.P224)
    valid = ecdsa.verify((r, s), m, public_key, curve=curve.P224)

    ''' using SHA3 via pysha3>=1.0b1 package '''
    import sha3  # pip install [--user] pysha3==1.0b1
    from hashlib import sha3_256
    private_key, public_key = keys.gen_keypair(curve.P256)
    r, s = ecdsa.sign(m, private_key, hashfunc=sha3_256)
    valid = ecdsa.verify((r, s), m, public_key, hashfunc=sha3_256)

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

Importing and Exporting Keys
~~~~~~~~~~~~~~~~~~~~~~~~~~~~
You can also export keys as files, ASN.1 encoded and formatted per RFC5480_ and RFC5915_. Both
private keys and public keys can be exported as follows:

.. code:: python

    from fastecdsa.curve import P256
    from fastecdsa.encoding.pem import PEMEncoder
    from fastecdsa.keys import export_private_key, export_public_key, gen_keypair

    d, Q = gen_keypair(P256)
    encoder = PEMEncoder()

    # save the private key to disk
    export_private_key(d, curve=P256, encoder=encoder, filepath='/path/to/exported/p256.key')
    # save the public key to disk (curve inferred from Q)
    export_public_key(Q, encoder=encoder, filepath='/path/to/exported/p256.pub')

Keys stored in this format can also be imported. The import function will figure out if the key
is a public or private key and parse it accordingly:

.. code:: python

    from fastecdsa.curve import P256
    from fastecdsa.encoding.pem import PEMEncoder
    from fastecdsa.keys import import_public_key, import_private_key

    decoder = PEMEncoder()
    # returns an int
    parsed_d = import_private_key('/path/to/exported/p256.key', decoder=decoder)
    # returns a fastecdsa.point.Point
    parsed_Q = import_public_key('/path/to/exported/p256.pub', curve=P256, decoder=decoder)

Other encoding formats can also be specified, such as SEC1_ for public keys. This is done using
classes found in the :code:`fastecdsa.encoding` package, and passing them as keyword args to
the key functions:

.. code:: python

    from fastecdsa.curve import P256
    from fastecdsa.encoding.sec1 import SEC1Encoder
    from fastecdsa.keys import export_public_key, gen_keypair, import_public_key

    _, Q = gen_keypair(P256)
    encoder = SEC1Encoder()

    export_public_key(Q, encoder=encoder, filepath='/path/to/exported/p256.pub')
    parsed_Q = import_public_key('/path/to/exported/p256.pub', curve=P256, decoder=encoder)

Encoding Signatures
~~~~~~~~~~~~~~~~~~~
DER encoding of ECDSA signatures as defined in RFC2459_ is also supported. The
:code:`fastecdsa.encoding.der` provides the :code:`DEREncoder` class which encodes signatures:

.. code:: python

    from fastecdsa.encoding.der import DEREncoder

    r, s = 0xdeadc0de, 0xbadc0de
    encoded = DEREncoder.encode_signature(r, s)
    decoded_r, decoded_s = DEREncoder.decode_signature(encoded)

Acknowledgements
----------------
Thanks to those below for contributing improvements:

- boneyard93501
- clouds56
- m-kus
- sirk390
- targon
- NotStatilko
- bbbrumley
- luinxz
- JJChiDguez
- J08nY
- trevor-crypto
- sylvainpelissier
- akaIDIOT
- Peter-Bergman
- DimitriPapadopoulos

.. _RFC2459: https://tools.ietf.org/html/rfc2459
.. _RFC5480: https://tools.ietf.org/html/rfc5480
.. _RFC5915: https://tools.ietf.org/html/rfc5915
.. _RFC6979: https://tools.ietf.org/html/rfc6979
.. _SEC1: http://www.secg.org/sec1-v2.pdf
