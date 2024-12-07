#!/bin/bash

set -e

setup() {
    echo -e "\n========= Running $0 starting ========="
    cd ./tests/tests/data/
    make ITEST=1 --no-print-directory  --silent
    cd - > /dev/null
}

teardown() {
    cd ./tests/tests/data/
    make clean --no-print-directory  --silent
    cd - > /dev/null
}

setup
cargo test
teardown

echo -e "========= Running $0 Done =========\n"
