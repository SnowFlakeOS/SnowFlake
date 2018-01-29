NASM = nasm
BUILD_DIR = ../../../build/kernel

init = $(BUILD_DIR)/start.o
init_src = start.asm

.PHONY: all

all: $(init)

$(init): $(init_src)
	@$(NASM) -f elf64 $< -o $@ -MD $@.d -MP