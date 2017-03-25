fastecdsa
=========
.. image:: https://travis-ci.org/AntonKueltz/fastecdsa.svg?branch=master
    :target: https://travis-ci.org/AntonKueltz/fastecdsa
.. image:: https://badge.fury.io/py/fastecdsa.svg
    :target: https://badge.fury.io/py/fastecdsa

About
-----
This is a python package for doing fast elliptic curve cryptography, specifically
digital signatures.

Security
--------
I am not aware of any current issues. There is no nonce reuse, no branching on secret material,
and all points are validated before any operations are performed on them. Timing side challenges
are mitigated via Montgomery point multiplication. Nonces are generated per RFC6979. The default
curve used throughout the package is P256 which provides 128 bits of security. If you require a
higher level of security you can specify the curve parameter in a method to use a curve over a
bigger field e.g. P384. All that being said, crypto is tricky and I'm not beyond making mistakes.
Please use a more established and reviewed library for security critical applications. Open an
issue or email me if you see any security issue or risk with this library.

Python Versions Supported
-------------------------
The initial release of this package was targeted at python2.7. Earlier versions may work but have
no guarantee of correctness or stability. As of release 1.2.1+ python3 is now supported as well.

Supported Primitives
--------------------
Curves over Prime Fields
~~~~~~~~~~~~~~~~~~~~~~~~
* P192 (:code:`fastecdsa.curve.P192`)
* P224 (:code:`fastecdsa.curve.P224`)
* P256 (:code:`fastecdsa.curve.P256`)
* P384 (:code:`fastecdsa.curve.P384`)
* P521 (:code:`fastecdsa.curve.P521`)
* secp256k1 (bitcoin curve) (:code:`fastecdsa.curve.secp256k1`)

Hash Functions
~~~~~~~~~~~~~~
Any hash function in the :code:`hashlib` module (:code:`md5, sha1, sha224, sha256, sha384, sha512`)
will work, as will any hash function that implements the same interface / core functionality as the
those in :code:`hashlib`. For instance, if you wish to use SHA3 as the hash function the
:code:`pysha3` package will work with this library as long as it is at version >=1.0b1 (as previous
versions didn't work with the :code:`hmac` module which is used in nonce generation).

Performance
-----------

Curves over Prime Fields
~~~~~~~~~~~~~~~~~~~~~~~~
Currently it does basic point multiplication significantly faster than the :code:`ecdsa`
package. You can see the times for 1,000 signature and verification operations below,
:code:`fast.py` corresponding to this package and :code:`regular.py` corresponding
to :code:`ecdsa` package.

.. image:: http://i.imgur.com/oNOfnG6.png?1

As you can see, this package in this case is ~25x faster.

Installing
----------
You can use pip: :code:`$ pip install fastecdsa` or clone the repo and use
:code:`$ python setup.py install`. Note that you need to have a C compiler.
You  also need to have GMP_ on your system as the underlying
C code in this package includes the :code:`gmp.h` header (and links against gmp
via the :code:`-lgmp` flag). On debian you can install all dependencies as follows:

.. code:: bash

    $ sudo apt-get install python-dev libgmp3-dev

Usage
-----
Generating Keys
~~~~~~~~~~~~~~~
You can use this package to generate keys if you like. Recall that private keys on elliptic curves
are integers, and public keys are points i.e. integer pairs.

.. code:: python

    from fastecdsa import keys, curve

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

Acknowledgements
----------------
Thanks to those below for contributing improvements:

- targon

.. _GMP: https://gmplib.org/
