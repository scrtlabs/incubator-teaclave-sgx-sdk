#!/usr/bin/env bash

set -e

DOCKERFILE_2204_NIGHTLY=Dockerfile.2204.nightly
DOCKERFILE_2004_NIGHTLY=Dockerfile.2004.nightly
# DOCKERFILE_centos8_NIGHTLY=Dockerfile.centos8.nightly

IMAGE_2204_NIGHTLY=scrtlabs/sgx-rust:2204-2.18
IMAGE_2004_NIGHTLY=scrtlabs/sgx-rust:2004-2.18
# IMAGE_centos8_NIGHTLY=baiduxlab/sgx-rust:centos8-1.1.5

build_one() {
	docker build --load --no-cache -t $1 -f $2 .
}

build_one ${IMAGE_2004_NIGHTLY} ${DOCKERFILE_2004_NIGHTLY}
build_one ${IMAGE_2204_NIGHTLY} ${DOCKERFILE_2204_NIGHTLY}
# build_one ${IMAGE_centos8_NIGHTLY} ${DOCKERFILE_centos8_NIGHTLY}
