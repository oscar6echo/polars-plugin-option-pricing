#! /bin/bash

maturin build -m /io/Cargo.toml --release --manylinux 2014 --out /io/dist
