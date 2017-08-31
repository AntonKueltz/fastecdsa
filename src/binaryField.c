#include "binaryField.h"

#include <stdio.h>
#include <stdlib.h>

// K163 constants
uint32_t K163_POLYNOMIAL[K163_WORDS] = {0xc9, 0x0, 0x0, 0x0, 0x0, 0x8};
Field K163_FIELD = {
    K163,
    K163_WORDS,
    K163_WORDS_UNREDUCED,
    K163_POLYNOMIAL
};

// K233 constants
uint32_t K233_POLYNOMIAL[K233_WORDS] = {0x1, 0x0, 0x400, 0x0, 0x0, 0x0, 0x0, 0x200};
Field K233_FIELD = {
    K233,
    K233_WORDS,
    K233_WORDS_UNREDUCED,
    K233_POLYNOMIAL
};

// K283 constants
uint32_t K283_POLYNOMIAL[K283_WORDS] = {0x10a1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x8000000};
Field K283_FIELD = {
    K283,
    K283_WORDS,
    K283_WORDS_UNREDUCED,
    K283_POLYNOMIAL
};

// K409 constants
uint32_t K409_POLYNOMIAL[K409_WORDS] = {0x1, 0x0, 0x800000, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x2000000};
Field K409_FIELD = {
    K409,
    K409_WORDS,
    K409_WORDS_UNREDUCED,
    K409_POLYNOMIAL
};

// K571 constants
uint32_t K571_POLYNOMIAL[K571_WORDS] = {0x425, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x8000000};
Field K571_FIELD = {
    K571,
    K571_WORDS,
    K571_WORDS_UNREDUCED,
    K571_POLYNOMIAL
};

uint16_t SQR_T[0x100] = {
    0x0, 0x1, 0x4, 0x5, 0x10, 0x11, 0x14, 0x15, 0x40, 0x41, 0x44, 0x45, 0x50, 0x51, 0x54, 0x55,
    0x100, 0x101, 0x104, 0x105, 0x110, 0x111, 0x114, 0x115, 0x140, 0x141, 0x144, 0x145, 0x150,
    0x151, 0x154, 0x155, 0x400, 0x401, 0x404, 0x405, 0x410, 0x411, 0x414, 0x415, 0x440, 0x441,
    0x444, 0x445, 0x450, 0x451, 0x454, 0x455, 0x500, 0x501, 0x504, 0x505, 0x510, 0x511, 0x514,
    0x515, 0x540, 0x541, 0x544, 0x545, 0x550, 0x551, 0x554, 0x555, 0x1000, 0x1001, 0x1004, 0x1005,
    0x1010, 0x1011, 0x1014, 0x1015, 0x1040, 0x1041, 0x1044, 0x1045, 0x1050, 0x1051, 0x1054, 0x1055,
    0x1100, 0x1101, 0x1104, 0x1105, 0x1110, 0x1111, 0x1114, 0x1115, 0x1140, 0x1141, 0x1144, 0x1145,
    0x1150, 0x1151, 0x1154, 0x1155, 0x1400, 0x1401, 0x1404, 0x1405, 0x1410, 0x1411, 0x1414, 0x1415,
    0x1440, 0x1441, 0x1444, 0x1445, 0x1450, 0x1451, 0x1454, 0x1455, 0x1500, 0x1501, 0x1504, 0x1505,
    0x1510, 0x1511, 0x1514, 0x1515, 0x1540, 0x1541, 0x1544, 0x1545, 0x1550, 0x1551, 0x1554, 0x1555,
    0x4000, 0x4001, 0x4004, 0x4005, 0x4010, 0x4011, 0x4014, 0x4015, 0x4040, 0x4041, 0x4044, 0x4045,
    0x4050, 0x4051, 0x4054, 0x4055, 0x4100, 0x4101, 0x4104, 0x4105, 0x4110, 0x4111, 0x4114, 0x4115,
    0x4140, 0x4141, 0x4144, 0x4145, 0x4150, 0x4151, 0x4154, 0x4155, 0x4400, 0x4401, 0x4404, 0x4405,
    0x4410, 0x4411, 0x4414, 0x4415, 0x4440, 0x4441, 0x4444, 0x4445, 0x4450, 0x4451, 0x4454, 0x4455,
    0x4500, 0x4501, 0x4504, 0x4505, 0x4510, 0x4511, 0x4514, 0x4515, 0x4540, 0x4541, 0x4544, 0x4545,
    0x4550, 0x4551, 0x4554, 0x4555, 0x5000, 0x5001, 0x5004, 0x5005, 0x5010, 0x5011, 0x5014, 0x5015,
    0x5040, 0x5041, 0x5044, 0x5045, 0x5050, 0x5051, 0x5054, 0x5055, 0x5100, 0x5101, 0x5104, 0x5105,
    0x5110, 0x5111, 0x5114, 0x5115, 0x5140, 0x5141, 0x5144, 0x5145, 0x5150, 0x5151, 0x5154, 0x5155,
    0x5400, 0x5401, 0x5404, 0x5405, 0x5410, 0x5411, 0x5414, 0x5415, 0x5440, 0x5441, 0x5444, 0x5445,
    0x5450, 0x5451, 0x5454, 0x5455, 0x5500, 0x5501, 0x5504, 0x5505, 0x5510, 0x5511, 0x5514, 0x5515,
    0x5540, 0x5541, 0x5544, 0x5545, 0x5550, 0x5551, 0x5554, 0x5555
};

