# Polar plugin explo

## Overview

TBD

## Build

Commands:

```sh
# prerequisite
mm activate work
pip install build

# dev install
pip install -v -e .

# watch 
cargo watch --watch ./src -- pip install -e .

# test
pytest

# install
pip install .

# ------- build native wheel
python -m build .
```

## Ref

Packaging offical recommendation: [Is setup.py deprecated?](https://packaging.python.org/en/latest/discussions/setup-py-deprecated/).
