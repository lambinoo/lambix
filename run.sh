#!/bin/bash

exec qemu-system-x86_64 \
	-cdrom target/x86_64-unknown-lambix/release/lambix.iso \
	--enable-kvm \
	-no-reboot \
	-no-shutdown \
	-m 4G \
	-smp 4 \
	-nographic