void bfAdd(uint32_t * op1, uint32_t * op2, uint32_t * rop, Field * field) {
    for(int i = 0; i < field->words; i++) {
        rop[i] = op1[i] ^ op2[i];
    }
}

void bfK163Mod(uint32_t * op, uint32_t * rop) {
    for(int i = 0; i < K163_WORDS; i++) {
        rop[i] = op[i];
    }

    for(int i = 10; i >= 6; i--) {
        uint32_t t = op[i];
        rop[i-6] = rop[i-6] ^ (t << 29);
        rop[i-5] = rop[i-5] ^ (t << 4) ^ (t << 3) ^ t ^ (t >> 3);
        rop[i-4] = rop[i-4] ^ (t >> 28) ^ (t >> 29);
    }

    uint32_t t = rop[5] >> 3;
    rop[0] = rop[0] ^ (t << 7) ^ (t << 6) ^ (t << 3) ^ t;
    rop[1] = rop[1] ^ (t >> 25) ^ (t >> 26);
    rop[5] = rop[5] & 0x7;
}

void bfK233Mod(uint32_t * op, uint32_t * rop) {
    for(int i = 0; i < K233_WORDS; i++) {
        rop[i] = op[i];
    }

    for(int i = 15; i >= 8; i--) {
        uint32_t t = op[i];
        rop[i-8] = rop[i-8] ^ (t << 23);
        rop[i-7] = rop[i-7] ^ (t >> 9);
        rop[i-5] = rop[i-5] ^ (t << 1);
        rop[i-4] = rop[i-4] ^ (t >> 31);;
    }

    uint32_t t = rop[7] >> 9;
    rop[0] = rop[0] ^ t;
    rop[2] = rop[2] ^ (t << 10);
    rop[3] = rop[3] ^ (t >> 22);
    rop[7] = rop[7] & 0x1FF;
}

void bfK283Mod(uint32_t * op, uint32_t * rop) {
    for(int i = 0; i < K283_WORDS; i++) {
        rop[i] = op[i];
    }

    for(int i = 17; i >= 9; i--) {
        uint32_t t = op[i];
        rop[i-9] = rop[i-9] ^ (t << 5) ^ (t << 10) ^ (t << 12) ^ (t << 17);
        rop[i-8] = rop[i-8] ^ (t >> 27) ^ (t >> 22) ^ (t >> 20) ^ (t >> 15);
    }

    uint32_t t = rop[8] >> 27;
    rop[0] = rop[0] ^ t ^ (t << 5) ^ (t << 7) ^ (t << 12);
    rop[8] = rop[8] & 0x7FFFFFF;
}

void bfK409Mod(uint32_t * op, uint32_t * rop) {
    for(int i = 0; i < K409_WORDS; i++) {
        rop[i] = op[i];
    }

    for(int i = 25; i >= 13; i--) {
        uint32_t t = op[i];
        rop[i-13] = rop[i-13] ^ (t << 7);
        rop[i-12] = rop[i-12] ^ (t >> 25);
        rop[i-11] = rop[i-11] ^ (t << 30);
        rop[i-10] = rop[i-10] ^ (t >> 2);
    }

    uint32_t t = rop[12] >> 25;
    rop[0] = rop[0] ^ t;
    rop[2] = rop[2] ^ (t << 23);
    rop[12] = rop[12] & 0x1FFFFFF;
}

void bfK571Mod(uint32_t * op, uint32_t * rop) {
    for(int i = 0; i < K571_WORDS; i++) {
        rop[i] = op[i];
    }

    for(int i = 35; i >= 18; i--) {
        uint32_t t = op[i];
        rop[i-18] = rop[i-18] ^ (t << 5) ^ (t << 7) ^ (t << 10) ^ (t << 15);
        rop[i-17] = rop[i-17] ^ (t >> 27) ^ (t >> 25) ^ (t >> 22) ^ (t >> 17);
    }

    uint32_t t = rop[17] >> 27;
    rop[0] = rop[0] ^ t ^ (t << 2) ^ (t << 5) ^ (t << 10);
    rop[17] = rop[17] & 0x7FFFFFF;
}

