#!/bin/bash

set -e

setup() {
    echo -e "\n========= Running $0 starting ========="
    cd ./tests/tests/data/
    make ITEST=1 md5 --no-print-directory  --silent
    cd - > /dev/null

    cargo build --quiet
}

teardown() {
    cd ./tests/tests/data/
    make clean --no-print-directory  --silent
    cd - > /dev/null
}


setup
output=$(./target/debug/rrv-iss -f tests/tests/data/md5/md5.elf)
echo "$output"
teardown

exit_code=$(echo "$output" | grep "Target application exit code:" | awk -F': ' '{print $2}')

if [ "$exit_code" -ne 0 ]; then
    echo "Target application exit code is not 0, it is $exit_code"
    echo -e "\033[31mTest FAILED\033[0m"
    echo -e "========= Running $0 Done =========\n"
    exit 1
fi

echo -e "\033[32mTest PASSED\033[0m"
echo -e "========= Running $0 Done =========\n"
