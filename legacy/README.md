# Cross Compile

This section is deprecated.  
Cf [README](../README.md) for convenient instructions without docker.  

## Prerequisite

Copy files in [legacy](./legacy) to repo root.

Commands:

```sh
# from repo root
cp -r legacy/* .
```

## Compile

Commands:

```sh
# ------- manylinux wheel
# image used by build
docker build -t builder-manylinux:local -f ./manylinux.Dockerfile .

# build wheels
docker run --rm -v "$(pwd)":/io builder-manylinux:local /bin/bash /io/maturin-build.sh
```
