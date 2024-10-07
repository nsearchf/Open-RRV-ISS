# make TOOLCHAIN_PREFIX=/home/zsw/xtools/riscv_i_zicsr/bin/riscv64-unknown-elf-

ITEST ?= 0

TOOLCHAIN_PREFIX ?= riscv64-unknown-elf-

# -march=rv32izicsr  -nostdlib -Ttext 0x10000
CC       := $(TOOLCHAIN_PREFIX)gcc
CFLAGS_COMMON := -std=c99 -g -O0 -Wall -Wextra -Werror -Wconversion -Wfloat-equal -Wformat -Wpacked -Wundef
CFLAGS_FOR_RV := -march=rv32i_zicsr -mabi=ilp32 -static
CFLAGS   ?= 
OBJCOPY  := $(TOOLCHAIN_PREFIX)objcopy
OBJDUMP  := $(TOOLCHAIN_PREFIX)objdump
READELF  := $(TOOLCHAIN_PREFIX)readelf

# Define a function to perform compilation, binary conversion, and disassembly operations.
ifeq ($(ITEST),1)

define compile_asm_template
	$(CC) $(CFLAGS_FOR_RV) -nostdlib -Ttext 0x10000 $(1).s -o $(1).elf || exit 1
	$(OBJCOPY) -O binary $(1).elf $(1).bin || exit 1
	rm -f $(1).elf 
endef

else

define compile_asm_template
	$(CC) $(CFLAGS_FOR_RV) -nostdlib -Ttext 0x10000 $(1).s -o $(1).elf || exit 1
	$(OBJCOPY) -O binary $(1).elf $(1).bin || exit 1

	$(OBJDUMP) -DSlt $(1).elf > $(1).asm || exit 1
	rm -f $(1).elf 
	rm -f $(1).asm
endef

endif

# Use a wildcard to compile all .s files in the current directory.
SOURCES := $(wildcard *.s)
TARGETS := $(patsubst %.s, %, $(SOURCES))

.PHONY: all clean hello hok empty_main md5 exit_code

# Define the 'all' target which depends on all the generated targets.
all: $(TARGETS) empty_main md5 exit_code

# Use automatic variables $@ for the target file and $^ for all dependencies.
# The $@ variable represents the target file name, and $^ represents all dependency files.
# This rule will be applied automatically to all .s files by make.
%: %.s
	$(eval NAME := $(patsubst %.s, %, $@))
	$(call compile_asm_template,$(NAME))

# Define the 'clean' target to remove all generated files.
clean:
	rm -f *.elf *.bin *.o *.asm *.log
	cd exit_code/; rm -f *.elf *.bin *.o *.asm *.log
	cd md5/; rm -f *.elf *.bin *.o *.asm *.log

empty_main:
	$(CC) $(CFLAGS_COMMON) $(CFLAGS_FOR_RV) empty_main.c -o empty_main.elf
ifneq ($(ITEST),1)
	$(READELF) -Sh empty_main.elf
	$(OBJDUMP) -DSlt empty_main.elf > empty_main.asm
endif

md5:
	$(CC)  $(CFLAGS_COMMON) $(CFLAGS_FOR_RV) \
		-nostartfiles -T common/linker_script.ld common/start.s \
		common/uart_printf.c md5/md5.c md5/main.c -o md5/md5.elf
ifneq ($(ITEST),1)		
	$(OBJDUMP) -DSlt md5/md5.elf > md5/md5.asm
	$(OBJDUMP) -D md5/md5.elf > md5/md5_bare.asm
	
	gcc $(CFLAGS_COMMON) -DX86_PLATFORM \
		common/uart_printf.c md5/md5.c md5/main.c -o md5/md5_x86.elf
endif

exit_code:
	$(CC) $(CFLAGS_COMMON) $(CFLAGS_FOR_RV) \
		-nostartfiles -T common/linker_script.ld common/start.s \
		exit_code/exit_code_0.c -o exit_code/exit_code_0.elf
ifneq ($(ITEST),1)		
	$(OBJDUMP) -DSlt exit_code/exit_code_0.elf > exit_code/exit_code_0.asm
endif

	$(CC) $(CFLAGS_COMMON) $(CFLAGS_FOR_RV) \
		-nostartfiles -T common/linker_script.ld common/start.s \
		exit_code/exit_code_1.c -o exit_code/exit_code_1.elf

hello:
	$(CC) $(CFLAGS_COMMON) $(CFLAGS_FOR_RV) hello.c -o hello.elf || exit 1	
	$(OBJDUMP) -DSlt hello.elf > hello.asm || exit 1
	spike -l --log=./run.log --isa=rv32gc /home/zsw/xtools/riscv/riscv32-unknown-elf/bin/pk hello.elf
