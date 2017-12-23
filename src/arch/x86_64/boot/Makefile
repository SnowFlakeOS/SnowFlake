NASM = nasm
BUILD_DIR = ../../../../build/arch/x86_64/boot

bootsector_source := stage1/bootsector.asm
bootsector_object := $(BUILD_DIR)/boot.bin

loader_source := stage2/loader.asm
loader_object := $(BUILD_DIR)/loader.bin

.PHONY: all

all: $(bootsector_object) $(loader_object)

# compile stage1
$(bootsector_object): $(bootsector_source)
	@mkdir -p $(shell dirname $@)
	@$(NASM) -f bin $< -o $@

# compile stage2
$(loader_object): $(loader_source)
	@mkdir -p $(shell dirname $@)
	@$(NASM) -f bin $< -o $@