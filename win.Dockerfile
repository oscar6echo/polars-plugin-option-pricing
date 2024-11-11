FROM python:3.12-slim-bullseye

# ref https://learn.microsoft.com/en-us/vcpkg/users/platforms/mingw#mingw-cross
RUN apt-get update -y && \
    apt-get install -y curl wget vim build-essential && \
    apt-get install -y gcc-mingw-w64-x86-64 g++-mingw-w64-x86-64

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH=/root/.cargo/bin:$PATH

RUN rustup target add x86_64-pc-windows-gnu

RUN python -m pip install --no-cache-dir -U pip && \
    python -m pip install --no-cache-dir cffi && \
    python -m pip install --no-cache-dir maturin

# RUN wget -q -P /opt https://ziglang.org/download/0.13.0/zig-linux-x86_64-0.13.0.tar.xz && \
#     cd /opt && \
#     tar xf zig-linux-x86_64-0.13.0.tar.xz

# ENV  PATH=/opt/zig-linux-x86_64-0.13.0:$PATH

WORKDIR /io
COPY ./maturin-build-win.sh .
ENTRYPOINT [ "/io/maturin-build-win.sh" ]



# https://www.redhat.com/en/blog/container-permission-denied-errors
# docker run --security-opt label=disable --rm -v "$(pwd)":/io builder-win:local
# docker run --rm -v "$(pwd)":/io:Z builder-win:local
# docker run --rm -v "$(pwd)":/io builder-win:local
