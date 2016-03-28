#fastecdsa
[![Build Status](https://travis-ci.org/AntonKueltz/fastecdsa.svg?branch=master)](https://travis-ci.org/AntonKueltz/fastecdsa)

**DO NOT USE THIS PACKAGE FOR PROD, I HAVE NOT MITIGATED SIDE CHANNELS YET**

This is a python package for doing elliptic curve cryptography, specifically
digital signatures. Currently it does basic point multiplication significantly
faster than the `ecdsa` package. You can see the times for 10,000 point
multiplications below, `fast.py` corresponding to this package and `regular.py`
corresponding to `ecdsa` package.

![Performance](http://i.imgur.com/olqitSs.png?1)

As you can see, this package in this case is ~50x faster.

# Supported Curves
* P192
* P224
* P256
* P384
* P521

#Installing
Run `python setup.py install`. Note that you need to have C compiler that
supports the C99  standard. You also need to have [GMP](https://gmplib.org/) on
your system as the underlying C code in this package includes the `gmp.h` header
(and links against gmp via the `-lgmp` flag).

#Usage
Some kinks are still being worked out, so far limited usage is as follows:

#### signing and verifying signatures via ECDSA
```python
from fastecdsa import curve, ecdsa

keys = ecdsa.KeyPair(curve=curve.P256)  # use NOST curve P256
m = "..."  # some message

(r, s) = keys.sign(m)  # standard signature, returns two integers
valid = keys.verify((r, s), m)  # should return True as the signature we just generated is valid.
```
