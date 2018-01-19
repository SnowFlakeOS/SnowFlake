arch ?= x86_64
LD = $(arch)-elf-ld
NASM = nasm
BUILD_DIR = ../../../../build/arch/x86_64/boot2snow

CRYSTAL = crystal
CRYSTALFLAGS = --emit=obj --cross-compile --target=i386 --prelude=empty --link-flags "-nostdlib -m32 -fno-pie"

target ?= i386-boot2snow

linker_script := linker.ld

stage1_source := stage1/stage1.asm
stage1_object := $(BUILD_DIR)/stage1.bin

stage2_asm_src := $(shell find stage2 -name "*.asm")
stage2_asm_obj := $(patsubst stage2/%.asm, \
    $(BUILD_DIR)/stage2/asm/%.o, $(stage2_asm_src))

stage2_crystal_src := stage2/s2main.cr
stage2_crystal_obj := $(patsubst stage2/%.cr, \
    $(BUILD_DIR)/stage2/crystal/%.o, $(stage2_crystal_src))

stage2_object := $(BUILD_DIR)/stage2.bin

rust_os := target/$(target)/debug/libBoot2Snow.a

.PHONY: all

all: $(stage1_object) $(stage2_object)

# compile stage1
$(stage1_object): $(stage1_source)
	@mkdir -p $(shell dirname $@)
	@$(NASM) -f bin $< -o $@

# compile stage2
$(stage2_object): $(stage2_asm_obj) $(stage2_crystal_obj)
	@$(LD) --ignore-unresolved-symbol _GLOBAL_OFFSET_TABLE_ -T $(linker_script) -o $@ $(stage2_asm_obj) $(stage2_crystal_obj).o

# compile stage2 asm files
$(BUILD_DIR)/stage2/asm/%.o: stage2/%.asm
	@mkdir -p $(shell dirname $@)
	@$(NASM) -f elf $< -o $@

# compile stage2 crystal files
$(BUILD_DIR)/stage2/crystal/%.o: stage2/%.cr
	@mkdir -p $(shell dirname $@)
	@$(CRYSTAL) build $(CRYSTALFLAGS) -o $@ $<