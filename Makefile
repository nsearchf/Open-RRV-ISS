.PHONY: all-tests help run-cargo-test use-riscv-tests use-md5-test use-riscv-arch-tests run-freertos-tests

all-tests: run-cargo-test use-riscv-tests use-md5-test use-riscv-arch-tests run-freertos-tests

use-md5-test:
	@./scripts/run_md5_test.sh

use-riscv-tests:
	@./scripts/run_riscv_tests.sh

run-cargo-test:
	@./scripts/run_cargo_test.sh

use-riscv-arch-tests:
	@./scripts/run_riscv_arch_tests.sh

run-freertos-tests:
	@./scripts/run_freertos_tests.sh

help:
	@echo "Usage: make [target]"
	@echo "Targets:"
	@echo "  help: Show this help message"
	@echo "  run-cargo-test: Run cargo test"
	@echo "  use-riscv-tests: Run riscv tests"
	@echo "  use-md5-test: Run md5 test"
	@echo "  use-riscv-arch-tests: Run riscv arch tests"
	@echo "  all-tests: Run all tests"
	@echo "  run-freertos-tests: Run FreeRTOS tests"
	@echo "  default target is 'all-tests'"
	@echo "Note: This Makefile is only used for development purposes."