#ifndef POINT_H
#define POINT_H

#include <gmp.h>

typedef struct {
    mpz_t x, y;
} Point;

Point * buildPoint(char * x, char * y, int base);
void destroyPoint(Point * point);

#endif
