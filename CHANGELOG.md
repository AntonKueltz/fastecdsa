# Changelog

## [3.1.0]
### Fixed
- Typos

### Added
- Support for python3.14
- Dependabot github action

### Changed
- Static methods in `SEC1Encoder` changed to instance methods
- Github action "uses" versions
- Setuptools version
- Replaced mypy with ty

## [3.0.1]
### Fixed
- Updated acceptable range of private keys from `[0, n)` to `[1, n)` as per SEC spec guidance (pull #97)

## [3.0.0]
### Added
- Support for python3.13
- `uv` for package management
- Typing for all non C extension modules in this package and a corresponding `py.typed` marker
- Github Actions for testing and publishing
- Pre-commit hooks to enforce formatting and type checks
- Functions in `fastedcsa.keys` specific to public and private keys:
  - `export_private_key`
  - `export_public_key`
  - `import_private_key`
  - `import_public_key`
- `__self__` to `fastecdsa.curve.Curve`

### Changed
- `fastecdsa.encodng.KeyEncoder` interface:
  - All methods changed from static methods to instance methods
  - Encoders always return `bytes`
  - Decoders always take `bytes` for key data
  - Private key methods do not consume or return public key data in their signatures
    - Private key encoding _does_ take a curve parameter as some encoders do encode the public key as part of the encoded private key (in this case the public key is computed using the private key and the curve)

### Removed
- Support for python3.7
- Support for python3.8
- `int` tuples are no longer allowed to be passed as `Q` in `fastecdsa.ecdsa.verify`. `Q` must be of type `point.Point` now.
- `fastecdsa.keys.export_key` and `fastecdsa.keys.import_key`. Superseded by functions specific to public and private keys.
- Default curve parameter from `fastecdsa.point.Point` constructor
- `__unicode__` from `fastecdsa.point.Point`

## [2.3.2]
### Added
- Support for python3.12

### Removed
- CI/CD pipelines

### Fixed
- [Memory corruption issue](https://gist.github.com/keltecc/49da037072276f21b005a8337c15db26). Special thanks to [keltecc](https://gist.github.com/keltecc) for discovering this and to Snyk for reporting it.

## [2.3.1]
### Added
- Building wheels via CI/CD for the following architectures
  - Linux x86_64
  - Linux i686
  - MacOS x86_64
  - MacOS arm64 (M1)

## [2.3.0]
### Added
- Support for python3.11
- `pyproject.toml` per [PEP-621](https://peps.python.org/pep-0621/)
- Support for [PEP-517](https://peps.python.org/pep-0517/) builds

### Removed
- Support for python3.6

## [2.2.3]
### Fixed
- Minor doc issues and doc build

## [2.2.2]
### Added
- Support for python3.10

### Fixed
- Error when adding two equal points where one point's coordinates are not reduced mod p (issue #75)

## [2.2.1]
### Fixed
- Incorrect DER encoding of data with length >=0x80 (issue #61)

## [2.2.0]
### Added
- Support for python3.9

### Removed
- Support for python3.5

### Fixed
- Multiplying by negative scalars (issue #70)
- Reading and writing keys to disk using encoders that output bytes (PR #62)

### Updated
- `fastecdsa.keys.import_key` now has a consistent return type (`Tuple[Optional[int], Point]`) for public and private
keys. Previously public key imports returned `Point`, they now return `Tuple[None, Point]`, in other words a tuple
with the first entry being `None` (the private key) and the second entry being the point representing the public key.

## [2.1.5]
### Fixed
- Timing leakage from nonce bit length aka the [Minerva](https://minerva.crocs.fi.muni.cz/) vulnerability (PR #60)

## [2.1.4]
### Fixed
- Reduction by base point order in scalar multiplication breaks when input point does not have that order (PR #58)
- Edge case for curves with order 2 was not handled (PR #58)
- Signature verification of pre-hashed messages broken (PR #56)

## [2.1.3]
### Fixed
- Multiplying base point by curve order caused an error (issue #54)

## [2.1.2]
### Fixed
- Point at infinity handling in C extensions (issue #52)
- DER signature decoding that assumed length was always encoded in one byte

## [2.1.1]
### Fixed
- RFC6979 nonce generation for signatures on pre-hashed messages (issue #46)

## [2.1.0]
### Added
- Curves W25519 and W448 from [NIST.SP.800-186](https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-186-draft.pdf)

### Fixed
- Curves with no OID are not added to the lookup by OID map
- Type validation for operations of points (issue #50)

## [2.0.0]
### Added
- This changelog
- Type hints for clearer function signatures

### Removed
- Support for python2.x
- Support for python3.4 and older
- Various unused imports

### Fixed
- Issue with benchmark script when C extensions weren't built in place (issue #44)
