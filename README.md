# Polar plugin option-pricing

## Overview

[Polars plugin](https://docs.pola.rs/user-guide/expressions/plugins/) exposing rust crate [option-pricing](https://crates.io/crates/option-pricing).

## Install

Commands:

```sh
# ------- install from repo
# clone
git clone https://github.com/oscar6echo/polars-plugin-option-pricing.git
cd polars-plugin-option-pricing

# regular install
pip install build

# dev install
# -v (verbose) displays rust logs
pip install -v -e .

# watch 
cargo watch --watch ./src -- pip install -v -e .

# ------- install pypi/artifactory
pip install polars_plugin_option_pricing
```

## Build

Commands:

```sh
# ------- build native wheel
unset CARGO
unset CARGO_BUILD_TARGET
unset PYO3_CROSS_LIB_DIR
unset PYO3_CROSS_PYTHON_VERSION
unset DIST_EXTRA_CONFIG

python -m build .

# ------- build manylinux wheel
# image used by build
docker build -t builder-manylinux:local -f ./Dockerfile.manylinux .

# build wheels for several python versions
docker run --rm -v $(pwd):/io builder-manylinux:local /bin/bash /io/build-manylinux-wheels.sh


# ------- build windows wheel - using cross
# prerequisite
cargo install cross

export CARGO=cross
export CARGO_BUILD_TARGET=x86_64-pc-windows-gnu
export DIST_EXTRA_CONFIG=/tmp/build-opts.cfg

# set wheel suffix
echo -e "[bdist_wheel]\nplat_name=win-amd64" > $DIST_EXTRA_CONFIG

# image used by cross
docker build -t cross-pyo3:x86_64-pc-windows-gnu -f ./Dockerfile.win .

# build windows wheel
python -m build .
```

This produces wheels for linux and windows:

```sh
ls -1  dist
polars_plugin_option_pricing-0.1.0-cp312-cp312-linux_x86_64.whl
polars_plugin_option_pricing-0.1.0-cp312-cp312-linux_x86_64.whl
polars_plugin_option_pricing-0.1.0-cp312-cp312-manylinux_2_17_x86_64.manylinux2014_x86_64.whl
polars_plugin_option_pricing-0.1.0-cp312-cp312-win_amd64.whl
polars_plugin_option_pricing-0.1.0-cp311-cp311-linux_x86_64.whl
polars_plugin_option_pricing-0.1.0-cp311-cp311-manylinux_2_17_x86_64.manylinux2014_x86_64.whl
polars_plugin_option_pricing-0.1.0.tar.gz
```

## Publish

Commands:

```sh
# prerequisite
pip install -U twine

twine check dist/*

# assuming .pypirc configured
# for linux only manylinux: the others will be refused
twine upload dist/*.tar.gz
twine upload dist/*manylinux*
twine upload dist/*win_amd64*
```

## Ref

+ [Polars plugins tutorial](https://marcogorelli.github.io/polars-plugins-tutorial/) by Marco Gorelli -> Very useful !
+ Github issue [Suggestion: Plugin full example with input of n cols and output of m cols](https://github.com/MarcoGorelli/polars-plugins-tutorial/issues/58)
