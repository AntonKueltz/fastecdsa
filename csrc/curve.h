#ifndef CURVE_H
#define CURVE_H

#include "gmp.h"

#include "point.h"

// curve over a prime field
typedef struct {
    mpz_t p, a, b, q;
    PointZZ_p * g;
} CurveZZ_p;

CurveZZ_p * buildCurveZZ_p(char * p, char * a, char * b, char * q, char * gx, char * gy, int base);
void destroyCurveZZ_p(CurveZZ_p * curve);

#endif
