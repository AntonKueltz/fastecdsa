#include "curveMath.h"
#include "binaryField.h"
#include <string.h>

int pointZZ_pEqual(const PointZZ_p * op1, const PointZZ_p * op2) {
    // check x coords
    if(mpz_cmp(op1->x, op2->x) != 0) {
        return 0;
    }
    // check y coords
    if(mpz_cmp(op1->y, op2->y) != 0) {
        return 0;
    }

    return 1;
}


int pointZZ_pXEqual(const PointZZ_pX * op1, const PointZZ_pX * op2) {
    // check x coords
    if(!f2m_equal(op1->x, op2->x)) {
        return 0;
    }
    // check y coords
    if(!f2m_equal(op1->y, op2->y)) {
        return 0;
    }

    return 1;
}


void pointZZ_pDouble(PointZZ_p * rop, const PointZZ_p * op, const CurveZZ_p * curve) {
    mpz_t numer, denom, lambda;
    mpz_inits(numer, denom, lambda, NULL);

    // calculate lambda
    mpz_mul(numer, op->x, op->x);
    mpz_mul_ui(numer, numer, 3);
    mpz_add(numer, numer, curve->a);
    mpz_mod(numer, numer, curve->p);
    mpz_mul_ui(denom, op->y, 2);
    mpz_mod(denom, denom, curve->p);
    mpz_invert(denom, denom, curve->p);  // TODO check status
    mpz_mul(lambda, numer, denom);
    mpz_mod(lambda, lambda, curve->p);

    // calculate resulting x coord
    mpz_mul(rop->x, lambda, lambda);
    mpz_sub(rop->x, rop->x, op->x);
    mpz_sub(rop->x, rop->x, op->x);
    mpz_mod(rop->x, rop->x, curve->p);

    //calculate resulting y coord
    mpz_sub(rop->y, op->x, rop->x);
    mpz_mul(rop->y, lambda, rop->y);
    mpz_sub(rop->y, rop->y, op->y);
    mpz_mod(rop->y, rop->y, curve->p);

    mpz_clears(numer, denom, lambda, NULL);
}


void pointZZ_pXDouble(PointZZ_pX * rop, const PointZZ_pX * op, const CurveZZ_pX * curve) {
    // calculate lambda
    BinaryField * xinv = f2m_invmod(op->x, curve->pt);
    BinaryField * tmp = f2m_mulmod(op->y, xinv, curve->degree);
    BinaryField * lambda = f2m_add(op->x, tmp);
    f2m_clear(xinv);
    f2m_clear(tmp);

    // calculate x coordinate
    tmp = f2m_mulmod(lambda, lambda, curve->degree);
    rop->x = f2m_add(tmp, lambda);
    tmp = f2m_add(rop->x, curve->a);
    f2m_clear(rop->x);
    rop->x = tmp;

    // calculate y coordinate
    BinaryField * x2 = f2m_mulmod(op->x, op->x, curve->degree);
    tmp = f2m_mulmod(lambda, rop->x, curve->degree);
    rop->y = f2m_add(x2, tmp);
    tmp = f2m_add(rop->y, rop->x);
    f2m_clear(rop->y);
    rop->y = tmp;
}


void pointZZ_pAdd(PointZZ_p * rop, const PointZZ_p * op1, const PointZZ_p * op2, const CurveZZ_p * curve) {
    mpz_t xdiff, ydiff, lambda;
    mpz_inits(xdiff, ydiff, lambda, NULL);

    // calculate lambda
    mpz_sub(ydiff, op2->y, op1->y);
    mpz_mod(ydiff, ydiff, curve->p);
    mpz_sub(xdiff, op2->x, op1->x);
    mpz_mod(xdiff, xdiff, curve->p);
    mpz_invert(xdiff, xdiff, curve->p);  // TODO check status
    mpz_mul(lambda, ydiff, xdiff);
    mpz_mod(lambda, lambda, curve->p);

    // calculate resulting x coord
    mpz_mul(rop->x, lambda, lambda);
    mpz_sub(rop->x, rop->x, op1->x);
    mpz_sub(rop->x, rop->x, op2->x);
    mpz_mod(rop->x, rop->x, curve->p);

    //calculate resulting y coord
    mpz_sub(rop->y, op1->x, rop->x);
    mpz_mul(rop->y, lambda, rop->y);
    mpz_sub(rop->y, rop->y, op1->y);
    mpz_mod(rop->y, rop->y, curve->p);

    mpz_clears(xdiff, ydiff, lambda, NULL);
}


