#ifndef _ECDSA_H
#define _ECDSA_H

#include <Python.h>

#if PY_MAJOR_VERSION >= 3
#define IS_PY3K
#endif

#include <gmp.h>
#include "curveMath.h"

typedef struct {
    mpz_t r, s;
} Sig;

void sign(Sig * sig, char * msg, mpz_t d, mpz_t k, Curve * curve);
int verify(Sig * sig, char * msg, Point * Q, Curve * curve);

#endif
