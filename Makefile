arch ?= x86_64
target ?= $(arch)-unknown-linux-gnu

rust_kernel_src := ./kernel/target/$(target)/debug/libkernel.a
rust_kernel := ./build/libkernel.a

grub_cfg := ./boot/grub.cfg
os-image := ./build/os-image.iso
kernel := ./build/kernel-$(arch).bin
linker_script := linker.ld

assembly_source_files := $(wildcard boot/asm/*.asm)
assembly_object_files := $(patsubst boot/asm/%.asm, \
	build/%.o, $(assembly_source_files))

.PHONY: all
all: os-image

.PHONY: run
run: all
	qemu-system-x86_64 -cdrom $(os-image) -m 2G

.PHONY: debug
debug: all
	qemu-system-x86_64 -d int -no-reboot -cdrom $(os-image)

.PHONY: clean
clean:
	@cargo clean --manifest-path ./kernel/Cargo.toml
	@rm -rf build

.PHONY: os-image
os-image: $(os-image)

$(os-image): $(kernel) $(grub_cfg)
	@mkdir -p build/isofiles/boot/grub
	@cp $(kernel) build/isofiles/boot/kernel.bin
	@cp $(grub_cfg) build/isofiles/boot/grub
	@grub-mkrescue -o $(os-image) build/isofiles 2> /dev/null
	@rm -r build/isofiles

.PHONY: kernel
kernel: $(kernel)

$(kernel): $(rust_kernel) $(assembly_object_files) $(linker_script)
	ld -n -T $(linker_script) -o $(kernel) $(assembly_object_files) $(rust_kernel)

.PHONY: $(rust_kernel)
$(rust_kernel):
	@mkdir -p ./build
	cargo rustc --manifest-path ./kernel/Cargo.toml --target $(target) -- -Z no-landing-pads
	@cp $(rust_kernel_src) $(rust_kernel)

build/%.o: boot/asm/%.asm
	@mkdir -p $(shell dirname $@)
	@nasm -felf64 $< -o $@
