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
Curve * buildP224();
Curve * buildP256();
Curve * buildP384();
Curve * buildP521();
Curve * buildSecp256k1();

#endif
