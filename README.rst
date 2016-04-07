fastecdsa
=========
.. image:: https://travis-ci.org/AntonKueltz/fastecdsa.svg?branch=master
    :target: https://travis-ci.org/AntonKueltz/fastecdsa

About
-----
This is a python package for doing fast elliptic curve cryptography, specifically
digital signatures.

Supported Primitives
--------------------
Curves
~~~~~~
* P192 (:code:`fastecdsa.curve.P192`)
* P224 (:code:`fastecdsa.curve.P224`)
* P256 (:code:`fastecdsa.curve.P256`)
* P384 (:code:`fastecdsa.curve.P384`)
* P521 (:code:`fastecdsa.curve.P521`)
* secp256k1 (bitcoin curve) (:code:`fastecdsa.curve.secp256k1`)

Hash Functions
~~~~~~~~~~~~~~
* SHA1 (:code:`hashlib.sha1`)
* SHA224 (:code:`hashlib.sha224`)
* SHA256 (SHA2) (:code:`hashlib.sha256`)
* SHA384 (:code:`hashlib.sha384`)
* SHA512 (:code:`hashlib.sha512`)

Performance
-----------
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
Some basic usage is shown below:

.. code:: python

    from fastecdsa import curve, ecdsa
    from hashlib import sha384

    m = "a message to sign via ECDSA"  # some message

    ''' use default curve and hash function (P256 and SHA2) '''
    private_key, public_key = ecdsa.gen_keypair()
    # standard signature, returns two integers
    r, s = ecdsa.sign(m, private_key)
    # should return True as the signature we just generated is valid.
    valid = ecdsa.verify((r, s), m, public_key)


    ''' specify a different curve to use with ECDSA '''
    private_key, public_key = ecdsa.gen_keypair(curve=curve.P224)
    r, s = ecdsa.sign(m, private_key, curve=curve.P224)
    valid = ecdsa.verify((r, s), m, public_key, curve=curve.P224)

    ''' specify a different hash function to use with ECDSA '''
    private_key, public_key = ecdsa.gen_keypair()
    r, s = ecdsa.sign(m, private_key, hashfunc=sha384)
    valid = ecdsa.verify((r, s), m, public_key, hashfunc=sha384)

Security
--------
No known current issues. Timing side challenges are mitigated via Montgomery
point multiplication. Nonces are generated per RFC6979.


.. _GMP: https://gmplib.org/