void pointZZ_pXAdd(PointZZ_pX * rop, const PointZZ_pX * op1, const PointZZ_pX * op2, const CurveZZ_pX * curve) {
    // calculate lambda
    BinaryField * tmp1 = f2m_add(op1->x, op2->x);
    BinaryField * tmp2 = f2m_invmod(tmp1, curve->pt);
    BinaryField * tmp3 = f2m_add(op1->y, op2->y);
    BinaryField * lambda = f2m_mulmod(tmp3, tmp2, curve->degree);
    f2m_clear(tmp1);
    f2m_clear(tmp2);
    f2m_clear(tmp3);

    // calculate x coordinate
    tmp1 = f2m_mulmod(lambda, lambda, curve->degree);
    tmp2 = f2m_add(tmp1, lambda);
    f2m_clear(tmp1);
    tmp1 = f2m_add(tmp2, op1->x);
    f2m_clear(tmp2);
    tmp2 = f2m_add(tmp1, op2->x);
    f2m_clear(tmp1);
    rop->x = f2m_add(tmp2, curve->a);
    f2m_clear(tmp2);

    // calculate y coordinate
    tmp1 = f2m_add(op1->x, rop->x);
    tmp2 = f2m_mulmod(lambda, tmp1, curve->degree);
    f2m_clear(tmp1);
    tmp1 = f2m_add(tmp2, rop->x);
    f2m_clear(tmp2);
    rop->y = f2m_add(tmp1, op1->y);
    f2m_clear(tmp1);
}


void pointZZ_pMul(PointZZ_p * rop, const PointZZ_p * point, const mpz_t scalar, const CurveZZ_p * curve) {
    PointZZ_p R0, R1, tmp;
    mpz_inits(R1.x, R1.y, tmp.x, tmp.y, NULL);
    mpz_init_set(R0.x, point->x);
    mpz_init_set(R0.y, point->y);
    pointZZ_pDouble(&R1, point, curve);

    char * dbits = mpz_get_str(NULL, 2, scalar);
    int i = 1;

    while(dbits[i] != '\0') {
        if(dbits[i] == '0') {
            mpz_set(tmp.x, R1.x);
            mpz_set(tmp.y, R1.y);
            pointZZ_pAdd(&R1, &R0, &tmp, curve);
            mpz_set(tmp.x, R0.x);
            mpz_set(tmp.y, R0.y);
            pointZZ_pDouble(&R0, &tmp, curve);
        }
        else {
            mpz_set(tmp.x, R0.x);
            mpz_set(tmp.y, R0.y);
            pointZZ_pAdd(&R0, &R1, &tmp, curve);
            mpz_set(tmp.x, R1.x);
            mpz_set(tmp.y, R1.y);
            pointZZ_pDouble(&R1, &tmp, curve);
        }

        i++;
    }

    mpz_init_set(rop->x, R0.x);
    mpz_init_set(rop->y, R0.y);
    mpz_clears(R0.x, R0.y, R1.x, R1.y, tmp.x, tmp.y, NULL);
}


