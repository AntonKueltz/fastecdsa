#!/usr/bin/env zsh
eval "$(pyenv init -)"

rm -rf dist build *.egg-info

for version in 3.7 3.8 3.9 3.10 3.11; do
    echo "Building wheel for python $version"
    pyenv shell $version
    pip install -U pip wheel build > /dev/null
    python -m build > /dev/null
done