#! /bin/bash

maturin build -m /io/Cargo.toml --release --target x86_64-pc-windows-gnu --out /io/dist/docker
