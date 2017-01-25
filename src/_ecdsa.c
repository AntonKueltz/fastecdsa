#include "_ecdsa.h"
#include <string.h>
#include <stdio.h>


void signZZ_p(Sig * sig, char * msg, mpz_t d, mpz_t k, const CurveZZ_p * curve) {
    mpz_t e, kinv;

    // R = k * G, r = R[x]
    PointZZ_p R;
    pointZZ_pMul(&R, curve->g, k, curve);
    mpz_init_set(sig->r, R.x);
    mpz_mod(sig->r, sig->r, curve->q);

    // convert digest to integer (digest is computed as hex in ecdsa.py)
    mpz_init_set_str(e, msg, 16);
    int orderBits = mpz_sizeinbase(curve->q, 2);
    int digestBits = strlen(msg) * 4;

    if(digestBits > orderBits) {
        mpz_fdiv_q_2exp(e, e, digestBits - orderBits);
    }

    // s = (k^-1 * (e + d * r)) mod n
    mpz_inits(kinv, sig->s, NULL);
    mpz_invert(kinv, k, curve->q);
    mpz_mul(sig->s, d, sig->r);
    mpz_add(sig->s, sig->s, e);
    mpz_mul(sig->s, sig->s, kinv);
    mpz_mod(sig->s, sig->s, curve->q);

    mpz_clears(R.x, R.y, e, kinv, NULL);
}


void signZZ_pX(Sig * sig, char * msg, mpz_t d, mpz_t k, const CurveZZ_pX * curve) {
    mpz_t e, kinv;

    // R = k * G, r = R[x]
    PointZZ_pX R;
    pointZZ_pXMul(&R, curve->g, k, curve);
    mpz_init(sig->r);
    f2m_to_mpz(sig->r, R.x);
    mpz_mod(sig->r, sig->r, curve->q);

    // convert digest to integer (digest is computed as hex in ecdsa.py)
    mpz_init_set_str(e, msg, 16);
    int orderBits = mpz_sizeinbase(curve->q, 2);
    int digestBits = strlen(msg) * 4;

    if(digestBits > orderBits) {
        mpz_fdiv_q_2exp(e, e, digestBits - orderBits);
    }

    // s = (k^-1 * (e + d * r)) mod n
    mpz_inits(kinv, sig->s, NULL);
    mpz_invert(kinv, k, curve->q);
    mpz_mul(sig->s, d, sig->r);
    mpz_add(sig->s, sig->s, e);
    mpz_mul(sig->s, sig->s, kinv);
    mpz_mod(sig->s, sig->s, curve->q);

    destroyPointZZ_pX(&R);
    mpz_clears(e, kinv, NULL);
}


// TODO Shamir's trick for two mults and add
// TODO validate Q, r, s
int verifyZZ_p(Sig * sig, char * msg, PointZZ_p * Q, const CurveZZ_p * curve) {
    mpz_t e, w, u1, u2;
    PointZZ_p tmp1, tmp2, tmp3;
    mpz_inits(w, u1, u2, tmp1.x, tmp1.y, tmp2.x, tmp2.y, tmp3.x, tmp3.y, NULL);

    // convert digest to integer (digest is computed as hex in ecdsa.py)
    mpz_init_set_str(e, msg, 16);
    int orderBits = mpz_sizeinbase(curve->q, 2);
    int digestBits = strlen(msg) * 4;

    if(digestBits > orderBits) {
        mpz_fdiv_q_2exp(e, e, digestBits - orderBits);
    }

    mpz_invert(w, sig->s, curve->q);
    mpz_mul(u1, e, w);
    mpz_mod(u1, u1, curve->q);
    mpz_mul(u2, sig->r, w);
    mpz_mod(u2, u2, curve->q);

    pointZZ_pMul(&tmp1, curve->g, u1, curve);
    pointZZ_pMul(&tmp2, Q, u2, curve);
    pointZZ_pAdd(&tmp3, &tmp1, &tmp2, curve);
    mpz_mod(tmp3.x, tmp3.x, curve->q);

    int equal = (mpz_cmp(tmp3.x, sig->r) == 0);
    mpz_clears(e, w, u1, u2, tmp1.x, tmp1.y, tmp2.x, tmp2.y, tmp3.x, tmp3.y, NULL);
    return equal;
}


int verifyZZ_pX(Sig * sig, char * msg, PointZZ_pX * Q, const CurveZZ_pX * curve) {
    mpz_t e, w, u1, u2, x;
    PointZZ_pX tmp1, tmp2, tmp3;
    mpz_inits(w, u1, u2, x, NULL);

    // convert digest to integer (digest is computed as hex in ecdsa.py)
    mpz_init_set_str(e, msg, 16);
    int orderBits = mpz_sizeinbase(curve->q, 2);
    int digestBits = strlen(msg) * 4;

    if(digestBits > orderBits) {
        mpz_fdiv_q_2exp(e, e, digestBits - orderBits);
    }

    mpz_invert(w, sig->s, curve->q);
    mpz_mul(u1, e, w);
    mpz_mod(u1, u1, curve->q);
    mpz_mul(u2, sig->r, w);
    mpz_mod(u2, u2, curve->q);

    pointZZ_pXMul(&tmp1, curve->g, u1, curve);
    pointZZ_pXMul(&tmp2, Q, u2, curve);
    pointZZ_pXAdd(&tmp3, &tmp1, &tmp2, curve);
    f2m_to_mpz(x, tmp3.x);
    mpz_mod(x, x, curve->q);

    int equal = (mpz_cmp(x, sig->r) == 0);
    destroyPointZZ_pX(&tmp1);
    destroyPointZZ_pX(&tmp2);
    destroyPointZZ_pX(&tmp3);
    mpz_clears(e, w, u1, u2, x, NULL);
    return equal;
}


