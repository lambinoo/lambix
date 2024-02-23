.PHONY: bootloader kernel iso

BOOTLOADER_TARGET := i686-unknown-lambix
KERNEL_TARGET := x86_64-unknown-lambix

BOOT_OUT_DIR := target/$(BOOTLOADER_TARGET)/release
OUT_DIR := target/$(KERNEL_TARGET)/release

TARGET_FLAGS := -Z build-std=core,compiler_builtins -Z build-std-features=compiler-builtins-mem

iso: $(OUT_DIR)/lambix.iso

$(OUT_DIR)/lambix.iso: $(OUT_DIR)/lambix packaging/grub/grub.cfg
	rm -rf $(OUT_DIR)/sysroot/
	mkdir -vp $(OUT_DIR)/sysroot/boot/grub/
	cp -rv $< $(OUT_DIR)/sysroot/boot/lambix
	cp -rv $(word 2,$^) $(OUT_DIR)/sysroot/boot/grub/
	grub2-mkrescue -o $@ $(OUT_DIR)/sysroot/ 2>/dev/null

$(BOOT_OUT_DIR)/bootloader: bootloader
	touch "$@"

$(OUT_DIR)/lambix: $(OUT_DIR)/kernel $(BOOT_OUT_DIR)/bootloader
	objcopy --update-section=.kernel=$< --set-section-flags=.kernel=CONTENTS,ALLOC,LOAD,READONLY,DATA $(BOOT_OUT_DIR)/bootloader $@

$(OUT_DIR)/kernel: kernel
	touch "$@"

bootloader:
	cargo build -p $@ --target ./$(BOOTLOADER_TARGET).json $(TARGET_FLAGS) --release

kernel:
	cargo build -p $@ --target ./$(KERNEL_TARGET).json $(TARGET_FLAGS) --release

lambemu:
	cargo build --package "tool_$@" --release
