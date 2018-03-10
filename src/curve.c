#include "curve.h"
#include <stdlib.h>

CurveZZ_p * buildCurveZZ_p(char * p, char * a, char * b, char * q, char * gx, char * gy, int base) {
    CurveZZ_p * curve = (CurveZZ_p *)malloc(sizeof(CurveZZ_p));
    mpz_init_set_str(curve->p, p, base);
    mpz_init_set_str(curve->a, a, base);
    mpz_init_set_str(curve->b, b, base);
    mpz_init_set_str(curve->q, q, base);
    curve->g = buildPointZZ_p(gx, gy, base);
    return curve;
}

void destroyCurveZZ_p(CurveZZ_p * curve) {
    mpz_clears(curve->p, curve->a, curve->b, curve->q, NULL);
    destroyPointZZ_p(curve->g);
    free(curve);
}
