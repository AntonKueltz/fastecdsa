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


PointZZ_pX * buildPointZZ_pX(char * x, char * y, int base, unsigned degree) {
    PointZZ_pX * point = (PointZZ_pX *)malloc(sizeof(PointZZ_pX));

    // set the base point
    mpz_t gx, gy;
    mpz_inits(gx, gy, NULL);
    mpz_set_str(gx, x, base);
    mpz_set_str(gy, y, base);

    // convert the base point to polynomials
    point->x = f2m_init(degree);
    point->y = f2m_init(degree);

    unsigned bitIndex;
    for(bitIndex = 0; bitIndex < degree; bitIndex++) {
        if(mpz_tstbit(gx, bitIndex)) {
            f2m_set_bit(point->x, bitIndex);
            point->x->degree = bitIndex;
        }
        if(mpz_tstbit(gy, bitIndex)) {
            f2m_set_bit(point->y, bitIndex);
            point->y->degree = bitIndex;
        }
    }

    mpz_clears(gx, gy, NULL);
    return point;
}


void destroyPointZZ_pX(PointZZ_pX * point) {
    f2m_clear(point->x);
    f2m_clear(point->y);
}
