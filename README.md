# Polar plugin Option Pricing

## Overview

[Polars plugin](https://docs.pola.rs/user-guide/expressions/plugins/) exposing rust crate [option-pricing](https://crates.io/crates/option-pricing).

## Install

Commands:

```sh
# ------- install from pypi/artifactory
pip install polars_plugin_option_pricing
```

## Use

### Black-Scholes

Calculate Call & Put Option price and greeks with [BlackScholes formula](https://en.wikipedia.org/wiki/Black%E2%80%93Scholes_model):

+ [run-bs.py](./test/run-bs.py)
+ [run-bs.ipynbpy](./test/run-bs.ipynb)

In short:

```py
import polars_plugin_option_pricing as m

# black scholes
df = df.with_columns(
    output_bs=m.black_scholes(
        "is_call", 
        "spot", 
        "strike", 
        "mat", 
        "vol", 
        "rate", 
        "div"
    ),
).drop(["is_call"]).unnest("output_bs")
```

### Implied Vol

Calculate [implied volatility](https://en.wikipedia.org/wiki/Implied_volatility) for call options:

+ [run-iv.py](./test/run-iv.py)
+ [run-iv.ipynbpy](./test/run-iv.ipynb)

In short:

```py
import polars_plugin_option_pricing as m

# implied vol
df = df.with_columns(
    iv_output=m.implied_vol(
        "price",
        "spot",
        "strike",
        "mat",
        "rate",
        "div",
        iter=10,
        prec=1e-7,
        # method="Newton",
        method="Halley",
    )
).unnest("iv_output")
```

## Install dev mode

Commands:

```sh
# ------- install from repo
# clone
git clone https://github.com/oscar6echo/polars-plugin-option-pricing.git
cd polars-plugin-option-pricing

# fast compile, slow exec
maturin develop
# slow compile, fast exec
maturin develop --release


# alternative
pip install -v -e .

# watch 
cargo watch --watch ./src -- maturin develop
```

## Build

Commands:

```sh
# ------- build native wheel
maturin build --sdist --release --out dist

# ------- build manylinux wheel
# install zig
pip install maturin[zig]

maturin build --release --target x86_64-unknown-linux-gnu --zig --out dist


#######################################################
# NOTE
# build wheel win specific
# edit src/lib.rs and comment PolarsAllocator 
# ref https://github.com/PyO3/maturin/discussions/2297
#######################################################

# ------- build windows wheel - 1st method
# debian & co
sudo apt-get install mingw-w64

# check compilation
maturin build --profile dev --target x86_64-pc-windows-gnu --out dist

maturin build --release --target x86_64-pc-windows-gnu --out dist

# ------- build windows wheel - 2nd method
# docker
docker build -t builder-win:local -f ./win.Dockerfile .
docker run --rm -v "$(pwd)":/io builder-win:local

```

This produces wheels for linux and windows:

```sh
❯ ls dist
polars_plugin_option_pricing-0.1.0-cp38-abi3-manylinux_2_17_x86_64.manylinux2014_x86_64.whl
polars_plugin_option_pricing-0.1.0-cp38-abi3-manylinux_2_34_x86_64.whl
polars_plugin_option_pricing-0.1.0-cp38-abi3-win_amd64.whl
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
+ repo [oscar6echo/pyo3-option-pricing](https://github.com/oscar6echo/pyo3-option-pricing) using the same underlying crate, but vastly less efficient for batch pricing.
