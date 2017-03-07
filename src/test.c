#include <stdlib.h>
#include <time.h>

#include "curveMath.h"
#include "_ecdsa.h"


void ecdsaTest(void) {
    mpz_t d, k;
    mpz_init_set_str(d, "70a12c2db16845ed56ff68cfc21a472b3f04d7d6851bf6349f2d7d5b3452b38a", 16);
    mpz_init_set_str(k, "580ec00d856434334cef3f71ecaed4965b12ae37fa47055b1965c7b134ee45d0", 16);

    char * msg = "7c3e883ddc8bd688f96eac5e9324222c8f30f9d6bb59e9c5f020bd39ba2b8377";
    CurveZZ_p * curve = buildP256();

    Sig sig;
    signZZ_p(&sig, msg, d, k, curve);
    gmp_printf("r: %Zx\ns: %Zx\n", sig.r, sig.s);

    mpz_init_set_str(sig.r, "7214bc9647160bbd39ff2f80533f5dc6ddd70ddf86bb815661e805d5d4e6f27c", 16);
    mpz_init_set_str(sig.s, "7d1ff961980f961bdaa3233b6209f4013317d3e3f9e1493592dbeaa1af2bc367", 16);
    PointZZ_p * Q = buildPointZZ_p(
        "8101ece47464a6ead70cf69a6e2bd3d88691a3262d22cba4f7635eaff26680a8",
        "d8a12ba61d599235f67d9cb4d58f1783d3ca43e78f0a5abaa624079936c0c3a9",
        16
    );
    int equal = verifyZZ_p(&sig, msg, Q, curve);
    printf("%s\n", equal ? "True" : "False");

    mpz_set_str(d, "C9AFA9D845BA75166B5C215767B1D6934E50C3DB36E89B127B8A622B120F6721", 16);
    mpz_set_str(k, "5FA81C63109BADB88C1F367B47DA606DA28CAD69AA22C4FE6AD7DF73A7173AA5", 16);

    char * msg2 = "39a5e04aaff7455d9850c605364f514c11324ce64016960d23d5dc57d3ffd8f49a739468ab8049bf18eef820cdb1ad6c9015f838556bc7fad4138b23fdf986c7";

    signZZ_p(&sig, msg2, d, k, curve);
    gmp_printf("r: %Zx\ns: %Zx\n", sig.r, sig.s);

    mpz_clears(sig.r, sig.s, d, k, NULL);
    destroyCurveZZ_p(curve);
    destroyPointZZ_p(Q);
}


void pythonTest(void) {
    char * x, * y, * d;
    PointZZ_p result;
    mpz_t scalar;

    x = "100477533340815411662634551128749658785907042636435106397366501380429417453513";
    y = "87104997799923409786648856004022766656120419079854375215656946413621911659094";
    d = "89159128863034313675150798691418246016730671603224848136445263738857221457661";

    PointZZ_p * point = buildPointZZ_p(x, y, 10);
    CurveZZ_p * curve = buildP256();
    mpz_init_set_str(scalar, d, 10);

    pointZZ_pMul(&result, point, scalar, curve);
    gmp_printf("(%Zx,\n %Zx)\n", result.x, result.y);

    destroyPointZZ_p(point);
    destroyCurveZZ_p(curve);
    mpz_clears(result.x, result.y, scalar, NULL);
}


void p256Test(void) {
    PointZZ_p r, s, t;
    mpz_t d, e;

    // https://www.nsa.gov/ia/_files/nist-routines.pdf
    CurveZZ_p * c = buildP256();
    PointZZ_p * p = buildPointZZ_p(
        "100477533340815411662634551128749658785907042636435106397366501380429417453513",
        "87104997799923409786648856004022766656120419079854375215656946413621911659094",
        10
    );
    PointZZ_p * q = buildPointZZ_p(
        "38744637563132252572193375526521585173096338380822965394069276390274998769771",
        "38053931953835384495674052639602881660154657110782968445504801383088376660758",
        10
    );


    mpz_init_set_str(d, "c51e4753afdec1e6b6c6a5b992f43f8dd0c7a8933072708b6522468b2ffb06fd", 16);
    mpz_init_set_str(e, "d37f628ece72a462f0145cbefe3f0b355ee8332d37acdd83a358016aea029db7", 16);
    mpz_inits(r.x, r.y, s.x, s.y, t.x, t.y, NULL);

    pointZZ_pAdd(&r, p, q, c);
    gmp_printf("(%Zx,\n %Zx)\n", r.x, r.y);

    pointZZ_pDouble(&r, p, c);
    gmp_printf("(%Zx,\n %Zx)\n", r.x, r.y);

    pointZZ_pMul(&r, p, d, c);
    gmp_printf("(%Zx,\n %Zx)\n", r.x, r.y);

    pointZZ_pMul(&r, p, d, c);
    pointZZ_pMul(&s, q, e, c);
    pointZZ_pAdd(&t, &r, &s, c);
    gmp_printf("(%Zx,\n %Zx)\n", t.x, t.y);

    destroyCurveZZ_p(c);
    destroyPointZZ_p(p);
    destroyPointZZ_p(q);
    mpz_clears(r.x, r.y, s.x, s.y, t.x, t.y, d, e, NULL);
}

void secp256k1Test(void) {
    PointZZ_p r;
    mpz_t d;

    PointZZ_p * g = buildPointZZ_p(
        "79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798",
        "483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8",
        16
    );
    CurveZZ_p * c = buildSecp256k1();
    // param from http://crypto.stackexchange.com/a/787/17884
    mpz_init_set_str(d, "AA5E28D6A97A2479A65527F7290311A3624D4CC0FA1578598EE3C2613BF99522", 16);
    mpz_inits(r.x, r.y, NULL);

    pointZZ_pMul(&r, g, d, c);
    gmp_printf("(%ZX,\n %ZX)\n", r.x, r.y);

    destroyCurveZZ_p(c);
    destroyPointZZ_p(g);
    mpz_clears(d, r.x, r.y, NULL);
}


int main(int argc, char * argv[]) {
    ecdsaTest();
    secp256k1Test();
    return 0;
}
