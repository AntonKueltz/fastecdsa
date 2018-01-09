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
