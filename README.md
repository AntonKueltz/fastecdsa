#fastecdsa

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
* P256

#Installing
Run `python setup.py install`. Note that you need to have C compiler that
supports the C99  standard. You also need to have [GMP](https://gmplib.org/) on
your system as the underlying C code in this package includes the `gmp.h` header
(and links against gmp via the `-lgmp` flag).

#Usage
Some kinks are still being worked out, so far limited usage is as follows:

#### elliptic curve point scalar multiplication
```python
from fastecdsa import point

point = point.Point(..., ..., ['P256'|'P192'])  # whatever curve coords you want on a supported curve
d = ...  # some scalar
result = point * d
print result.x, result.y  # printing resulting coordinates
```

#### signing via ECDSA
*Currently only works with P256*
```python
from fastecdsa import ecdsa

d = ...  # some scalar
m = "..."  # some message

(r, s) = ecdsa.sign(m, d)  # standard signature, returns two integers
```
