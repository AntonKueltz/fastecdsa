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


int f2m_equal(BinaryField * op1, BinaryField * op2) {
    if(op1->wordslen != op2->wordslen) {
        return 0;
    }

    unsigned i, diff = 0;
    for(i = 0; i < op1->wordslen; i++) {
        diff = diff | (op1->words[i] ^ op2->words[i]);
    }

    return diff == 0;
}


void f2m_to_mpz(mpz_t rop, BinaryField * op) {
    unsigned i;
    for(i = 0; i <= op->degree; i++) {
        if(f2m_is_set(op, i)) {
            mpz_setbit(rop, i);
        }
    }
}


void f2m_set_bit(BinaryField * op, unsigned bitIndex) {
    unsigned word = bitIndex / WSIZE;
    unsigned bit = bitIndex % WSIZE;
    op->words[word] |= (1 << bit);
}


int f2m_is_set(const BinaryField * op, const unsigned bitIndex) {
    if(bitIndex > op->degree) {
        return 0;
    }

    unsigned word = bitIndex / WSIZE;
    unsigned bit = bitIndex % WSIZE;

    return op->words[word] & (1 << bit);
}


int f2m_is_zero(const BinaryField * op) {
    unsigned i = 0;

    for(i = 0; i < op->wordslen; i++) {
        if(op->words[i] != 0) {
            return 0;
        }
    }

    return 1;
}


int f2m_is_one(const BinaryField * op) {
    unsigned i = 0;

    if(op->words[i] != 1) {
        return 0;
    }

    for(i = 1; i < op->wordslen; i++) {
        if(op->words[i] != 0) {
            return 0;
        }
    }

    return 1;
}


BinaryField * f2m_add(const BinaryField * op1, const BinaryField * op2) {
    if(f2m_is_zero(op1)) {
        return f2m_copy(op2);
    }
    else if(f2m_is_zero(op2)) {
        return f2m_copy(op1);
    }

    unsigned maxWords = op1->wordslen > op2->wordslen ? op1->wordslen : op2->wordslen;
    BinaryField * rop = f2m_init(maxWords * WSIZE - 1);
    int i;

    for(i = 0; i < maxWords; i++) {
        if(i >= op1->wordslen) {
            rop->words[i] = op2->words[i];
        }
        else if(i >= op2->wordslen) {
            rop->words[i] = op1->words[i];
        }
        else {
            rop->words[i] = op1->words[i] ^ op2->words[i];
        }
    }

    _f2m_recalculate_degree(rop);
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
        _f2m_reduce_k163(rop);
        break;
    case 233:
        _f2m_reduce_k233(rop);
        break;
    case 283:
        _f2m_reduce_k283(rop);
        break;
    case 409:
        _f2m_reduce_k409(rop);
        break;
    case 571:
        _f2m_reduce_k571(rop);
        break;
    default:
        break;
    }

    return rop;
}


BinaryField * f2m_invmod(const BinaryField * op, const BinaryField * mod) {
    BinaryField * u = f2m_copy(op);
    BinaryField * v = f2m_copy(mod);
    BinaryField * g1 = f2m_init(0);
    BinaryField * g2 = f2m_init(0);
    BinaryField * tmp;
    f2m_set_bit(g1, 0);

    while(!f2m_is_one(u)) {
        _f2m_recalculate_degree(u);
        _f2m_recalculate_degree(v);
        _f2m_recalculate_degree(g1);
        _f2m_recalculate_degree(g2);
        int j = (int)u->degree - (int)v->degree;

        if(j < 0) {
            tmp = u;
            u = v;
            v = tmp;
            tmp = g1;
            g1 = g2;
            g2 = tmp;
            j = -j;
        }

        _f2m_scaled_add(u, v, j);
        _f2m_scaled_add(g1, g2, j);
    }

    f2m_clear(g2);
    f2m_clear(v);
    f2m_clear(u);
    return g1;
}


void _f2m_reduce_k163(BinaryField * op) {
    // see Guide to Elliptic Curve Cryptography - Algorithm 2.41
    unsigned i;
    uint32_t t;

    for(i = 10; i >= 6; i--) {
        t = i < op->wordslen ? op->words[i] : 0;
        op->words[i-6] = op->words[i-6] ^ (t << 29);
        op->words[i-5] = op->words[i-5] ^ (t << 4) ^ (t << 3) ^ t ^ (t >> 3);
        op->words[i-4] = op->words[i-4] ^ (t >> 28) ^ (t >> 29);
    }

    t = op->words[5] >> 3;
    op->words[0] = op->words[0] ^ (t << 7) ^ (t << 6) ^ (t << 3) ^ t;
    op->words[1] = op->words[1] ^ (t >> 25) ^ (t >> 26);
    op->words[5] = op->words[5] & 0x7;
    op->wordslen = 6;

    _f2m_recalculate_degree(op);
}


void _f2m_reduce_k233(BinaryField * op) {
    // see Guide to Elliptic Curve Cryptography - Algorithm 2.42
    unsigned i;
    uint32_t t;

    for(i = 15; i >= 8; i--) {
        t = i < op->wordslen ? op->words[i] : 0;
        op->words[i-8] = op->words[i-8] ^ (t << 23);
        op->words[i-7] = op->words[i-7] ^ (t >> 9);
        op->words[i-5] = op->words[i-5] ^ (t << 1);
        op->words[i-4] = op->words[i-4] ^ (t >> 31);
    }

    t = op->words[7] >> 9;
    op->words[0] = op->words[0] ^ t;
    op->words[2] = op->words[2] ^ (t << 10);
    op->words[3] = op->words[3] ^ (t >> 22);
    op->words[7] = op->words[7] & 0x1FF;
    op->wordslen = 8;

    _f2m_recalculate_degree(op);
}


