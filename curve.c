#include "curve.h"
#include <stdlib.h>

Curve * buildCurve(char * p, char * a, char * b, char * q, int base) {
    Curve * curve = (Curve *)malloc(sizeof(Curve));
    mpz_init_set_str(curve->p, p, base);
    mpz_init_set_str(curve->a, a, base);
    mpz_init_set_str(curve->b, b, base);
    mpz_init_set_str(curve->q, q, base);
    return curve;
}

void destroyCurve(Curve * curve) {
    mpz_clears(NULL);
    free(curve);
}
