#include "curveMath.h"
#include "_ecdsa.h"


void timeTest() {
    Point p, r;
    Curve c;
    mpz_t d;

    mpz_init_set_str(p.x, "48439561293906451759052585252797914202762949526041747995844080717082404635286", 10);
    mpz_init_set_str(p.y, "36134250956749795798585127919587881956611106672985015071877198253568414405109", 10);
    mpz_init_set_str(c.p, "115792089210356248762697446949407573530086143415290314195533631308867097853951", 10);
    mpz_init_set_si(c.a, -3);
    mpz_inits(r.x, r.y, d, NULL);

    for(unsigned int i = 2; i < 1000; i++) {
        mpz_set_ui(d, i);
        pointMul(&p, &r, d, &c);
        gmp_printf("%Zx\n", r.x);
    }

    mpz_clears(d, r.y, r.x, c.a, c.p, p.y, p.x, NULL);
}


void ecdsaTest() {
    mpz_t d;
    mpz_init_set_str(d, "70a12c2db16845ed56ff68cfc21a472b3f04d7d6851bf6349f2d7d5b3452b38a", 16);
    char * msg = "7c3e883ddc8bd688f96eac5e9324222c8f30f9d6bb59e9c5f020bd39ba2b8377";
    Curve * curve = buildP256();

    Sig sig;
    sign(&sig, msg, d, curve);
    gmp_printf("r: %Zx\ns: %Zx\n", sig.r, sig.s);

    destroyCurve(curve);
    mpz_clears(sig.r, sig.s, d, NULL);
}


void pythonTest() {
    char * x, * y, * d;
    Point result;
    mpz_t scalar;

    x = "100477533340815411662634551128749658785907042636435106397366501380429417453513";
    y = "87104997799923409786648856004022766656120419079854375215656946413621911659094";
    d = "89159128863034313675150798691418246016730671603224848136445263738857221457661";

    Point * point = buildPoint(x, y, 10);
    Curve * curve = buildP256();
    mpz_init_set_str(scalar, d, 10);

    pointMul(point, &result, scalar, curve);
    gmp_printf("(%Zx,\n %Zx)\n", result.x, result.y);
    char * resultX = mpz_get_str(NULL, 10, result.x);
    char * resultY = mpz_get_str(NULL, 10, result.y);

    destroyPoint(point);
    destroyCurve(curve);
    mpz_clears(result.x, result.y, scalar, NULL);
}


void p256Test() {
    Point r, s, t;
    mpz_t d, e;

    // https://www.nsa.gov/ia/_files/nist-routines.pdf
    Curve * c = buildP256();
    Point * p = buildPoint(
        "100477533340815411662634551128749658785907042636435106397366501380429417453513",
        "87104997799923409786648856004022766656120419079854375215656946413621911659094",
        10
    );
    Point * q = buildPoint(
        "38744637563132252572193375526521585173096338380822965394069276390274998769771",
        "38053931953835384495674052639602881660154657110782968445504801383088376660758",
        10
    );


    mpz_init_set_str(d, "c51e4753afdec1e6b6c6a5b992f43f8dd0c7a8933072708b6522468b2ffb06fd", 16);
    mpz_init_set_str(e, "d37f628ece72a462f0145cbefe3f0b355ee8332d37acdd83a358016aea029db7", 16);
    mpz_inits(r.x, r.y, s.x, s.y, t.x, t.y, NULL);

    pointAdd(p, q, &r, c);
    gmp_printf("(%Zx,\n %Zx)\n", r.x, r.y);

    pointDouble(p, &r, c);
    gmp_printf("(%Zx,\n %Zx)\n", r.x, r.y);

    pointMul(p, &r, d, c);
    gmp_printf("(%Zx,\n %Zx)\n", r.x, r.y);

    pointMul(p, &r, d, c);
    pointMul(q, &s, e, c);
    pointAdd(&r, &s, &t, c);
    gmp_printf("(%Zx,\n %Zx)\n", t.x, t.y);

    destroyCurve(c);
    destroyPoint(p);
    destroyPoint(q);
    mpz_clears(r.x, r.y, s.x, s.y, t.x, t.y, d, e, NULL);
}


int main(int argc, char * argv[]) {
    ecdsaTest();
}
