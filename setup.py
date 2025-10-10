from os import environ
from setuptools import setup, Extension  # type: ignore[import-untyped]

extra_compile_args = ["-std=c99"]
extra_link_args = []

if environ.get("COVERAGE_BUILD"):
    extra_compile_args.extend(["--coverage", "-g", "-O0"])
    extra_link_args.extend(["--coverage"])

curvemath = Extension(
    "fastecdsa.curvemath",
    include_dirs=["src/"],
    libraries=["gmp"],
    sources=["src/curveMath.c", "src/curve.c", "src/point.c"],
    extra_compile_args=extra_compile_args,
    extra_link_args=extra_link_args,
)

_ecdsa = Extension(
    "fastecdsa._ecdsa",
    include_dirs=["src/"],
    libraries=["gmp"],
    sources=["src/_ecdsa.c", "src/curveMath.c", "src/curve.c", "src/point.c"],
    extra_compile_args=extra_compile_args,
    extra_link_args=extra_link_args,
)

setup(
    ext_modules=[curvemath, _ecdsa],
)
