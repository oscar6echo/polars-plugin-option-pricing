
FROM quay.io/pypa/manylinux2014_x86_64:latest


RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH=/root/.cargo/bin:$PATH

ENV PATH=/opt/python/cp38-cp38/bin:/opt/python/cp39-cp39/bin:/opt/python/cp310-cp310/bin:/opt/python/cp311-cp311/bin:/opt/python/cp312-cp312/bin:$PATH

RUN curl --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    && yum install -y libffi-devel openssh-clients \
    && python3.8 -m pip install --no-cache-dir cffi \
    && python3.8 -m pip install --no-cache-dir maturin \
    && mkdir /io

