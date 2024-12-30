from setuptools import setup, Extension  # type: ignore


curvemath = Extension(
    "fastecdsa.curvemath",
    include_dirs=["src/"],
    libraries=["gmp"],
    sources=["src/curveMath.c", "src/curve.c", "src/point.c"],
    extra_compile_args=["-O2"],
)

_ecdsa = Extension(
    "fastecdsa._ecdsa",
    include_dirs=["src/"],
    libraries=["gmp"],
    sources=["src/_ecdsa.c", "src/curveMath.c", "src/curve.c", "src/point.c"],
    extra_compile_args=["-O2"],
)

setup(
    ext_modules=[curvemath, _ecdsa],
)
