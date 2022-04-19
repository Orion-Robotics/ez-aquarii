#!/usr/bin/env bash

PI_HOST=$1
PI_USER=${2:-pi}
BUILD_MODE=${3:-debug}

./scripts/camera.sh $PI_HOST &
./scripts/controller.sh $PI_HOST $BUILD_MODE $PI_USER &
trap 'kill -9 -$$; exit 1' INT
wait
