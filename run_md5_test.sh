set -e

cd ./tests/tests/data/
make ITEST=1 md5
cd -

cargo build -r

echo -e "\n========= Running rrv-iss [Begin] ========="
./target/release/rrv-iss -f tests/tests/data/md5/md5.elf
echo -e "========= Running rrv-iss [End] =========\n"

cd ./tests/tests/data/
make clean
cd -
