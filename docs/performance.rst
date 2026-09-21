Performance
-----------
Several optimizations are included in this package, including specialized
field arithmetic based on the field of the curve, specialized point addition
and doubling based on the curve parameters, Lim-Lee combs for faster point
scaling in the sign operation, interleaved wNAF for faster multiple point
arithmetic in the verify operation, and lattice reductions for finding smaller
scalars in EdDSA verify operations. There is still much room for improving
performance, contributions and suggestions are welcomed.

You can see some performance metrics and comparative analysis to both the
:code:`ecdsa[gmpy2]` and :code:`cryptography` packages below. Note that
the latter package wraps OpenSSL for much of its ECDSA and EdDSA logic.
These were run on a machine with a 3.6 GHz Intel Core i9-9900K. This data
is given solely to provide an idea of performance across several popular python
packages and is not meant to imply anything about the quality of these
packages or their suitability for a given cryptographic application.

.. code:: bash

    fastecdsa ECDSA benchmark: 1s per test, hash=sha256, single-threaded
                                            sign    verify     sign/s   verify/s
     192 bits ecdsa (P192)             0.000064s 0.000084s    15540.4    11923.3
     224 bits ecdsa (P224)             0.000103s 0.000155s     9707.4     6434.6
     256 bits ecdsa (P256)             0.000115s 0.000189s     8658.7     5303.7
     384 bits ecdsa (P384)             0.000287s 0.000495s     3485.6     2018.7
     521 bits ecdsa (P521)             0.000535s 0.001083s     1869.5      923.1
     160 bits ecdsa (brainpoolP160r1)  0.000059s 0.000075s    16923.4    13281.7
     192 bits ecdsa (brainpoolP192r1)  0.000065s 0.000089s    15477.3    11230.2
     224 bits ecdsa (brainpoolP224r1)  0.000099s 0.000162s    10101.3     6168.3
     256 bits ecdsa (brainpoolP256r1)  0.000116s 0.000187s     8635.2     5350.5
     320 bits ecdsa (brainpoolP320r1)  0.000181s 0.000346s     5537.5     2886.2
     384 bits ecdsa (brainpoolP384r1)  0.000265s 0.000530s     3769.2     1888.4
     512 bits ecdsa (brainpoolP512r1)  0.000491s 0.001144s     2037.2      874.0
     192 bits ecdsa (secp192k1)        0.000055s 0.000060s    18172.5    16786.3
     224 bits ecdsa (secp224k1)        0.000088s 0.000121s    11345.3     8258.9
     256 bits ecdsa (secp256k1)        0.000093s 0.000135s    10805.0     7424.1
     255 bits eddsa (Ed25519)          0.000095s 0.000104s    10505.6     9603.0
     448 bits eddsa (Ed448)            0.000301s 0.000304s     3323.4     3293.5

    Baseline: OpenSSL (via `cryptography`)
                                            sign    verify     sign/s   verify/s   sign x  verify x
     192 bits ecdsa (P192)             0.000193s 0.000182s     5175.5     5499.8     3.00      2.17
     224 bits ecdsa (P224)             0.000045s 0.000076s    22449.5    13166.3     0.43      0.49
     256 bits ecdsa (P256)             0.000026s 0.000060s    38236.9    16606.3     0.23      0.32
     384 bits ecdsa (P384)             0.000178s 0.000391s     5606.0     2558.0     0.62      0.79
     521 bits ecdsa (P521)             0.000227s 0.000414s     4409.7     2415.2     0.42      0.38
    brainpoolP160r1: not supported by OpenSSL (via `cryptography`), skipping
    brainpoolP192r1: not supported by OpenSSL (via `cryptography`), skipping
    brainpoolP224r1: not supported by OpenSSL (via `cryptography`), skipping
     256 bits ecdsa (brainpoolP256r1)  0.000312s 0.000299s     3200.2     3340.0     2.70      1.60
    brainpoolP320r1: not supported by OpenSSL (via `cryptography`), skipping
     384 bits ecdsa (brainpoolP384r1)  0.000729s 0.000630s     1370.9     1587.6     2.75      1.19
     512 bits ecdsa (brainpoolP512r1)  0.001028s 0.000855s      972.6     1169.7     2.09      0.75
    secp192k1: not supported by OpenSSL (via `cryptography`), skipping
    secp224k1: not supported by OpenSSL (via `cryptography`), skipping
     256 bits ecdsa (secp256k1)        0.000345s 0.000308s     2896.6     3243.6     3.73      2.29
     255 bits eddsa (Ed25519)          0.000029s 0.000089s    34547.2    11257.3     0.30      0.85
     448 bits eddsa (Ed448)            0.000167s 0.000176s     5987.3     5685.0     0.56      0.58

    Baseline: python-ecdsa
                                            sign    verify     sign/s   verify/s   sign x  verify x
     192 bits ecdsa (P192)             0.000206s 0.000727s     4843.2     1375.5     3.21      8.67
     224 bits ecdsa (P224)             0.000269s 0.000864s     3719.2     1156.8     2.61      5.56
     256 bits ecdsa (P256)             0.000293s 0.000987s     3416.4     1012.9     2.53      5.24
     384 bits ecdsa (P384)             0.000500s 0.001688s     1999.9      592.3     1.74      3.41
     521 bits ecdsa (P521)             0.000721s 0.002732s     1386.5      366.1     1.35      2.52
     160 bits ecdsa (brainpoolP160r1)  0.000194s 0.000600s     5157.8     1667.0     3.28      7.97
     192 bits ecdsa (brainpoolP192r1)  0.000229s 0.000700s     4366.4     1429.6     3.54      7.86
     224 bits ecdsa (brainpoolP224r1)  0.000257s 0.000870s     3890.6     1149.8     2.60      5.36
     256 bits ecdsa (brainpoolP256r1)  0.000290s 0.001017s     3449.7      983.6     2.50      5.44
     320 bits ecdsa (brainpoolP320r1)  0.000377s 0.001338s     2653.1      747.2     2.09      3.86
     384 bits ecdsa (brainpoolP384r1)  0.000467s 0.001728s     2140.2      578.7     1.76      3.26
     512 bits ecdsa (brainpoolP512r1)  0.000647s 0.002549s     1546.4      392.3     1.32      2.23
    secp192k1: not supported by python-ecdsa, skipping
    secp224k1: not supported by python-ecdsa, skipping
     256 bits ecdsa (secp256k1)        0.000297s 0.001013s     3367.4      987.4     3.21      7.52
     255 bits eddsa (Ed25519)          0.000205s 0.001571s     4882.5      636.4     2.15     15.09
     448 bits eddsa (Ed448)            0.000425s 0.003353s     2353.5      298.2     1.41     11.04
