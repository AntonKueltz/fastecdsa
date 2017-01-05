#include <stdint.h>

typedef struct {
    unsigned degree;
    unsigned wordslen;
    uint32_t * words;
} BinaryField;

BinaryField * f2m_init(unsigned m);
BinaryField * f2m_copy(const BinaryField * op);
void f2m_clear(BinaryField * f2m);

void f2m_set_bit(BinaryField * op, unsigned bitIndex);

BinaryField * f2m_add(const BinaryField * op1, const BinaryField * op2);
BinaryField * f2m_mul(const BinaryField * op1, const BinaryField * op2);
BinaryField * f2m_mulmod(const BinaryField * op1, const BinaryField * op2, unsigned degree);
BinaryField * f2m_invmod(const BinaryField * op, const BinaryField * mod);
void f2m_reduce_k163(BinaryField * op);

void f2m_pretty_print(const BinaryField * op, const char * var);
