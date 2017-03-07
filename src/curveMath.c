#include "curveMath.h"
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


void pointZZ_pMul(PointZZ_p * rop, const PointZZ_p * point, const mpz_t scalar, const CurveZZ_p * curve) {
    PointZZ_p R0, R1, tmp;
    mpz_inits(R1.x, R1.y, tmp.x, tmp.y, NULL);
    mpz_init_set(R0.x, point->x);
    mpz_init_set(R0.y, point->y);
    pointZZ_pDouble(&R1, point, curve);

    int dbits = mpz_sizeinbase(scalar, 2), i;

    for(i = dbits - 2; i >= 0; i--) {
        if(mpz_tstbit(scalar, i)) {
            mpz_set(tmp.x, R0.x);
            mpz_set(tmp.y, R0.y);
            pointZZ_pAdd(&R0, &R1, &tmp, curve);
            mpz_set(tmp.x, R1.x);
            mpz_set(tmp.y, R1.y);
            pointZZ_pDouble(&R1, &tmp, curve);
        }
        else {
            mpz_set(tmp.x, R1.x);
            mpz_set(tmp.y, R1.y);
            pointZZ_pAdd(&R1, &R0, &tmp, curve);
            mpz_set(tmp.x, R0.x);
            mpz_set(tmp.y, R0.y);
            pointZZ_pDouble(&R0, &tmp, curve);
        }
    }

    mpz_init_set(rop->x, R0.x);
    mpz_init_set(rop->y, R0.y);
    mpz_clears(R0.x, R0.y, R1.x, R1.y, tmp.x, tmp.y, NULL);
}


void pointZZ_pShamirsTrick(PointZZ_p * rop, const PointZZ_p * point1, const mpz_t scalar1,
    const PointZZ_p * point2, const mpz_t scalar2, const CurveZZ_p * curve)
{
    PointZZ_p sum, tmp;
    mpz_inits(sum.x, sum.y, tmp.x, tmp.y, NULL);
    pointZZ_pAdd(&sum, point1, point2, curve);

    int scalar1Bits = mpz_sizeinbase(scalar1, 2);
    int scalar2Bits = mpz_sizeinbase(scalar2, 2);
    int l = (scalar1Bits > scalar2Bits ? scalar1Bits : scalar2Bits) - 1;

    if(mpz_tstbit(scalar1, l) && mpz_tstbit(scalar2, l)) {
        mpz_set(rop->x, sum.x);
        mpz_set(rop->y, sum.y);
    } else if(mpz_tstbit(scalar1, l)) {
        mpz_set(rop->x, point1->x);
        mpz_set(rop->y, point1->y);
    } else if(mpz_tstbit(scalar2, l)) {
        mpz_set(rop->x, point2->x);
        mpz_set(rop->y, point2->y);
    }

    for(l = l - 1; l >= 0; l--) {
        mpz_set(tmp.x, rop->x);
        mpz_set(tmp.y, rop->y);
        pointZZ_pDouble(rop, &tmp, curve);

        mpz_set(tmp.x, rop->x);
        mpz_set(tmp.y, rop->y);

        if(mpz_tstbit(scalar1, l) && mpz_tstbit(scalar2, l)) {
            pointZZ_pAdd(rop, &tmp, &sum, curve);
        } else if(mpz_tstbit(scalar1, l)) {
            pointZZ_pAdd(rop, &tmp, point1, curve);
        } else if(mpz_tstbit(scalar2, l)) {
            pointZZ_pAdd(rop, &tmp, point2, curve);
        }
    }

    mpz_clears(sum.x, sum.y, tmp.x, tmp.y, NULL);
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
    CurveZZ_p * curve;

    if(strcmp(curveName, "P192") == 0) { curve = buildP192(); }
    else if(strcmp(curveName, "P224") == 0) { curve = buildP224(); }
    else if(strcmp(curveName, "P256") == 0) { curve = buildP256(); }
    else if(strcmp(curveName, "P384") == 0) { curve = buildP384(); }
    else if(strcmp(curveName, "P521") == 0) { curve = buildP521(); }
    else if(strcmp(curveName, "secp256k1") == 0) { curve = buildSecp256k1(); }
    else { return NULL; }

    PointZZ_p * point = buildPointZZ_p(x, y, 10);
    pointZZ_pMul(&result, point, scalar, curve);
    destroyPointZZ_p(point);
    destroyCurveZZ_p(curve);

    char * resultX = mpz_get_str(NULL, 10, result.x);
    char * resultY = mpz_get_str(NULL, 10, result.y);
    mpz_clears(result.x, result.y, scalar, NULL);

    PyObject * ret = Py_BuildValue("ss", resultX, resultY);
    free(resultX);
    free(resultY);
    return ret;
}

static PyObject * curvemath_add(PyObject *self, PyObject *args) {
    char * px, * py, * qx, * qy, * curveName;

    if (!PyArg_ParseTuple(args, "sssss", &px, &py, &qx, &qy, &curveName)) {
        return NULL;
    }

    PointZZ_p result;
    mpz_inits(result.x, result.y, NULL);
    void * curve;

    if(strcmp(curveName, "P192") == 0) { curve = buildP192(); }
    else if(strcmp(curveName, "P224") == 0) { curve = buildP224(); }
    else if(strcmp(curveName, "P256") == 0) { curve = buildP256(); }
    else if(strcmp(curveName, "P384") == 0) { curve = buildP384(); }
    else if(strcmp(curveName, "P521") == 0) { curve = buildP521(); }
    else if(strcmp(curveName, "secp256k1") == 0) { curve = buildSecp256k1(); }
    else { return NULL; }

    PointZZ_p * p = buildPointZZ_p(px, py, 10);
    PointZZ_p * q = buildPointZZ_p(qx, qy, 10);

    if(pointZZ_pEqual(p, q)) {
        pointZZ_pDouble(&result, p, curve);
    }
    else {
        pointZZ_pAdd(&result, p, q, curve);
    }

    destroyPointZZ_p(p);
    destroyPointZZ_p(q);
    destroyCurveZZ_p(curve);

    char * resultX = mpz_get_str(NULL, 10, result.x);
    char * resultY = mpz_get_str(NULL, 10, result.y);
    mpz_clears(result.x, result.y, NULL);

    PyObject * ret = Py_BuildValue("ss", resultX, resultY);
    free(resultX);
    free(resultY);
    return ret;
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
