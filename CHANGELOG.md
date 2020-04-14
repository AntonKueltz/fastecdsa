# Changelog

## [2.1.2]
### Fixed
- Point at infinity handling in C extensions
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