/******************************************************************************
 PYTHON BINDINGS
 ******************************************************************************/
static PyObject * _ecdsa_sign(PyObject *self, PyObject *args) {
    char * msg, * d, * k, * curveName;

    if (!PyArg_ParseTuple(args, "ssss", &msg, &d, &k, &curveName)) {
        return NULL;
    }

    mpz_t privKey, nonce;
    void * curve;
    Sig sig;
    int binaryField = 0;

    if(strcmp(curveName, "P192") == 0) { curve = buildP192(); }
    else if(strcmp(curveName, "P224") == 0) { curve = buildP224(); }
    else if(strcmp(curveName, "P256") == 0) { curve = buildP256(); }
    else if(strcmp(curveName, "P384") == 0) { curve = buildP384(); }
    else if(strcmp(curveName, "P521") == 0) { curve = buildP521(); }
    else if(strcmp(curveName, "secp256k1") == 0) { curve = buildSecp256k1(); }
    else if(strcmp(curveName, "K163") == 0) { curve = buildK163(); binaryField = 1; }
    else if(strcmp(curveName, "K233") == 0) { curve = buildK233(); binaryField = 1; }
    else if(strcmp(curveName, "K283") == 0) { curve = buildK283(); binaryField = 1; }
    else if(strcmp(curveName, "K409") == 0) { curve = buildK409(); binaryField = 1; }
    else if(strcmp(curveName, "K571") == 0) { curve = buildK571(); binaryField = 1; }
    else { return NULL; }

    mpz_init_set_str(privKey, d, 10);
    mpz_init_set_str(nonce, k, 10);

    if(binaryField) {
        signZZ_pX(&sig, msg, privKey, nonce, (CurveZZ_pX *)curve);
        destroyCurveZZ_pX((CurveZZ_pX *)curve);
    }
    else {
        signZZ_p(&sig, msg, privKey, nonce, (CurveZZ_p *)curve);
        destroyCurveZZ_p((CurveZZ_p *)curve);
    }

    char * resultR = mpz_get_str(NULL, 10, sig.r);
    char * resultS = mpz_get_str(NULL, 10, sig.s);
    mpz_clears(sig.r, sig.s, privKey, NULL);

    return Py_BuildValue("ss", resultR, resultS);
}


static PyObject * _ecdsa_verify(PyObject *self, PyObject *args) {
    char * r, * s, * msg, * qx, * qy, * curveName;

    if (!PyArg_ParseTuple(args, "ssssss", &r, &s, &msg, &qx, &qy, &curveName)) {
        return NULL;
    }

    Sig sig;
    mpz_init_set_str(sig.r, r, 10);
    mpz_init_set_str(sig.s, s, 10);

    void * curve;
    int binaryField = 0, valid = 0;

    if(strcmp(curveName, "P192") == 0) { curve = buildP192(); }
    else if(strcmp(curveName, "P224") == 0) { curve = buildP224(); }
    else if(strcmp(curveName, "P256") == 0) { curve = buildP256(); }
    else if(strcmp(curveName, "P384") == 0) { curve = buildP384(); }
    else if(strcmp(curveName, "P521") == 0) { curve = buildP521(); }
    else if(strcmp(curveName, "secp256k1") == 0) { curve = buildSecp256k1(); }
    else if(strcmp(curveName, "K163") == 0) { curve = buildK163(); binaryField = 1; }
    else if(strcmp(curveName, "K233") == 0) { curve = buildK233(); binaryField = 1; }
    else if(strcmp(curveName, "K283") == 0) { curve = buildK283(); binaryField = 1; }
    else if(strcmp(curveName, "K409") == 0) { curve = buildK409(); binaryField = 1; }
    else if(strcmp(curveName, "K571") == 0) { curve = buildK571(); binaryField = 1; }
    else { return NULL; }

    if(binaryField) {
        PointZZ_pX * Q = buildPointZZ_pX(qx, qy, 10, ((CurveZZ_pX *)curve)->degree);
        valid = verifyZZ_pX(&sig, msg, Q, (CurveZZ_pX *)curve);

        destroyCurveZZ_pX((CurveZZ_pX *)curve);
        destroyPointZZ_pX(Q);
    }
    else {
        PointZZ_p * Q = buildPointZZ_p(qx, qy, 10);
        valid = verifyZZ_p(&sig, msg, Q, (CurveZZ_p *)curve);

        destroyCurveZZ_p((CurveZZ_p *)curve);
        destroyPointZZ_p(Q);
    }

    mpz_clears(sig.r, sig.s, NULL);
    return Py_BuildValue("O", valid ? Py_True : Py_False);
}


static PyMethodDef _ecdsa__methods__[] = {
    {"sign", _ecdsa_sign, METH_VARARGS, "Sign a message via ECDSA."},
    {"verify", _ecdsa_verify, METH_VARARGS, "Verify a signature via ECDSA."},
    {NULL, NULL, 0, NULL}        /* Sentinel */
};


#if PY_MAJOR_VERSION >= 3
static struct PyModuleDef moduledef = {
    PyModuleDef_HEAD_INIT,
    "_ecdsa",            /* m_name */
    NULL,                   /* m_doc */
    -1,                     /* m_size */
    _ecdsa__methods__,   /* m_methods */
    NULL,                   /* m_reload */
    NULL,                   /* m_traverse */
    NULL,                   /* m_clear */
    NULL,                   /* m_free */
};

PyMODINIT_FUNC PyInit__ecdsa(void) {
    PyObject * m = PyModule_Create(&moduledef);
    return m;
}


#else
PyMODINIT_FUNC init_ecdsa(void) {
    Py_InitModule("_ecdsa", _ecdsa__methods__);
}
#endif
