#include "fastecdsamodule.h"
#include <gmp.h>
#include "curveMath.h"

static PyMethodDef FastEcdsaMethods[] = {
    {"mul",  fastecdsa_mul, METH_VARARGS,
     "Multiply a curve point by an integer scalar."},
    {NULL, NULL, 0, NULL}        /* Sentinel */
};

PyMODINIT_FUNC initfastecdsa(void) {
    (void) Py_InitModule("fastecdsa", FastEcdsaMethods);
}

static PyObject * fastecdsa_mul(PyObject *self, PyObject *args) {
    char * x, * y, * d;
    Point result;
    mpz_t scalar;

    if (!PyArg_ParseTuple(args, "sss", &x, &y, &d)) {
        return NULL;
    }

    Point * point = buildPoint(x, y, 10);
    Curve * curve = buildCurve(  // Only support P256 for now
        "115792089210356248762697446949407573530086143415290314195533631308867097853951",
        "-3",
        "41058363725152142129326129780047268409114441015993725554835256314039467401291",
        "115792089210356248762697446949407573529996955224135760342422259061068512044369",
        10
    );
    mpz_init_set_str(scalar, d, 10);

    pointMul(point, &result, scalar, curve);
    char * resultX = mpz_get_str(NULL, 10, result.x);
    char * resultY = mpz_get_str(NULL, 10, result.y);

    destroyPoint(point);
    destroyCurve(curve);
    mpz_clears(result.x, result.y, scalar, NULL);

    return Py_BuildValue("ss", resultX, resultY);
}
