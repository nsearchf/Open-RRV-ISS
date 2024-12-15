#!/bin/bash

set -e

# Specify the folder path
# RISCV is the path of the RISCV toolchain installation
folder_path="$RISCV/target/share/riscv-tests/isa"

echo -e "\n========= Running $0 starting ========="

cargo build

file_count=0

# Loop through all files in the specified folder
for file in "$folder_path"/rv32ui-p-*; do
    # Extract the filename without the path
    filename=$(basename -- "$file")
    
    # Check if the filename does not end with .dump
    if [[ $filename != *.dump ]]; then
        # If the filename does not end with .dump, process the file
        echo -e "\nRun $filename in rrv-iss."
        if [[ $filename == rv32ui-p-fence_i ]]; then
            continue
        fi
        if [[ $filename == rv32ui-p-ma_data ]]; then
            continue
        fi

        file_count=$((file_count + 1))

        # ./target/debug/rrv-iss -l debug -f "$file" -i tmp.instr
        # echo -e "\n========= Running $filename ========="
        output=$(./target/debug/rrv-iss -f "$file")
        exit_code=$(echo "$output" | grep "Target application exit code:" | awk -F': ' '{print $2}')

        # echo "$output"
        if [ "$exit_code" -ne 0 ]; then
            echo -e "\nTarget application exit code is not 0, it is $exit_code"
            echo "Total number of processed files: $file_count"
            echo -e "\033[31mTest FAILED\033[0m"
            echo -e "========= Running $0 Done =========\n"
            exit 1
        fi
    fi
done

echo -e "\nTotal number of processed files: $file_count"
echo -e "\033[32mTest PASSED\033[0m"
echo -e "========= Running  $0 Done =========\n"
