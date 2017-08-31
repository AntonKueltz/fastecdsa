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

CurveZZ_p * buildCurveZZ_p(char * p, char * a, char * b, char * q, char * gx, char * gy, int base);
void destroyCurveZZ_p(CurveZZ_p * curve);

CurveZZ_p * buildP192(void);
CurveZZ_p * buildP224(void);
CurveZZ_p * buildP256(void);
CurveZZ_p * buildP384(void);
CurveZZ_p * buildP521(void);

CurveZZ_p * buildSecp256k1(void);

// curve over a binary field
typedef struct {
    mpz_t a, b, q;
    Field * field;
    PointZZ_pX * g;
} CurveZZ_pX;

void destroyCurveZZ_pX(CurveZZ_pX * curve);

CurveZZ_pX * buildK163(void);
CurveZZ_pX * buildK233(void);

#endif
