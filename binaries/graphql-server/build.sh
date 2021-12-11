#!/bin/bash

LAMBDA_ARCH="linux/arm64" # set this to either linux/arm64 for ARM functions, or linux/amd64 for x86 functions.
RUST_TARGET="aarch64-unknown-linux-gnu" # corresponding with the above, set this to aarch64 or x86_64 -unknown-linux-gnu for ARM or x86 functions.
RUST_VERSION="latest" # Set this to a specific version of rust you want to compile for, or to latest if you want the latest stable version.
docker run \
  --platform ${LAMBDA_ARCH} \
  --rm --user "$(id -u)":"$(id -g)" \
  -v "${PWD}":/usr/src/myapp -w /usr/src/myapp rust:${RUST_VERSION} \
  cargo build --release --target ${RUST_TARGET} # This line can be any cargo command

(cd target/aarch64-unknown-linux-gnu/release && mkdir -p lambda && cp graphql-server lambda/bootstrap)
