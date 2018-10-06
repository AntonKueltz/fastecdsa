#include "_ecdsa.h"
#include <string.h>
#include <stdio.h>


void signZZ_p(Sig * sig, char * msg, mpz_t d, mpz_t k, const CurveZZ_p * curve) {
    mpz_t e, kinv;
	int orderBits, digestBits;
    // R = k * G, r = R[x]
    PointZZ_p R;
    pointZZ_pMul(&R, curve->g, k, curve);
    mpz_init_set(sig->r, R.x);
    mpz_mod(sig->r, sig->r, curve->q);

    // convert digest to integer (digest is computed as hex in ecdsa.py)
    mpz_init_set_str(e, msg, 16);
    orderBits = mpz_sizeinbase(curve->q, 2);
    digestBits = strlen(msg) * 4;

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


int verifyZZ_p(Sig * sig, char * msg, PointZZ_p * Q, const CurveZZ_p * curve) {
    mpz_t e, w, u1, u2;
    PointZZ_p tmp;
	int orderBits, digestBits, equal;

    mpz_inits(w, u1, u2, tmp.x, tmp.y, NULL);

    // convert digest to integer (digest is computed as hex in ecdsa.py)
    mpz_init_set_str(e, msg, 16);
    orderBits = mpz_sizeinbase(curve->q, 2);
    digestBits = strlen(msg) * 4;

    if(digestBits > orderBits) {
        mpz_fdiv_q_2exp(e, e, digestBits - orderBits);
    }

    mpz_invert(w, sig->s, curve->q);
    mpz_mul(u1, e, w);
    mpz_mod(u1, u1, curve->q);
    mpz_mul(u2, sig->r, w);
    mpz_mod(u2, u2, curve->q);

    pointZZ_pShamirsTrick(&tmp, curve->g, u1, Q, u2, curve);
    mpz_mod(tmp.x, tmp.x, curve->q);

    equal = (mpz_cmp(tmp.x, sig->r) == 0);
    mpz_clears(e, w, u1, u2, tmp.x, tmp.y, NULL);
    return equal;
}


/******************************************************************************
 PYTHON BINDINGS
 ******************************************************************************/
static PyObject * _ecdsa_sign(PyObject *self, PyObject *args) {
    char * msg, * d, * k, * p, * a, * b, * q, * gx, * gy;
	char * resultR;
	char * resultS; 
    mpz_t privKey, nonce;
    Sig sig;
	CurveZZ_p * curve;
	PyObject * ret;
    if (!PyArg_ParseTuple(args, "sssssssss", &msg, &d, &k, &p, &a, &b, &q, &gx, &gy)) {
        return NULL;
    }

    curve = buildCurveZZ_p(p, a, b, q, gx, gy, 10);

    mpz_init_set_str(privKey, d, 10);
    mpz_init_set_str(nonce, k, 10);

    signZZ_p(&sig, msg, privKey, nonce, curve);
    destroyCurveZZ_p(curve);

    resultR = mpz_get_str(NULL, 10, sig.r);
    resultS = mpz_get_str(NULL, 10, sig.s);
    mpz_clears(sig.r, sig.s, privKey, NULL);

    ret = Py_BuildValue("ss", resultR, resultS);
    free(resultR);
    free(resultS);
    return ret;
}


static PyObject * _ecdsa_verify(PyObject *self, PyObject *args) {
    char * r, * s, * msg, * qx, * qy, * p, * a, * b, * q, * gx, * gy;
    Sig sig;
	CurveZZ_p * curve;
    int valid = 0;
	PointZZ_p * Q;
	
    if (!PyArg_ParseTuple(args, "sssssssssss", &r, &s, &msg, &qx, &qy, &p, &a, &b, &q, &gx, &gy)) {
        return NULL;
    }

    mpz_init_set_str(sig.r, r, 10);
    mpz_init_set_str(sig.s, s, 10);

    curve = buildCurveZZ_p(p, a, b, q, gx, gy, 10);

    Q = buildPointZZ_p(qx, qy, 10);
    valid = verifyZZ_p(&sig, msg, Q, curve);

    destroyCurveZZ_p(curve);
    destroyPointZZ_p(Q);

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
