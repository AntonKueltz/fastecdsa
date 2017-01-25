#ifndef CURVE_H
#define CURVE_H

#include "gmp.h"

#include "binaryField.h"
#include "point.h"

// curve over a prime field
typedef struct {
    mpz_t p, a, b, q;
    PointZZ_p * g;
} CurveZZ_p;

// curve over a binary field
typedef struct {
    BinaryField * pt, * a, * b;
    mpz_t q;
    PointZZ_pX * g;
    unsigned degree;
} CurveZZ_pX;

CurveZZ_p * buildCurveZZ_p(char * p, char * a, char * b, char * q, char * gx, char * gy, int base);
void destroyCurveZZ_p(CurveZZ_p * curve);

CurveZZ_pX * buildCurveZZ_pX(unsigned * pt, unsigned ptlen, unsigned degree, int a, char * q, char * gx, char * gy, int base);
void destroyCurveZZ_pX(CurveZZ_pX * curve);

CurveZZ_p * buildP192(void);
CurveZZ_p * buildP224(void);
CurveZZ_p * buildP256(void);
CurveZZ_p * buildP384(void);
CurveZZ_p * buildP521(void);

CurveZZ_p * buildSecp256k1(void);

CurveZZ_pX * buildK163(void);
CurveZZ_pX * buildK233(void);
CurveZZ_pX * buildK283(void);
CurveZZ_pX * buildK409(void);
CurveZZ_pX * buildK571(void);

#endif
