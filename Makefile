clean::
	rm -rf build fastecdsa/*.so
coverage-build:
	COVERAGE_BUILD=1 uv run python setup.py build_ext --inplace
coverage-run: coverage-build
	uv run pytest
coverage-report:
	uv run gcovr -r . --print-summary
coverage: clean coverage-run coverage-report
