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
    mpz_clears(curve->p, curve->a, curve->b, curve->q, NULL);
    free(curve);
}

Curve * buildP256() {
    return buildCurve(
        "115792089210356248762697446949407573530086143415290314195533631308867097853951",
        "-3",
        "41058363725152142129326129780047268409114441015993725554835256314039467401291",
        "115792089210356248762697446949407573529996955224135760342422259061068512044369",
        10
    );
}
