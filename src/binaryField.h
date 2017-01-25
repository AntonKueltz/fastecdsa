#ifndef BINARYFIELD_H
#define BINARYFIELD_H

#include <stdint.h>

#include "gmp.h"

typedef struct {
    unsigned degree;
    unsigned wordslen;
    uint32_t * words;
} BinaryField;

BinaryField * f2m_init(unsigned m);
BinaryField * f2m_copy(const BinaryField * op);
void f2m_clear(BinaryField * f2m);
int f2m_equal(BinaryField * op1, BinaryField * op2);
void f2m_to_mpz(mpz_t rop, BinaryField * op);

void f2m_set_bit(BinaryField * op, unsigned bitIndex);
int f2m_is_set(const BinaryField * op, const unsigned bitIndex);
int f2m_is_zero(const BinaryField * op);
int f2m_is_one(const BinaryField * op);

BinaryField * f2m_add(const BinaryField * op1, const BinaryField * op2);
BinaryField * f2m_mul(const BinaryField * op1, const BinaryField * op2);
BinaryField * f2m_mulmod(const BinaryField * op1, const BinaryField * op2, unsigned degree);
BinaryField * f2m_invmod(const BinaryField * op, const BinaryField * mod);

void _f2m_reduce_k163(BinaryField * op);
void _f2m_reduce_k233(BinaryField * op);
void _f2m_reduce_k283(BinaryField * op);
void _f2m_reduce_k409(BinaryField * op);
void _f2m_reduce_k571(BinaryField * op);

void _f2m_recalculate_degree(BinaryField * op);
void _f2m_scaled_add(BinaryField * rop, const BinaryField * op, const unsigned shift);
void _f2m_left_shift(BinaryField * op, const unsigned amt);
void f2m_pretty_print(const BinaryField * op, const char * var);

#endif