void _f2m_reduce_k283(BinaryField * op) {
    // see Guide to Elliptic Curve Cryptography - Algorithm 2.43
    unsigned i;
    uint32_t t;

    for(i = 17; i >= 9; i--) {
        t = i < op->wordslen ? op->words[i] : 0;
        op->words[i-9] = op->words[i-9] ^ (t << 5) ^ (t << 10) ^ (t << 12) ^ (t << 17);
        op->words[i-8] = op->words[i-8] ^ (t >> 27) ^ (t >> 22) ^ (t >> 20) ^ (t >> 15);
    }

    t = op->words[8] >> 27;
    op->words[0] = op->words[0] ^ t ^ (t << 5) ^ (t << 7) ^ (t << 12);
    op->words[8] = op->words[8] & 0x7FFFFFF;
    op->wordslen = 9;

    _f2m_recalculate_degree(op);
}


void _f2m_reduce_k409(BinaryField * op) {
    // see Guide to Elliptic Curve Cryptography - Algorithm 2.44
    unsigned i;
    uint32_t t;

    for(i = 25; i >= 13; i--) {
        t = i < op->wordslen ? op->words[i] : 0;
        op->words[i-13] = op->words[i-13] ^ (t << 7);
        op->words[i-12] = op->words[i-12] ^ (t >> 25);
        op->words[i-11] = op->words[i-11] ^ (t << 30);
        op->words[i-10] = op->words[i-10] ^ (t >> 2);
    }

    t = op->words[12] >> 25;
    op->words[0] = op->words[0] ^ t;
    op->words[2] = op->words[2] ^ (t << 23);
    op->words[12] = op->words[12] & 0x1FFFFFF;
    op->wordslen = 13;

    _f2m_recalculate_degree(op);
}


void _f2m_reduce_k571(BinaryField * op) {
    // see Guide to Elliptic Curve Cryptography - Algorithm 2.45
    unsigned i;
    uint32_t t;

    for(i = 35; i >= 18; i--) {
        t = i < op->wordslen ? op->words[i] : 0;
        op->words[i-18] = op->words[i-18] ^ (t << 5) ^ (t << 7) ^ (t << 10) ^ (t << 15);
        op->words[i-17] = op->words[i-17] ^ (t >> 27) ^ (t >> 25) ^ (t >> 22) ^ (t >> 17);
    }

    t = op->words[17] >> 27;
    op->words[0] = op->words[0] ^ t ^ (t << 2) ^ (t << 5) ^ (t << 10);
    op->words[17] = op->words[17] & 0x7FFFFFF;
    op->wordslen = 18;

    _f2m_recalculate_degree(op);
}


void _f2m_recalculate_degree(BinaryField * op) {
    int i, j;

    for(i = op->wordslen - 1; i >= 0; i--) {
        for(j = WSIZE - 1; j >= 0; j--) {
            if(op->words[i] & (1 << j)) {
                op->degree = (i * WSIZE) + j;
                return;
            }
        }
    }

    op->degree = 0;
}


void _f2m_scaled_add(BinaryField * rop, const BinaryField * op, const unsigned shift) {
    unsigned maxDegree = rop->degree > (op->degree + shift) ? rop->degree : (op->degree + shift);
    unsigned maxWords = (maxDegree / WSIZE) + 1;
    unsigned i;

    if(maxWords > rop->wordslen) {
        uint32_t * newWords = (uint32_t *)malloc(WSIZE * (rop->wordslen + 1));
        for(i = 0; i < rop->wordslen; i++) {
            newWords[i] = rop->words[i];
        }
        newWords[rop->wordslen] = 0;
        free(rop->words);
        rop->words = newWords;
        rop->wordslen = maxWords;
    }

    uint32_t mask = ((1 << shift) - 1) << (WSIZE - shift);
    uint32_t shiftedBits = 0;

    for(i = 0; i < maxWords; i++) {
        uint32_t tmp = op->words[i] & mask;
        uint32_t shiftedWord = (op->words[i] << shift) | shiftedBits;

        if(i >= rop->wordslen) {
            rop->words[i] = shiftedWord;
        }
        else if(i >= op->wordslen) {
            rop->words[i] = rop->words[i] ^ shiftedBits;
        }
        else {
            rop->words[i] = rop->words[i] ^ shiftedWord;
        }

        shiftedBits = tmp >> (WSIZE - shift);
    }

    _f2m_recalculate_degree(rop);
}


void _f2m_left_shift(BinaryField * op, const unsigned amt) {
    if(amt == 0) {
        return;
    }

    op->degree += amt;
    uint32_t mask = ((1 << amt) - 1) << (WSIZE - amt);
    unsigned shiftedBits = 0, i;

    for(i = 0; i < op->wordslen; i++) {
        unsigned tmp = op->words[i] & mask;
        op->words[i] = (op->words[i] << amt) | shiftedBits;
        shiftedBits = tmp >> (WSIZE - amt);
    }

    if(shiftedBits) {
        op->wordslen++;
        uint32_t * newWords = malloc(WSIZE * op->wordslen);
        for(i = 0; i < op->wordslen-1; i++) {
            newWords[i] = op->words[i];
        }

        free(op->words);
        op->words = newWords;
        op->words[op->wordslen-1] = shiftedBits;
    }
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
