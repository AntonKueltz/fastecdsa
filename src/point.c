#include "point.h"
#include <stdlib.h>

uint32_t K163_X[K163_WORDS] = {0x5c94eee8, 0xde4e6d5e, 0xaa07d793, 0x7bbc11ac, 0xfe13c053, 0x2};
uint32_t K163_Y[K163_WORDS] = {0xccdaa3d9, 0x0536d538, 0x321f2e80, 0x5d38ff58, 0x89070fb0, 0x2};
uint32_t K163_Z[K163_WORDS] =  {0x1};
PointZZ_pX K163_G = {K163_X, K163_Y, K163_Z};

uint32_t K233_X[K233_WORDS] = {0xefad6126, 0x0a4c9d6e, 0x19c26bf5, 0x149563a4, 0x29f22ff4,
                               0x7e731af1, 0x32ba853a, 0x172};
uint32_t K233_Y[K233_WORDS] = {0x56fae6a3, 0x56e0c110, 0xf18aeb9b, 0x27a8cd9b, 0x555a67c4,
                               0x19b7f70f, 0x537dece8, 0x1db};
uint32_t K233_Z[K233_WORDS] =  {0x1};
PointZZ_pX K233_G = {K233_X, K233_Y, K233_Z};

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
