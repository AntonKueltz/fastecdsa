#include <gmp.h>

typedef struct Point {
    mpz_t x, y;
} Point;

Point * buildPoint(char * x, char * y, int base);
void destroyPoint(Point * point);
