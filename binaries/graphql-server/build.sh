#!/bin/bash

LAMBDA_ARCH="linux/arm64" # set this to either linux/arm64 for ARM functions, or linux/amd64 for x86 functions.

podman build . -t localhost/clubclouddemo --platform ${LAMBDA_ARCH}
mkdir -p lambda/bootstrap
podman run --platform ${LAMBDA_ARCH} --rm localhost/clubclouddemo > lambda/bootstrap/graphql-server
