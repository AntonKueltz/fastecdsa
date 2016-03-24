#include "curveMath.h"


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
    if(pointEqual(pointP, pointQ)) {
        return pointDouble(pointP, pointR, curve);
    }

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


// TODO - Montgomery ladder
void pointMul(const Point * pointP, Point * pointR, const mpz_t d, Curve * curve) {
    int isInf = 1;
    char * dbits = mpz_get_str(NULL, 2, d);

    Point p, tmp;
    mpz_inits(tmp.x, tmp.y, pointR->x, pointR->y, NULL);
    mpz_init_set(p.x, pointP->x);
    mpz_init_set(p.y, pointP->y);

    for(int i = mpz_sizeinbase(d, 2) - 1; i >= 0; i--) {
        if(dbits[i] == '1') {
            if(isInf == 1) {
                mpz_set(pointR->x, p.x);
                mpz_set(pointR->y, p.y);
                isInf = 0;
            }
            else {
                mpz_set(tmp.x, pointR->x);
                mpz_set(tmp.y, pointR->y);
                pointAdd(&p, &tmp, pointR, curve);
            }
        }

        mpz_set(tmp.x, p.x);
        mpz_set(tmp.y, p.y);
        pointDouble(&tmp, &p, curve);
    }

    mpz_clears(p.x, p.y, tmp.x, tmp.y, NULL);
}


/******************************************************************************
 PYTHON BINDINGS
 ******************************************************************************/
static PyMethodDef curvemath__methods__[] = {
    {"mul",  curvemath_mul, METH_VARARGS,
     "Multiply a curve point by an integer scalar."},
    {NULL, NULL, 0, NULL}        /* Sentinel */
};


PyMODINIT_FUNC initcurvemath(void) {
    (void) Py_InitModule("curvemath", curvemath__methods__);
}


static PyObject * curvemath_mul(PyObject *self, PyObject *args) {
    char * x, * y, * d;
    Point result;
    mpz_t scalar;

    if (!PyArg_ParseTuple(args, "sss", &x, &y, &d)) {
        return NULL;
    }

    Point * point = buildPoint(x, y, 10);
    Curve * curve = buildP256();
    mpz_init_set_str(scalar, d, 10);

    pointMul(point, &result, scalar, curve);
    char * resultX = mpz_get_str(NULL, 10, result.x);
    char * resultY = mpz_get_str(NULL, 10, result.y);

    destroyPoint(point);
    destroyCurve(curve);
    mpz_clears(result.x, result.y, scalar, NULL);

    return Py_BuildValue("ss", resultX, resultY);
}
