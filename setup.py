from distutils.core import setup, Extension

module1 = Extension('fastecdsa',
                    sources=['fastecdsamodule.c', 'curveMath.c', 'curve.c', 'point.c'],
                    extra_compile_args=['-std=c99', '-O2'],
                    extra_link_args=['-lgmp'])

setup(name='fastecdsa',
      version='0.1',
      description='Fast elliptic curve arithmetic',
      ext_modules=[module1])
