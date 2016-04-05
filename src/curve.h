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

Curve * buildP192(void);
Curve * buildP224(void);
Curve * buildP256(void);
Curve * buildP384(void);
Curve * buildP521(void);
Curve * buildSecp256k1(void);

#endif