void pointZZ_pXMul(PointZZ_pX * rop, const PointZZ_pX * point, const mpz_t scalar, const CurveZZ_pX * curve) {
    PointZZ_pX R0, R1;
    R0.x = f2m_copy(point->x);
    R0.y = f2m_copy(point->y);
    pointZZ_pXDouble(&R1, point, curve);

    char * dbits = mpz_get_str(NULL, 2, scalar);
    int i = 1;

    while(dbits[i] != '\0') {
        PointZZ_pX tmp;

        if(dbits[i] == '0') {
            tmp.x = f2m_copy(R1.x);
            tmp.y = f2m_copy(R1.y);
            pointZZ_pXAdd(&R1, &R0, &tmp, curve);
            f2m_clear(tmp.x);
            f2m_clear(tmp.y);
            tmp.x = f2m_copy(R0.x);
            tmp.y = f2m_copy(R0.y);
            pointZZ_pXDouble(&R0, &tmp, curve);
        }
        else {
            tmp.x = f2m_copy(R0.x);
            tmp.y = f2m_copy(R0.y);
            pointZZ_pXAdd(&R0, &R1, &tmp, curve);
            f2m_clear(tmp.x);
            f2m_clear(tmp.y);
            tmp.x = f2m_copy(R1.x);
            tmp.y = f2m_copy(R1.y);
            pointZZ_pXDouble(&R1, &tmp, curve);
        }

        destroyPointZZ_pX(&tmp);
        i++;
    }

    rop->x = f2m_copy(R0.x);
    rop->y = f2m_copy(R0.y);

    destroyPointZZ_pX(&R0);
    destroyPointZZ_pX(&R1);
}


/******************************************************************************
 PYTHON BINDINGS
 ******************************************************************************/
static PyObject * curvemath_mul(PyObject *self, PyObject *args) {
    char * x, * y, * d, * curveName;

    if (!PyArg_ParseTuple(args, "ssss", &x, &y, &d, &curveName)) {
        return NULL;
    }

    PointZZ_p result;
    mpz_t scalar;
    mpz_init_set_str(scalar, d, 10);

    void * curve;
    unsigned binaryField = 0, degree = 0;

    if(strcmp(curveName, "P192") == 0) { curve = buildP192(); }
    else if(strcmp(curveName, "P224") == 0) { curve = buildP224(); }
    else if(strcmp(curveName, "P256") == 0) { curve = buildP256(); }
    else if(strcmp(curveName, "P384") == 0) { curve = buildP384(); }
    else if(strcmp(curveName, "P521") == 0) { curve = buildP521(); }
    else if(strcmp(curveName, "secp256k1") == 0) { curve = buildSecp256k1(); }
    else if(strcmp(curveName, "K163") == 0) { curve = buildK163(); binaryField = 1; degree = 163; }
    else if(strcmp(curveName, "K233") == 0) { curve = buildK233(); binaryField = 1; degree = 233; }
    else if(strcmp(curveName, "K283") == 0) { curve = buildK283(); binaryField = 1; degree = 283; }
    else if(strcmp(curveName, "K409") == 0) { curve = buildK409(); binaryField = 1; degree = 409; }
    else if(strcmp(curveName, "K571") == 0) { curve = buildK571(); binaryField = 1; degree = 571; }
    else { return NULL; }

    if(binaryField) {
        PointZZ_pX r;
        PointZZ_pX * point = buildPointZZ_pX(x, y, 10, degree);
        pointZZ_pXMul(&r, point, scalar, (CurveZZ_pX *)curve);

        mpz_inits(result.x, result.y, NULL);
        f2m_to_mpz(result.x, r.x);
        f2m_to_mpz(result.y, r.y);

        destroyPointZZ_pX(&r);
        destroyCurveZZ_pX((CurveZZ_pX *)curve);
    }
    else {
        PointZZ_p * point = buildPointZZ_p(x, y, 10);
        pointZZ_pMul(&result, point, scalar, (CurveZZ_p *)curve);

        destroyPointZZ_p(point);
        destroyCurveZZ_p((CurveZZ_p *)curve);
    }

    char * resultX = mpz_get_str(NULL, 10, result.x);
    char * resultY = mpz_get_str(NULL, 10, result.y);
    mpz_clears(result.x, result.y, scalar, NULL);
    return Py_BuildValue("ss", resultX, resultY);
}

