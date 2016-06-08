arch ?= i686
target ?= $(arch)-unknown-linux-gnu

rust_kernel := ./kernel/target/$(target)/debug/libkernel.a

kernel := ./build/kernel-$(arch).bin
boot_sect := ./build/boot_sect.bin
kernel_entry := ./build/kernel_entry.o

os-image := ./build/os-image


all: os-image

run: all
	bochs

clean:
	@cargo clean --manifest-path ./kernel/Cargo.toml
	@rm -rf build

os-image: $(os-image)

$(os-image): $(boot_sect) $(kernel)
	cat $^ > ./build/os-image

boot_sect: $(boot_sect)

$(boot_sect): asm/boot_sect.asm
	@mkdir -p build
	nasm asm/boot_sect.asm -f bin -o $(boot_sect) -I ./asm/

kernel: $(kernel)

$(kernel): rust_kernel
	@mkdir -p build
	nasm asm/kernel_entry.asm -f elf -o $(kernel_entry)
	ld --gc-sections -m elf_i386 -o $(kernel) -Ttext 0x1000 $(kernel_entry) $(rust_kernel)

rust_kernel:
	cargo rustc --manifest-path ./kernel/Cargo.toml --target $(target)
