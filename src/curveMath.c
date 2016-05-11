#include "curveMath.h"
#include <string.h>


int pointEqual(const Point * pointA, const Point * pointB) {
    // check x coords
    if(mpz_cmp(pointA->x, pointB->x) != 0) {
        return 0;
    }
    // check y coords
    if(mpz_cmp(pointA->y, pointB->y) != 0) {
        return 0;
    }

    return 1;
}


void pointDouble(const Point * pointP, Point * pointR, Curve * curve) {
    mpz_t numer, denom, lambda;
    mpz_inits(numer, denom, lambda, NULL);

    // calculate lambda
    mpz_mul(numer, pointP->x, pointP->x);
    mpz_mul_ui(numer, numer, 3);
    mpz_add(numer, numer, curve->a);
    mpz_mod(numer, numer, curve->p);
    mpz_mul_ui(denom, pointP->y, 2);
    mpz_mod(denom, denom, curve->p);
    mpz_invert(denom, denom, curve->p);  // TODO check status
    mpz_mul(lambda, numer, denom);
    mpz_mod(lambda, lambda, curve->p);

    // calculate resulting x coord
    mpz_mul(pointR->x, lambda, lambda);
    mpz_sub(pointR->x, pointR->x, pointP->x);
    mpz_sub(pointR->x, pointR->x, pointP->x);
    mpz_mod(pointR->x, pointR->x, curve->p);

    //calculate resulting y coord
    mpz_sub(pointR->y, pointP->x, pointR->x);
    mpz_mul(pointR->y, lambda, pointR->y);
    mpz_sub(pointR->y, pointR->y, pointP->y);
    mpz_mod(pointR->y, pointR->y, curve->p);

    mpz_clears(numer, denom, lambda, NULL);
}


void pointAdd(const Point * pointP, const Point * pointQ, Point * pointR, Curve * curve) {
    mpz_t xdiff, ydiff, lambda;
    mpz_inits(xdiff, ydiff, lambda, NULL);

    // calculate lambda
    mpz_sub(ydiff, pointQ->y, pointP->y);
    mpz_mod(ydiff, ydiff, curve->p);
    mpz_sub(xdiff, pointQ->x, pointP->x);
    mpz_mod(xdiff, xdiff, curve->p);
    mpz_invert(xdiff, xdiff, curve->p);  // TODO check status
    mpz_mul(lambda, ydiff, xdiff);
    mpz_mod(lambda, lambda, curve->p);

    // calculate resulting x coord
    mpz_mul(pointR->x, lambda, lambda);
    mpz_sub(pointR->x, pointR->x, pointP->x);
    mpz_sub(pointR->x, pointR->x, pointQ->x);
    mpz_mod(pointR->x, pointR->x, curve->p);

    //calculate resulting y coord
    mpz_sub(pointR->y, pointP->x, pointR->x);
    mpz_mul(pointR->y, lambda, pointR->y);
    mpz_sub(pointR->y, pointR->y, pointP->y);
    mpz_mod(pointR->y, pointR->y, curve->p);

    mpz_clears(xdiff, ydiff, lambda, NULL);
}


void pointMul(const Point * P, Point * R, const mpz_t d, Curve * curve) {
    Point R0, R1, tmp;
    mpz_inits(R1.x, R1.y, tmp.x, tmp.y, NULL);
    mpz_init_set(R0.x, P->x);
    mpz_init_set(R0.y, P->y);
    pointDouble(P, &R1, curve);

    char * dbits = mpz_get_str(NULL, 2, d);
    int i = 1;

    while(dbits[i] != '\0') {
        if(dbits[i] == '0') {
            mpz_set(tmp.x, R1.x);
            mpz_set(tmp.y, R1.y);
            pointAdd(&R0, &tmp, &R1, curve);
            mpz_set(tmp.x, R0.x);
            mpz_set(tmp.y, R0.y);
            pointDouble(&tmp, &R0, curve);
        }
        else {
            mpz_set(tmp.x, R0.x);
            mpz_set(tmp.y, R0.y);
            pointAdd(&R1, &tmp, &R0, curve);
            mpz_set(tmp.x, R1.x);
            mpz_set(tmp.y, R1.y);
            pointDouble(&tmp, &R1, curve);
        }

        i++;
    }

    mpz_init_set(R->x, R0.x);
    mpz_init_set(R->y, R0.y);
    mpz_clears(R0.x, R0.y, R1.x, R1.y, tmp.x, tmp.y, NULL);
}


/******************************************************************************
 PYTHON BINDINGS
 ******************************************************************************/
static PyObject * curvemath_mul(PyObject *self, PyObject *args) {
    char * x, * y, * d, * curveName;

    if (!PyArg_ParseTuple(args, "ssss", &x, &y, &d, &curveName)) {
        return NULL;
    }

    Point * point = buildPoint(x, y, 10);
    Curve * curve;
    Point result;
    mpz_t scalar;

    if(strcmp(curveName, "P192") == 0) { curve = buildP192(); }
    else if(strcmp(curveName, "P224") == 0) { curve = buildP224(); }
    else if(strcmp(curveName, "P256") == 0) { curve = buildP256(); }
    else if(strcmp(curveName, "P384") == 0) { curve = buildP384(); }
    else if(strcmp(curveName, "P521") == 0) { curve = buildP521(); }
    else if(strcmp(curveName, "secp256k1") == 0) { curve = buildSecp256k1(); }
    else { return NULL; }

    mpz_init_set_str(scalar, d, 10);

    pointMul(point, &result, scalar, curve);
    char * resultX = mpz_get_str(NULL, 10, result.x);
    char * resultY = mpz_get_str(NULL, 10, result.y);

    destroyPoint(point);
    destroyCurve(curve);
    mpz_clears(result.x, result.y, scalar, NULL);

    return Py_BuildValue("ss", resultX, resultY);
}


static PyMethodDef curvemath__methods__[] = {
    {"mul",  curvemath_mul, METH_VARARGS,
     "Multiply a curve point by an integer scalar."},
    {NULL, NULL, 0, NULL}        /* Sentinel */
};


PyMODINIT_FUNC initcurvemath(void) {
    (void) Py_InitModule("curvemath", curvemath__methods__);
}
