#include "_ecdsa.h"
#include <string.h>


void sign(Sig * sig, char * msg, mpz_t d, Curve * curve) {
    // TODO - hardcoded for now to test NIST vectors. DON'T USE IN PROD OR BE LIKE SONY
    mpz_t k, e, kinv;
    mpz_init_set_str(k, "580ec00d856434334cef3f71ecaed4965b12ae37fa47055b1965c7b134ee45d0", 16);

    // R = k * G, r = R[x]
    Point R;
    pointMul(curve->g, &R, k, curve);
    mpz_init_set(sig->r, R.x);

    // convert digest to integer (digest is computed as hex in ecdsa.py)
    mpz_init_set_str(e, msg, 16);

    // s = (k^-1 * (e + d * r)) mod n
    mpz_inits(kinv, sig->s, NULL);
    mpz_invert(kinv, k, curve->q);
    mpz_mul(sig->s, d, sig->r);
    mpz_add(sig->s, sig->s, e);
    mpz_mul(sig->s, sig->s, kinv);
    mpz_mod(sig->s, sig->s, curve->q);

    mpz_clears(k, e, kinv, NULL);
}

// TODO Shamir's trick for two mults and add
// TODO validate Q, r, s
int verify(Sig * sig, char * msg, Point * Q, Curve * curve) {
    mpz_t e, w, u1, u2;
    Point tmp1, tmp2, tmp3;
    mpz_inits(w, u1, u2, tmp1.x, tmp1.y, tmp2.x, tmp2.y, tmp3.x, tmp3.y, NULL);

    // convert digest to integer (digest is computed as hex in ecdsa.py)
    mpz_init_set_str(e, msg, 16);

    mpz_invert(w, sig->s, curve->q);
    mpz_mul(u1, e, w);
    mpz_mod(u1, u1, curve->q);
    mpz_mul(u2, sig->r, w);
    mpz_mod(u2, u2, curve->q);

    pointMul(curve->g, &tmp1, u1, curve);
    pointMul(Q, &tmp2, u2, curve);
    pointAdd(&tmp1, &tmp2, &tmp3, curve);

    int equal = (mpz_cmp(tmp3.x, sig->r) == 0);
    mpz_clears(e, w, u1, u2, tmp1.x, tmp1.y, tmp2.x, tmp2.y, tmp3.x, tmp3.y, NULL);
    return equal;
}

/******************************************************************************
 PYTHON BINDINGS
 ******************************************************************************/
static PyMethodDef _ecdsa__methods__[] = {
    {"sign",  _ecdsa_sign, METH_VARARGS,
     "Sign a message via ECDSA."},
    {"verify",  _ecdsa_verify, METH_VARARGS,
     "Verify a signature via ECDSA."},
    {NULL, NULL, 0, NULL}        /* Sentinel */
};


PyMODINIT_FUNC init_ecdsa(void) {
    (void) Py_InitModule("_ecdsa", _ecdsa__methods__);
}


static PyObject * _ecdsa_sign(PyObject *self, PyObject *args) {
    char * msg, * d, * curveName;

    if (!PyArg_ParseTuple(args, "sss", &msg, &d, &curveName)) {
        return NULL;
    }

    mpz_t privKey;
    Curve * curve;
    Sig sig;

    if(strcmp(curveName, "P192") == 0) { curve = buildP192(); }
    else if(strcmp(curveName, "P256") == 0) { curve = buildP256(); }
    else { return NULL; }

    mpz_init_set_str(privKey, d, 10);

    sign(&sig, msg, privKey, curve);
    char * resultR = mpz_get_str(NULL, 10, sig.r);
    char * resultS = mpz_get_str(NULL, 10, sig.s);

    destroyCurve(curve);
    mpz_clears(sig.r, sig.s, privKey, NULL);

    return Py_BuildValue("ss", resultR, resultS);
}


static PyObject * _ecdsa_verify(PyObject *self, PyObject *args) {
    char * r, * s, * msg, * qx, * qy, * curveName;

    if (!PyArg_ParseTuple(args, "ssssss", &r, &s, &msg, &qx, &qy, &curveName)) {
        return NULL;
    }

    Point * Q = buildPoint(qx, qy, 10);
    Curve * curve;
    Sig sig;

    if(strcmp(curveName, "P192") == 0) { curve = buildP192(); }
    else if(strcmp(curveName, "P256") == 0) { curve = buildP256(); }
    else { return NULL; }

    mpz_init_set_str(sig.r, r, 10);
    mpz_init_set_str(sig.s, s, 10);

    int valid = verify(&sig, msg, Q, curve);

    destroyPoint(Q);
    destroyCurve(curve);
    mpz_clears(sig.r, sig.s, NULL);

    return Py_BuildValue("O", valid ? Py_True : Py_False);
}
