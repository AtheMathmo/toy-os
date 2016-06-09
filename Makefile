arch ?= i686
target ?= $(arch)-unknown-linux-gnu

rust_kernel := ./kernel/target/$(target)/debug/libkernel.a

kernel := ./build/kernel-$(arch).bin
boot_sect := ./build/boot_sect.bin
kernel_entry := ./build/kernel_entry.o

os-image := ./build/os-image

.PHONY: all
all: os-image

.PHONY: run
run: all
	bochs

.PHONY: clean
clean:
	@cargo clean --manifest-path ./kernel/Cargo.toml
	@rm -rf build

.PHONY: os-image
os-image: $(os-image)

$(os-image): $(boot_sect) $(kernel)
	cat $^ > ./build/os-image

.PHONY: boot_sect
boot_sect: $(boot_sect)

$(boot_sect): asm/boot_sect.asm
	@mkdir -p build
	nasm asm/boot_sect.asm -f bin -o $(boot_sect) -I ./asm/

.PHONY: kernel
kernel: $(kernel)

$(kernel): $(rust_kernel)
	@mkdir -p build
	nasm asm/kernel_entry.asm -f elf -o $(kernel_entry)
	ld -m elf_i386 -o $(kernel) -T linker.ld $(kernel_entry) $(rust_kernel)

.PHONY: $(rust_kernel)
$(rust_kernel):
	cargo rustc --manifest-path ./kernel/Cargo.toml --target $(target) -- -C relocation-model=static -O
