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
* P192
* P224
* P256
* P384
* P521
* secp256k1 (bitcoin curve)

Hash Functions
~~~~~~~~~~~~~~
* SHA1
* SHA224
* SHA256 (SHA2)
* SHA384
* SHA512

Performance
-----------
Currently it does basic point multiplication significantly faster than the :code:`ecdsa`
package. You can see the times for 1,000 signature and verification operations below,
:code:`fast.py` corresponding to this package and :code:`regular.py` corresponding 
to :code:`ecdsa` package.

.. image:: http://i.imgur.com/ZH8Oodm.png

As you can see, this package in this case is ~25x faster.

Installing
----------
You can use pip: :code:`$ pip install fastecdsa` or clone the repo and use 
:code:`$ python setup.py install`. Note that you need to have C compiler. 
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

    keys = ecdsa.KeyPair(curve=curve.P256)  # use NIST curve P256
    m = "a message to sign via ECDSA"  # some message

    (r, s) = keys.sign(m)  # standard signature, returns two integers
    valid = keys.verify((r, s), m)  # should return True as the signature we just generated is valid.

    ''' specify a different hash function to use with ECDSA '''
    keys = ecdsa.KeyPair(curve=curve.P256, hashfunc=sha384)  # use NIST curve P256 with SHA384
    (r, s) = keys.sign(m)
    valid = keys.verify((r, s), m)

Security
--------
No known current issues. Timing side challenges are prevented via Montgomery
point multiplication. Nonces are generate per RFC6979.


.. _GMP: https://gmplib.org/
