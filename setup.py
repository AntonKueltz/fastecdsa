from os import environ
from setuptools import setup, Extension  # type: ignore[import-untyped]


def pkgconfig(package, kw):
    result = subprocess.run(
        ["pkg-config", "--cflags", "--libs", package],
        capture_output=True,
        text=True,
    )
    if result.returncode == 0:
        mapping = {
            '-I': 'include_dirs',
            '-L': 'library_dirs',
            '-l': 'libraries',
        }
        for token in result.stdout.strip().split():
            option = token[:2]
            if option in mapping:
                kw.setdefault(mapping[option], []).append(token[2:])
    return kw


extra_compile_args = ["-std=c99"]
extra_link_args = []

if environ.get("COVERAGE_BUILD"):
    extra_compile_args.extend(["--coverage", "-g", "-O0"])
    extra_link_args.extend(["--coverage"])

extra_kwargs = {'include_dirs': ["src/"]}
extra_kwargs = pkgconfig('gmp', extra_kwargs)

curvemath = Extension(
    "fastecdsa.curvemath",
    sources=["src/curveMath.c", "src/curve.c", "src/point.c"],
    extra_compile_args=extra_compile_args,
    extra_link_args=extra_link_args,
    **extra_kwargs,
)

_ecdsa = Extension(
    "fastecdsa._ecdsa",
    sources=["src/_ecdsa.c", "src/curveMath.c", "src/curve.c", "src/point.c"],
    extra_compile_args=extra_compile_args,
    extra_link_args=extra_link_args,
    **extra_kwargs,
)

setup(
    ext_modules=[curvemath, _ecdsa],
)
