#ifndef CURVE_H
#define CURVE_H

#include <gmp.h>
#include "point.h"

typedef struct {
    mpz_t p, a, b, q;
    Point * g;
} Curve;

Curve * buildCurve(char * p, char * a, char * b, char * q, char * gx, char * gy, int base);
void destroyCurve(Curve * curve);

Curve * buildP192();
Curve * buildP256();

#endif
