from setuptools import setup, Extension, Command
from sys import executable

class BenchmarkCommand(Command):
    user_options = []
    description = "Benchmark this package"

    def initialize_options(self):
        pass

    def finalize_options(self):
        pass

    def run(self):
        from subprocess import call, PIPE
        call([executable, 'setup.py', 'build_ext', '--inplace'], stdout=PIPE)
        call([executable, '-m', 'fastecdsa.benchmark'])


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
    cmdclass={'benchmark': BenchmarkCommand},
    ext_modules=[curvemath, _ecdsa],
)