static PyObject * curvemath_add(PyObject *self, PyObject *args) {
    char * px, * py, * qx, * qy, * curveName;

    if (!PyArg_ParseTuple(args, "sssss", &px, &py, &qx, &qy, &curveName)) {
        return NULL;
    }

    PointZZ_p result;
    mpz_inits(result.x, result.y, NULL);

    void * curve;
    unsigned binaryField = 0, degree = 0;

    if(strcmp(curveName, "P192") == 0) { curve = buildP192(); }
    else if(strcmp(curveName, "P224") == 0) { curve = buildP224(); }
    else if(strcmp(curveName, "P256") == 0) { curve = buildP256(); }
    else if(strcmp(curveName, "P384") == 0) { curve = buildP384(); }
    else if(strcmp(curveName, "P521") == 0) { curve = buildP521(); }
    else if(strcmp(curveName, "secp256k1") == 0) { curve = buildSecp256k1(); }
    else if(strcmp(curveName, "K163") == 0) { curve = buildK163(); binaryField = 1; degree = 163; }
    else if(strcmp(curveName, "K233") == 0) { curve = buildK233(); binaryField = 1; degree = 233; }
    else if(strcmp(curveName, "K283") == 0) { curve = buildK283(); binaryField = 1; degree = 283; }
    else if(strcmp(curveName, "K409") == 0) { curve = buildK409(); binaryField = 1; degree = 409; }
    else if(strcmp(curveName, "K571") == 0) { curve = buildK571(); binaryField = 1; degree = 571; }
    else { return NULL; }

    if(binaryField) {
        PointZZ_pX * p = buildPointZZ_pX(px, py, 10, degree);
        PointZZ_pX * q = buildPointZZ_pX(qx, qy, 10, degree);
        PointZZ_pX r;

        if(pointZZ_pXEqual(p, q)) {
            pointZZ_pXDouble(&r, p, (CurveZZ_pX *)curve);
        }
        else {
            pointZZ_pXAdd(&r, p, q, (CurveZZ_pX *)curve);
        }

        f2m_to_mpz(result.x, r.x);
        f2m_to_mpz(result.y, r.y);

        destroyPointZZ_pX(p);
        destroyPointZZ_pX(q);
        destroyPointZZ_pX(&r);
        destroyCurveZZ_pX((CurveZZ_pX *)curve);
    }
    else {
        PointZZ_p * p = buildPointZZ_p(px, py, 10);
        PointZZ_p * q = buildPointZZ_p(qx, qy, 10);

        if(pointZZ_pEqual(p, q)) {
            pointZZ_pDouble(&result, p, (CurveZZ_p *)curve);
        }
        else {
            pointZZ_pAdd(&result, p, q, (CurveZZ_p *)curve);
        }

        destroyPointZZ_p(p);
        destroyPointZZ_p(q);
        destroyCurveZZ_p((CurveZZ_p *)curve);
    }

    char * resultX = mpz_get_str(NULL, 10, result.x);
    char * resultY = mpz_get_str(NULL, 10, result.y);
    mpz_clears(result.x, result.y, NULL);
    return Py_BuildValue("ss", resultX, resultY);
}


static PyMethodDef curvemath__methods__[] = {
    {"mul", curvemath_mul, METH_VARARGS, "Multiply a curve point by an integer scalar."},
    {"add", curvemath_add, METH_VARARGS, "Add two points on a curve."},
    {NULL, NULL, 0, NULL}        /* Sentinel */
};


#if PY_MAJOR_VERSION >= 3
static struct PyModuleDef moduledef = {
    PyModuleDef_HEAD_INIT,
    "curvemath",            /* m_name */
    NULL,                   /* m_doc */
    -1,                     /* m_size */
    curvemath__methods__,   /* m_methods */
    NULL,                   /* m_reload */
    NULL,                   /* m_traverse */
    NULL,                   /* m_clear */
    NULL,                   /* m_free */
};


PyMODINIT_FUNC PyInit_curvemath(void) {
    PyObject * m = PyModule_Create(&moduledef);
    return m;
}


#else
PyMODINIT_FUNC initcurvemath(void) {
    Py_InitModule("curvemath", curvemath__methods__);
}
#endif
