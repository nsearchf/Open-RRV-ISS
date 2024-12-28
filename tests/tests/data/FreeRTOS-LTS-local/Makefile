.PHONY: clean build_linux_native run_linux_native

build_linux_native:
	mkdir -p build
	cmake -S ./examples/cmake_linux_native -B build
	cmake --build build/

run_linux_native: build_linux_native
	./build/linux_native_example

build_for_riscv:
	mkdir -p build
	# cmake -S ./examples/rrv_iss_riscv -B build -DCMAKE_VERBOSE_MAKEFILE=ON \
	cmake -S ./examples/rrv_iss_riscv -B build \
		-DFREERTOS_PORT=GCC_RISC_V \
		-DCMAKE_TOOLCHAIN_FILE=../cmake/toolchains/riscv-gcc.cmake \
		-DAFR_TOOLCHAIN_PATH='/home/zsw/xtools/riscv/'
	cmake --build build/
	# riscv64-unknown-elf-objdump -DSlt build/rv32_example > build/rv32_example.asm

clean:
	rm -rf build
