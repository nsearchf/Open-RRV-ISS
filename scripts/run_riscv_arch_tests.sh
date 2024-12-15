#!/bin/bash

set -e

echo -e "\n========= Running $0 starting ========="

cargo build
cd ./tests/tests/use_riscof/
source ~/python-venv/python3.6.0/bin/activate
make run

echo -e "========= Running  $0 Done =========\n"
