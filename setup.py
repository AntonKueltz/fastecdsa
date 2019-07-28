from setuptools import find_packages, setup, Extension, Command


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
    author='Anton Kueltz',
    author_email='kueltz.anton@gmail.com',
    classifiers=[
        'Development Status :: 4 - Beta',
        'Intended Audience :: Developers',
        'Topic :: Security :: Cryptography',
        'License :: CC0 1.0 Universal (CC0 1.0) Public Domain Dedication',
        'Programming Language :: Python :: 2.7',
        'Programming Language :: Python :: 3.4',
        'Programming Language :: Python :: 3.5',
        'Programming Language :: Python :: 3.6',
        'Programming Language :: Python :: 3.7',
        'Programming Language :: Python :: 3.8',
        'Operating System :: MacOS :: MacOS X',
        'Operating System :: POSIX :: Linux',
    ],
    cmdclass={'benchmark': BenchmarkCommand},
    description='Fast elliptic curve digital signatures',
    ext_modules=[curvemath, _ecdsa],
    install_requires=['six'],
    keywords='elliptic curve cryptography ecdsa ecc',
    license='CC0 1.0 Universal (CC0 1.0) Public Domain Dedication',
    long_description=''.join(open('README.rst', 'r').readlines()),
    name='fastecdsa',
    packages=find_packages(),
    tests_require=['pytest', 'pytest-cov'],
    url='https://github.com/AntonKueltz/fastecdsa',
    version='1.7.4',
)
