#ifndef BINARYFIELD_H
#define BINARYFIELD_H

#include <stdint.h>
#include "gmp.h"

#define BITS_IN_WORD 32

enum FieldName { K163, K233, K283, K409, K571 };

typedef struct {
    const enum FieldName identifier;
    const unsigned words;  // max 32bit words needed to represent a field element
    const unsigned words_unreduced;  // max 32bit words needed to represent multiplication result
    uint32_t * polynomial;  // the reduction polynomial for the field
} Field;

// K163 constants
#define K163_WORDS 6
#define K163_WORDS_UNREDUCED 11
extern uint32_t K163_POLYNOMIAL[K163_WORDS];
extern Field K163_FIELD;

// K233 constants
#define K233_WORDS 8
#define K233_WORDS_UNREDUCED 15
extern uint32_t K233_POLYNOMIAL[K233_WORDS];
extern Field K233_FIELD;

// K283 constants
#define K283_WORDS 9
#define K283_WORDS_UNREDUCED 18
extern uint32_t K283_POLYNOMIAL[K283_WORDS];
extern Field K283_FIELD;

// K409 constants
#define K409_WORDS 13
#define K409_WORDS_UNREDUCED 26
extern uint32_t K409_POLYNOMIAL[K409_WORDS];
extern Field K409_FIELD;

// K571 constants
#define K571_WORDS 18
#define K571_WORDS_UNREDUCED 36
extern uint32_t K571_POLYNOMIAL[K571_WORDS];
extern Field K571_FIELD;

extern uint16_t SQR_T[0x100];

void bfAdd(uint32_t * op1, uint32_t * op2, uint32_t * rop, Field * field);
void bfInv(uint32_t * op, uint32_t * rop, Field * field);
void bfMul(uint32_t * op1, uint32_t * op2, uint32_t * rop, Field * field);
void bfSqr(uint32_t * op, uint32_t * rop, Field * field);
void bfPrint(uint32_t * op, Field * field);

#endif
