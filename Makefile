.PHONY: bootloader kernel iso

BOOTLOADER_TARGET := i686-unknown-lambix
KERNEL_TARGET := x86_64-unknown-lambix

BOOT_OUT_DIR := target/$(BOOTLOADER_TARGET)/release
OUT_DIR := target/$(KERNEL_TARGET)/release
TARGET_FLAGS :=

iso: $(OUT_DIR)/lambix.iso

$(OUT_DIR)/lambix.iso: $(OUT_DIR)/lambix packaging/grub/grub.cfg
	@rm -rf $(OUT_DIR)/sysroot/
	@mkdir -p $(OUT_DIR)/sysroot/boot/grub/
	@cp -r $< $(OUT_DIR)/sysroot/boot/lambix
	@cp -r $(word 2,$^) $(OUT_DIR)/sysroot/boot/grub/
	@grub2-mkrescue -o $@ $(OUT_DIR)/sysroot/ 2>/dev/null
	@printf "\nISO has been generated at %s\n" "$@"

$(BOOT_OUT_DIR)/bootloader: bootloader
	@touch "$@"

$(OUT_DIR)/lambix.header: $(OUT_DIR)/kernel
	@perl -we 'print pack "c", shift' 108  > $@
	@perl -we 'print pack "c", shift' 97  >> $@
	@perl -we 'print pack "c", shift' 109 >> $@
	@perl -we 'print pack "c", shift' 98  >> $@
	@perl -we 'print pack "N", shift' $$(stat --printf="%s" $(OUT_DIR)/kernel) >> $@

$(OUT_DIR)/lambix: $(OUT_DIR)/lambix.header $(OUT_DIR)/kernel $(BOOT_OUT_DIR)/bootloader
	@cat "$(OUT_DIR)/kernel" >>$<
	@objcopy --update-section=.kernel=$< --set-section-flags=.kernel=CONTENTS,ALLOC,LOAD,READONLY,DATA $(BOOT_OUT_DIR)/bootloader $@

$(OUT_DIR)/kernel: kernel
	@touch "$@"

bootloader:
	cargo build -p $@ --target ./$(BOOTLOADER_TARGET).json $(TARGET_FLAGS) --release

kernel:
	cargo build -p $@ --target ./$(KERNEL_TARGET).json $(TARGET_FLAGS) --release

lambemu:
	cargo build --package "tool_$@" --release