void bfMul(uint32_t * op1, uint32_t * op2, uint32_t * rop, Field * field) {
    uint32_t * intermediate = (uint32_t *)calloc(field->words_unreduced, sizeof(uint32_t));

    for(int k = BITS_IN_WORD - 1; k >= 0; k--) {
        for(int j = 0; j < field->words; j++) {
            if((1 << k) & op1[j]) {
                for(int i = 0; i < field->words; i++) {
                    intermediate[i+j] ^= op2[i];
                }
            }
        }

        if(k != 0) {
            uint32_t shiftBit = 0;
            for(int i = 0; i < field->words_unreduced; i++) {
                uint32_t newShift = (intermediate[i] & (1 << (BITS_IN_WORD - 1))) ? 1 : 0;
                intermediate[i] = (intermediate[i] << 1) | shiftBit;
                shiftBit = newShift;
            }
        }
    }

    switch(field->identifier) {
        case K163 : bfK163Mod(intermediate, rop); break;
        case K233 : bfK233Mod(intermediate, rop); break;
        case K283 : bfK283Mod(intermediate, rop); break;
        case K409 : bfK409Mod(intermediate, rop); break;
        case K571 : bfK571Mod(intermediate, rop); break;
    }
    free(intermediate);
}

void bfSqr(uint32_t * op, uint32_t * rop, Field * field) {
    uint32_t * intermediate = (uint32_t *)calloc(field->words_unreduced, sizeof(uint32_t));

    for(int i = 0; i < field->words; i++) {
        uint32_t u0 = (0xff & op[i]),
                 u1 = ((0xff00 & op[i]) >> 8),
                 u2 = ((0xff0000 & op[i]) >> 16),
                 u3 = ((0xff000000 & op[i]) >> 24);
        intermediate[2*i] = (SQR_T[u1] << 16) | SQR_T[u0];
        intermediate[2*i + 1] = (SQR_T[u3] << 16) | SQR_T[u2];
    }

    switch(field->identifier) {
        case K163 : bfK163Mod(intermediate, rop); break;
        case K233 : bfK233Mod(intermediate, rop); break;
        case K283 : bfK283Mod(intermediate, rop); break;
        case K409 : bfK409Mod(intermediate, rop); break;
        case K571 : bfK571Mod(intermediate, rop); break;
    }
    free(intermediate);
}

int bfIsOne(uint32_t * op, Field * field) {
    if(op[0] != 1) return 0;
    for(int i = 1; i < field->words; i++) {
        if(op[i] != 0) return 0;
    }
    return 1;
}

int bfDegree(uint32_t * op, Field * field) {
    for(int i = field->words-1; i >= 0; i--) {
        for(int j = BITS_IN_WORD - 1; j >= 0; j--) {
            if(op[i] & (1 << j)) return i * BITS_IN_WORD + j;
        }
    }
    return 0;
}

void bfSwap(uint32_t * op1, uint32_t * op2, Field * field) {
    uint32_t * swap = (uint32_t *)calloc(field->words, sizeof(uint32_t));

    for(int i = 0; i < field->words; i++) {
        swap[i] = op1[i];
        op1[i] = op2[i];
        op2[i] = swap[i];
    }

    free(swap);
}

void bfInv(uint32_t * op, uint32_t * rop, Field * field) {
    uint32_t * u = (uint32_t *)calloc(field->words, sizeof(uint32_t));
    uint32_t * v = (uint32_t *)calloc(field->words, sizeof(uint32_t));
    uint32_t * g1 = (uint32_t *)calloc(field->words, sizeof(uint32_t));
    g1[0] = 0x1;
    uint32_t * g2 = (uint32_t *)calloc(field->words, sizeof(uint32_t));

    for(int i = 0; i < field->words; i++) {
        u[i] = op[i];
        v[i] = field->polynomial[i];
    }

    while(!bfIsOne(u, field)) {
        int j = bfDegree(u, field) - bfDegree(v, field);

        if(j < 0) {
            bfSwap(u, v, field);
            bfSwap(g1, g2, field);
            j = -j;
        }

        int offset = j / BITS_IN_WORD;
        j = j % BITS_IN_WORD;

        uint32_t vCarry = 0, g2Carry = 0;
        for(int i = offset; i < field->words; i++) {
            uint32_t newVCarry = j ? v[i-offset] >> (BITS_IN_WORD - j) : 0,
                     newG2Carry = j ? g2[i-offset] >> (BITS_IN_WORD - j) : 0;

            u[i] = u[i] ^ ((v[i-offset] << j) | vCarry);
            g1[i] = g1[i] ^ ((g2[i-offset] << j) | g2Carry);

            vCarry = newVCarry; g2Carry = newG2Carry;
        }
    }

    for(int i = 0; i < field->words; i++) {
        rop[i] = g1[i];
    }
    free(u); free(v); free(g1); free(g2);
}

void bfPrint(uint32_t * op, Field * field) {
    int i = field->words-1;
    printf("0x%X", op[i]);

    for(i = i-1; i >= 0; i--) {
        printf("%08X", op[i]);
    }
    printf("\n");
}
