#!/bin/bash

LAMBDA_ARCH="linux/arm64" # set this to either linux/arm64 for ARM functions, or linux/amd64 for x86 functions.

docker build . -t localhost/clubclouddemo --platform ${LAMBDA_ARCH}
mkdir -p lambda
docker run --platform ${LAMBDA_ARCH} --rm localhost/clubclouddemo > lambda/bootstrap
