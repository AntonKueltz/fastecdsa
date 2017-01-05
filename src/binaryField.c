#include "binaryField.h"
#include <stdio.h>
#include <stdlib.h>

#define WSIZE (sizeof(uint32_t) * 8)


BinaryField * f2m_init(unsigned degree) {
    BinaryField * f2m = (BinaryField *)malloc(sizeof(BinaryField));
    f2m->degree = degree;
    degree++; // we also need a bit for degree 0
    f2m->wordslen = degree / WSIZE + (degree % WSIZE ? 1 : 0);
    f2m->words = (uint32_t *)malloc(f2m->wordslen * WSIZE);

    unsigned i;
    for(i = 0; i < f2m->wordslen; i++) {
        f2m->words[i] = 0;
    }

    return f2m;
}


BinaryField * f2m_copy(const BinaryField * op) {
    BinaryField * copy = (BinaryField *)malloc(sizeof(BinaryField));
    copy->degree = op->degree;
    copy->wordslen = op->wordslen;
    copy->words = (uint32_t *)malloc(copy->wordslen * WSIZE);

    unsigned i;
    for(i = 0; i < copy->wordslen; i++) {
        copy->words[i] = op->words[i];
    }

    return copy;
}


void f2m_clear(BinaryField * f2m) {
    free(f2m->words);
    free(f2m);
}


void f2m_set_bit(BinaryField * op, unsigned bitIndex) {
    unsigned word = bitIndex / WSIZE;
    unsigned bit = bitIndex % WSIZE;
    op->words[word] |= (1 << bit);
}


BinaryField * f2m_add(const BinaryField * op1, const BinaryField * op2) {
    BinaryField * rop = f2m_init(op1->degree);
    unsigned i;

    for(i = 0; i < op1->wordslen; i++) {
        rop->words[i] = op1->words[i] ^ op2->words[i];
    }

    return rop;
}


BinaryField * f2m_mul(const BinaryField * op1, const BinaryField * op2) {
    BinaryField * rop = f2m_init(op1->degree + op2->degree);
    BinaryField * b = f2m_copy(op2);
    unsigned k, j, i;

    for(k = 0; k < WSIZE; k++) {
        for(j = 0; j < op1->wordslen; j ++) {
            if(op1->words[j] & (1 << k)) {
                for(i = 0; i < b->wordslen; i++) {
                    rop->words[i+j] = rop->words[i+j] ^ b->words[i];
                }
            }
        }

        if(k != (WSIZE-1)) {
            if(b->words[b->wordslen-1] & (1 << (WSIZE-1))) {
                uint32_t * newWords = (uint32_t *)malloc((b->wordslen + 1) * WSIZE);
                int carryBit = 0;

                for(i = 0; i < b->wordslen; i++) {
                    newWords[i] = (b->words[i] << 1) | carryBit;
                    carryBit = b->words[i] & (1 << (WSIZE-1)) ? 1 : 0;
                }

                newWords[b->wordslen] = 1;
                free(b->words);
                b->words = newWords;
                b->wordslen++;
            }
            else {
                int carryBit = 0;

                for(i = 0; i < b->wordslen; i++) {
                    int newCarry = b->words[i] & (1 << (WSIZE-1)) ? 1 : 0;
                    b->words[i] = (b->words[i] << 1) | carryBit;
                    carryBit = newCarry;
                }
            }
        }
    }

    f2m_clear(b);
    return rop;
}


BinaryField * f2m_mulmod(const BinaryField * op1, const BinaryField * op2, unsigned degree) {
    BinaryField * rop = f2m_mul(op1, op2);

    switch(degree) {
    case 163:
        f2m_reduce_k163(rop);
        break;

    default:
        break;
    }

    return rop;
}


BinaryField * f2m_invmod(const BinaryField * op, const BinaryField * mod) {
    BinaryField * u = f2m_copy(op);
    BinaryField * v = f2m_copy(mod);
    BinaryField * g1 = f2m_init(op->degree);
    BinaryField * g2 = f2m_init(op->degree);
    f2m_set_bit(g1, 0);

    f2m_clear(g2);
    f2m_clear(v);
    f2m_clear(u);
    return g1;
}


void f2m_reduce_k163(BinaryField * op) {
    // see Guide to Elliptic Curve Cryptography - Algorithm 2.41
    unsigned i;
    uint32_t t;

    for(i = op->wordslen; i >= 6; i--) {
        t = op->words[i];
        op->words[i-6] = op->words[i-6] ^ (t << 29);
        op->words[i-5] = op->words[i-5] ^ (t << 4) ^ (t << 3) ^ t ^ (t >> 3);
        op->words[i-4] = op->words[i-4] ^ (t >> 28) ^ (t >> 29);
    }

    t = op->words[5] >> 3;
    op->words[0] = op->words[0] ^ (t << 7) ^ (t << 6) ^ (t << 3) ^ t;
    op->words[1] = op->words[1] ^ (t >> 25) ^ (t >> 26);
    op->words[5] = op->words[5] & 0x7;
    op->wordslen = 6;
}


void f2m_hex_print(const BinaryField * op) {
    int i;
    printf("0x");

    for(i = op->wordslen-1; i >= 0; i--) {
        printf("%x", op->words[i]);
    }
    printf("\n");
}


void f2m_pretty_print(const BinaryField * op, const char * var) {
    int start = 1;
    unsigned i, j;

    for(i = 0; i < op->wordslen; i++) {
        for(j = 0; j < WSIZE; j++) {
            int set = (op->words[i] & (1 << j));

            if(set) {
                unsigned degree = j + i * WSIZE;

                if(degree == 0) {
                    printf("%s%d", (start ? "" : " + "), 1);
                }
                else {
                    printf("%s%s^%d", (start ? "" : " + "), var, degree);
                }
                if(start) start = 0;
            }
        }
    }
    printf("\n");
}
