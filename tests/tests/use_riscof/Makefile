.PHONY: help setup validateyaml testlist run

RISCV_ARCH_TEST_ROOT=/home/zsw/xtools/source_tools/tests_for_riscv/riscv-arch-test
VENV_PATH=/home/zsw/python-venv/python3.6.0/bin/activate

help:
	@echo "Available targets:"
	@echo "  setup        Initiate Setup for riscof."
	@echo "  validateyaml Validate the Input YAMLs using riscv-config."
	@echo "  testlist     Generate the test list for the given DUT and suite."
	@echo "  run          Run the tests on DUT and reference and compare signatures"
	@echo "  help         Print this help message"

# 
# should use the following command to activate the python virtual env
# source ~/python-venv/python3.6.0/bin/activate
# 
setup:
	riscof setup --dutname=rrv_iss

validateyaml:
	riscof validateyaml --config=config.ini

TESTLIST_RUN_PARAM=--config=config.ini --suite=$(RISCV_ARCH_TEST_ROOT)/riscv-test-suite/ --env=$(RISCV_ARCH_TEST_ROOT)/riscv-test-suite/env

testlist:
	riscof testlist $(TESTLIST_RUN_PARAM)

run:
	riscof run $(TESTLIST_RUN_PARAM)
