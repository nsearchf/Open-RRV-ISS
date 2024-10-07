set -e

cd ./tests/tests/data/
make ITEST=1
cd -

cargo test

cd ./tests/tests/data/
make clean
cd -
