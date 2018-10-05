from setuptools import setup, Extension, Command


class TestCommand(Command):
    user_options = []
    description = "Run all tests"

    def initialize_options(self):
        pass

    def finalize_options(self):
        pass

    def run(self):
        from subprocess import call
        call(['python', '-m', 'fastecdsa.test'])


class BenchmarkCommand(Command):
    user_options = []
    description = "Benchmark this package"

    def initialize_options(self):
        pass

    def finalize_options(self):
        pass

    def run(self):
        from subprocess import call
        call(['python', '-m', 'fastecdsa.benchmark'])


curvemath = Extension(
    'fastecdsa.curvemath',
    include_dirs=['src/'],
    libraries=['gmp'],
    sources=['src/curveMath.c', 'src/curve.c', 'src/point.c'],
    extra_compile_args=['-O2']
)

_ecdsa = Extension(
    'fastecdsa._ecdsa',
    include_dirs=['src/'],
    libraries=['gmp'],
    sources=['src/_ecdsa.c', 'src/curveMath.c', 'src/curve.c', 'src/point.c'],
    extra_compile_args=['-O2']
)

setup(
    name='fastecdsa',
    version='1.6.5',
    author='Anton Kueltz',
    author_email='kueltz.anton@gmail.com',
    license='CC0 1.0 Universal (CC0 1.0) Public Domain Dedication',
    keywords='elliptic curve cryptography ecdsa ecc',
    description='Fast elliptic curve digital signatures',
    long_description=''.join(open('README.rst', 'r').readlines()),
    url='https://github.com/AntonKueltz/fastecdsa',
    packages=['fastecdsa'],
    ext_modules=[curvemath, _ecdsa],
    cmdclass={'test': TestCommand, 'benchmark': BenchmarkCommand},
    classifiers=[
        'Development Status :: 4 - Beta',
        'Intended Audience :: Developers',
        'Topic :: Security :: Cryptography',
        'License :: CC0 1.0 Universal (CC0 1.0) Public Domain Dedication',
        'Programming Language :: Python :: 2.7',
        'Programming Language :: Python :: 3.4',
        'Programming Language :: Python :: 3.5',
        'Programming Language :: Python :: 3.6',
        'Operating System :: MacOS :: MacOS X',
        'Operating System :: POSIX :: Linux',
    ],
)
