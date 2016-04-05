#ifndef _ECDSA_H
#define _ECDSA_H

#ifdef __APPLE__
#include <Python/Python.h>
#else
#include <Python.h>
#endif

#include <gmp.h>
#include "curveMath.h"

typedef struct {
    mpz_t r, s;
} Sig;

void sign(Sig * sig, char * msg, mpz_t d, mpz_t k, Curve * curve);
int verify(Sig * sig, char * msg, Point * Q, Curve * curve);

#endif
