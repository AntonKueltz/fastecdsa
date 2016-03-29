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
    version='0.1.1.dev1',
    description='Fast elliptic curve digital signatures',
    url='https://github.com/AntonKueltz/fastecdsa',
    packages=['fastecdsa'],
    ext_modules=[curvemath, _ecdsa]
)
