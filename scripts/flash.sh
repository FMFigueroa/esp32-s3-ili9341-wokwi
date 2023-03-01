#!/usr/bin/env bash

set -e

BUILD_MODE=""
case "$1" in
"" | "release")
    bash scripts/build.sh
    BUILD_MODE="release"
    ;;
"debug")
    bash scripts/build.sh debug
    BUILD_MODE="debug"
    ;;
*)
    echo "Wrong argument. Only \"debug\"/\"release\" arguments are supported"
    exit 1
    ;;
esac

export ESP_ARCH=xtensa-esp32s3-none-elf

web-flash --chip esp32s3 target/${ESP_ARCH}/${BUILD_MODE}/esp32_s3_with_ili9341
