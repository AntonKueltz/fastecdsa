#ifndef POINT_H
#define POINT_H

#include "gmp.h"

#include "binaryField.h"

// point in a prime field
typedef struct {
    mpz_t x, y;
} PointZZ_p;

// point in a binary field
typedef struct {
    BinaryField * x, * y;
} PointZZ_pX;

PointZZ_p * buildPointZZ_p(char * x, char * y, int base);
void destroyPointZZ_p(PointZZ_p * point);

PointZZ_pX * buildPointZZ_pX(char * x, char * y, int base, unsigned degree);
void destroyPointZZ_pX(PointZZ_pX * point);

#endif
