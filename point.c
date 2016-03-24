#include "point.h"
#include <stdlib.h>

Point * buildPoint(char * x, char * y, int base) {
    Point * point = (Point *)malloc(sizeof(Point));
    mpz_init_set_str(point->x, x, base);
    mpz_init_set_str(point->y, y, base);
    return point;
}

void destroyPoint(Point * point) {
    mpz_clears(point->x, point->y, NULL);
    free(point);
}
