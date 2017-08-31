#ifndef POINT_H
#define POINT_H

#include "gmp.h"

#include "binaryField.h"

// point in a prime field
typedef struct {
    mpz_t x, y;
} PointZZ_p;

PointZZ_p * buildPointZZ_p(char * x, char * y, int base);
void destroyPointZZ_p(PointZZ_p * point);

// point in a binary field
typedef struct {
    uint32_t * x;
    uint32_t * y;
    uint32_t * z;
} PointZZ_pX;

extern PointZZ_pX K163_G;
extern PointZZ_pX K233_G;

#endif
