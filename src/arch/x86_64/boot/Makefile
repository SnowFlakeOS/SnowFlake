CXX = x86_64-elf-g++
CC = x86_64-elf-gcc
LD = x86_64-elf-ld
NASM = nasm
BUILD_DIR = ../../../../build/arch/x86_64/boot

CFLAGS = -m32 -masm=intel -ffreestanding -O2 -Wall -Wextra -fno-exceptions -mno-sse -Iinclude/libc -Iinclude
CXXFLAGS = $(CFLAGS) -std=c++1z -fno-rtti

linker_script := linker.ld

stage1_source := stage1/stage1.asm
stage1_object := $(BUILD_DIR)/stage1.bin

libc_src := $(shell find libc -name "*.c")
libc_obj := $(patsubst libc/%.c, \
    $(BUILD_DIR)/libc/%.o, $(libc_src))

stage2_cxx_src := $(shell find stage2 -name "*.cpp")
stage2_cxx_obj := $(patsubst stage2/%.cpp, \
    $(BUILD_DIR)/stage2/cxx/%.o, $(stage2_cxx_src))

stage2_asm_src := $(shell find stage2 -name "*.asm")
stage2_asm_obj := $(patsubst stage2/%.asm, \
    $(BUILD_DIR)/stage2/asm/%.o, $(stage2_asm_src))

stage2_object := $(BUILD_DIR)/stage2.bin

.PHONY: all

all: $(stage1_object) $(stage2_object)

# compile stage1
$(stage1_object): $(stage1_source)
	@mkdir -p $(shell dirname $@)
	@$(NASM) -f bin $< -o $@

# compile stage2
$(stage2_object): $(libc_obj) $(stage2_asm_obj) $(stage2_cxx_obj)
	@$(LD) -T $(linker_script) -o $@ $(libc_obj) $(stage2_asm_obj) $(stage2_cxx_obj)

# compile stage2 asm files
$(BUILD_DIR)/stage2/asm/%.o: stage2/%.asm
	@mkdir -p $(shell dirname $@)
	@$(NASM) -f elf $< -o $@

# compile stage2 cxx files
$(BUILD_DIR)/stage2/cxx/%.o: stage2/%.cpp
	@mkdir -p $(shell dirname $@)
	@$(CXX) $(CXXFLAGS) -c $< -o $@

# compile libc files
$(BUILD_DIR)/libc/%.o: libc/%.c
	@mkdir -p $(shell dirname $@)
	@$(CC) $(CFLAGS) -c $< -o $@
