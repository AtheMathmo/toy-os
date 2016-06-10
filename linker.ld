ENTRY(_start)

SECTIONS
{
	. = 1M;

	.boot :
	{
		KEEP(*(.multiboot_header))
	}

	.text :
	{
		*(.text .text.*)
		. = ALIGN(4K);
	}
}