#ifndef POINT_H
#define POINT_H

#include "gmp.h"

// point in a prime field
typedef struct {
    mpz_t x, y;
} PointZZ_p;

PointZZ_p * buildPointZZ_p(char * x, char * y, int base);
void destroyPointZZ_p(PointZZ_p * point);

#endif
