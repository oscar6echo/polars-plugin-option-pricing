#!/bin/bash
set -ex

# curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
# export PATH="$HOME/.cargo/bin:$PATH"

# Compile wheels
echo '-------- compile wheels --------'
for PYBIN in /opt/python/cp{311,312}*/bin; do
    echo "---- python=$PYBIN"
    rm -rf /io/build/
    "${PYBIN}/pip" install -U setuptools setuptools-rust wheel
    "${PYBIN}/pip" wheel /io/ -w /io/dist/ --no-deps
    # "${PYBIN}/python" -m build /io/ -w /io/dist/ --no-deps
done

# Bundle external shared libraries into the wheels
echo '-------- auditwheel wheels --------'
for whl in /io/dist/*{cp311,cp312}*linux*.whl; do
    echo "---- whl=$whl"
    auditwheel repair "$whl" -w /io/dist/
done
