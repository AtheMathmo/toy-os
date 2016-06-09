OUTPUT_FORMAT("binary")
ENTRY(_start)

SECTIONS
{
	. = 0x1000;
	.text : { *(.text) }
	. = 0x8000000;
	.data : { *(.data) }
	.bss : { *(.bss) }
}