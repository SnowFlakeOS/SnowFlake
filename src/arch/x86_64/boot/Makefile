NASM = nasm
BUILD_DIR = ../../../../build/arch/x86_64/boot

bootsector_source := stage1/bootsector.asm
bootsector_object := $(BUILD_DIR)/boot.igloo

.PHONY: all

all: $(bootsector_object)

# compile stage1
$(bootsector_object): $(bootsector_source)
	@mkdir -p $(shell dirname $@)
	@$(NASM) -f bin $< -o $@
