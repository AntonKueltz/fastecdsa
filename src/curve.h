#ifndef CURVE_H
#define CURVE_H

#include "gmp.h"

#include "flint/fq.h"
#include "flint/fq_poly.h"

#include "point.h"

// curve over a prime field
typedef struct {
    mpz_t p, a, b, q;
    PointZZ_p * g;
} CurveZZ_p;

// curve over a binary field
typedef struct {
    fq_poly_t pt, a, b;
    PointZZ_pX * g;
    unsigned degree;
    fq_ctx_t ctx;
} CurveZZ_pX;

CurveZZ_p * buildCurveZZ_p(char * p, char * a, char * b, char * q, char * gx, char * gy, int base);
void destroyCurveZZ_p(CurveZZ_p * curve);

CurveZZ_pX * buildCurveZZ_pX(unsigned * pt, unsigned ptlen, unsigned degree, int a, char * gx, char * gy, int base);
void destroyCurveZZ_pX(CurveZZ_pX * curve);

CurveZZ_p * buildP192(void);
CurveZZ_p * buildP224(void);
CurveZZ_p * buildP256(void);
CurveZZ_p * buildP384(void);
CurveZZ_p * buildP521(void);

CurveZZ_p * buildSecp256k1(void);

CurveZZ_pX * buildK163(void);

#endif
