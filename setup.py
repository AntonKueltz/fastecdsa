from distutils.core import setup, Extension


curvemath = Extension(
    'fastecdsa.curvemath',
    include_dirs=['src/'],
    libraries=['gmp'],
    sources=['src/curveMath.c', 'src/curve.c', 'src/point.c'],
    extra_compile_args=['-std=c99', '-O2']
)

_ecdsa = Extension(
    'fastecdsa._ecdsa',
    include_dirs=['src/'],
    libraries=['gmp'],
    sources=['src/_ecdsa.c', 'src/curveMath.c', 'src/curve.c', 'src/point.c'],
    extra_compile_args=['-std=c99', '-O2']
)

setup(
    name='fastecdsa',
    version='0.1',
    description='Fast elliptic curve arithmetic',
    packages=['fastecdsa'],
    ext_modules=[curvemath, _ecdsa]
)
