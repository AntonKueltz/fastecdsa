#include <gmp.h>

typedef struct Curve {
    mpz_t p, a, b, q;
} Curve;

Curve * buildCurve(char * p, char * a, char * b, char * q, int base);
void destroyCurve(Curve * curve);
