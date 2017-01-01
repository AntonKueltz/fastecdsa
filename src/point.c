#include "point.h"
#include <stdlib.h>

PointZZ_p * buildPointZZ_p(char * x, char * y, int base) {
    PointZZ_p * point = (PointZZ_p *)malloc(sizeof(PointZZ_p));
    mpz_init_set_str(point->x, x, base);
    mpz_init_set_str(point->y, y, base);
    return point;
}

void destroyPointZZ_p(PointZZ_p * point) {
    mpz_clears(point->x, point->y, NULL);
    free(point);
}

PointZZ_pX * buildPointZZ_pX(char * x, char * y, int base, unsigned degree, fq_ctx_t ctx) {
    PointZZ_pX * point = (PointZZ_pX *)malloc(sizeof(PointZZ_pX));

    fmpz_t one;
    fmpz_init_set_ui(one, 1);

    // set the base point
    mpz_t gx, gy;
    mpz_inits(gx, gy, NULL);
    mpz_set_str(gx, x, base);
    mpz_set_str(gy, y, base);

    // convert the base point to polynomials
    fq_poly_init2(point->x, degree, ctx);
    fq_poly_init2(point->y, degree, ctx);

    unsigned bitIndex;
    for(bitIndex = 0; bitIndex < degree; bitIndex++) {
        if(mpz_tstbit(gx, bitIndex)) {
            fq_poly_set_coeff_fmpz(point->x, bitIndex, one, ctx);
        }
        if(mpz_tstbit(gy, bitIndex)) {
            fq_poly_set_coeff_fmpz(point->y, bitIndex, one, ctx);
        }
    }

    mpz_clears(gx, gy, NULL);
    fmpz_clear(one);
    return point;
}

void destroyPointZZ_pX(PointZZ_pX * point) {
    // fq_poly_clear(z, )
}
